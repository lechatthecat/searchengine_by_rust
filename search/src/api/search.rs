use std::time::Duration;

use redis::aio::MultiplexedConnection;
use regex::Regex;
use serde::{Serialize, Deserialize};
use reqwest::Client;
use serde_json;
use actix_web::{
    HttpResponse,
    Responder,
    web,
    HttpRequest,
};

use serde_json::{json, Value};

use crate::{api::utility::{self, cache}, library::logger};

use crate::ELASTICSEARCH_CONNECTION_STRING;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    msg: String,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    s: String,
    f: String,
    sa: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResult {
    highlight: Value,
    url: String,
    title: String,
    sort: String,
}

pub async fn search(
    req: HttpRequest,
    redis: web::Data<MultiplexedConnection>,
) -> impl Responder {
    let params = web::Query::<Params>::from_query(req.query_string())
        .unwrap_or(web::Query(Params { s: String::from(""), f: String::from(""), sa: None }));
    let re = Regex::new(r"\s+").unwrap();
    let data_search = re.replace_all(&params.s, " ").into_owned();

    if data_search == "" || data_search == " " {
        return HttpResponse::BadRequest().json(Message {
            msg: "Empty search parameter is not allowed.".to_string(),
        });
    }
    let size = params.s.parse::<i32>().unwrap_or(0);
    if size > 20 {
        return HttpResponse::BadRequest().json(Message {
            msg: "Search size must be 20 or less.".to_string(),
        });
    }
    let search_after = params.sa.clone();
    if let Some(ref search_after_value) = search_after {
        if !is_valid_search_after(search_after_value) {
            return HttpResponse::BadRequest().json(Message {
                msg: "Invalid search_after parameter.".to_string(),
            });
        }
    }

    let data_from = &params.f;
    let data_from_usize = data_from.parse::<usize>().unwrap_or(0);

    let cache_key = format!("{}-{}", data_search, params.f);

    let closure = || async {

        let client = Client::builder()
            .danger_accept_invalid_certs(true)  // Disables SSL verification
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
            .build().unwrap();

        let json_string = serde_json::to_string(&utility::build_search_query(
            &data_search, 
            data_from_usize,
            search_after.clone()
        )).unwrap();

        let response = client
            .post(format!("{}/web_pages/_search", ELASTICSEARCH_CONNECTION_STRING.get().expect("ELASTICSEARCH_URL is not set")))
            .header("Content-Type", "application/json")
            .body(json_string.clone())
            .send()
            .await.unwrap();

        // read the response body. Consumes search_response
        let response_body = response.json::<Value>().await.unwrap();
        // read fields from the response body
        let hits = response_body["hits"]["hits"]
                                        .as_array()
                                        .unwrap_or(&Vec::new())
                                        .iter()
                                        .map(|value| {
                                            let object = value.as_object().unwrap();
                                            let highlight = object.get("highlight").unwrap().clone();
                                            let source = object.get("_source").unwrap().as_object().unwrap();
                                            let url = source.get("url").unwrap().as_str().unwrap().to_string();
                                            let title = source.get("title").unwrap().as_str().unwrap().to_string();
                                            let sort = object.get("sort").unwrap().to_string();
                                            SearchResult { highlight, url, title, sort }
                                        })
                                        .collect::<Vec<SearchResult>>();

        let search_result = json!({
            "hits": &hits,
        }).to_string();
        cache::set_cache(&redis, &cache_key, &search_result, Some(1440)).await;
        search_result  
    };

    let fetched_data = cache::async_cache_as_json(&redis, &cache_key, closure, 86400).await;

    return HttpResponse::Ok().json(Message {
        msg: fetched_data
    });
}


pub fn is_valid_search_after(search_after: &str) -> bool {
    // --------------- Rule 3 ― must parse as JSON -----------------------------
    let v: Value = match serde_json::from_str(search_after) {
        Ok(v) => v,
        Err(_) => return false,
    };

    // --------------- Rule 1 & 2 ― flat array of length 3 ---------------------
    let arr = match v.as_array() {
        Some(a) if a.len() == 3 => a,
        _ => return false,
    };

    // --------------- Rule 2 & 4 ― every item is a finite f64 -----------------
    let mut nums = [0_f64; 3];
    for (i, item) in arr.iter().enumerate() {
        // not a number? string? nested array/object? → reject
        let n = match item.as_f64() {
            Some(n) if n.is_finite() => n,
            _ => return false,
        };
        nums[i] = n;
    }

    true
}
