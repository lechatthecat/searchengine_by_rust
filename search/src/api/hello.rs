use serde::{Serialize, Deserialize};

use actix_web::{
    HttpResponse,
    Responder,
    HttpRequest,
};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    msg: String,
}

pub async fn hello(
    _req: HttpRequest,
) -> impl Responder {
    return HttpResponse::Ok().json(Message {
        msg: "hello".to_owned(),
    });
}
