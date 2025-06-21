mod api;
mod library;

use actix_cors::Cors;
use actix_web::{dev::ServiceRequest, http::Method, middleware, web::{self, Data}, App, HttpServer};
use actix_limitation::{Limiter, RateLimiter};
use dotenv::dotenv;
use redis::{Client, AsyncConnectionConfig};
use std::{env, sync::OnceLock, time::Duration};

const PROJECT_PATH: &'static str = env!("CARGO_MANIFEST_DIR");
const LOG_PATH: OnceLock<String> = OnceLock::new();
static REDIS_CONNECTION_STRING: OnceLock<String> = OnceLock::new();
static ELASTICSEARCH_CONNECTION_STRING: OnceLock<String> = OnceLock::new();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    // Load environment variables from .env file
    dotenv().ok();
    // Create the configuration object
    //let pool = db::pool::get_db_pool().await;
    LOG_PATH.get_or_init(|| {
        format!(
            "{}/log/server_log.txt",
            PROJECT_PATH
        )
    });

    // 0.0.0.0 must be "redis" when this app is used in docker compose containers
    // Build the Redis connection string with proper lifetimes
    let redis_url = REDIS_CONNECTION_STRING.get_or_init(|| {
        format!(
            "redis://:{}@{}",
            env::var("REDIS_PASSWORD").expect("REDIS_PASSWORD must be set"),
            env::var("REDIS_URL").expect("REDIS_URL must be set")
        )
    });
    let _elasticsearch_url = ELASTICSEARCH_CONNECTION_STRING.get_or_init(|| {
        format!(
            "http://{}:{}@{}",
            env::var("ELASTIC_USERNAME").expect("ELASTIC_USERNAME must be set"),
            env::var("ELASTIC_PASSWORD").expect("ELASTIC_PASSWORD must be set"),
            env::var("ELASTICSEARCH_URL").expect("ELASTICSEATCH_URL must be set")
        )
    });

    // Initialize the Redis actor with the owned Redis URL string
    let client = Client::open(redis_url.as_str()).unwrap(); // should be in env
    let config = AsyncConnectionConfig::new()
        .set_connection_timeout(std::time::Duration::from_secs(5))
        .set_response_timeout(std::time::Duration::from_secs(5));
    let conn = client.get_multiplexed_async_connection_with_config(&config).await
        .expect("Failed to connect to Redis");

    HttpServer::new(move || {
        let limiter = web::Data::new(
            Limiter::builder(redis_url)
                .key_by(|req: &ServiceRequest| {
                    if req.method() != Method::OPTIONS {
                        // 1) If you're behind a trusted proxy and want the "real" client IP
                        //    that might be in X-Forwarded-For, use:
                        let ip_from_forward = req.connection_info()
                            .realip_remote_addr()
                            .map(|s| s.to_string());
                    
                        // 2) If you're not behind a proxy, or want the actual socket IP:
                        let ip_from_peer = req.peer_addr().map(|addr| addr.to_string());
                    
                        // Then choose which approach you prefer:
                        Some(ip_from_forward
                            .or(ip_from_peer)
                            .unwrap_or_else(|| "unknown-ip".to_string()))
                    } else {
                        None
                    }
                })
                .limit(4)
                .period(Duration::from_secs(1)) // 4 requests / 1s
                .build()
                .unwrap(),
        );
        
        let cors = Cors::default()
            .allowed_origin("http://localhost")
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "OPTION"])
            .allowed_headers(vec!["Authorization", "Content-Type"]);

        // Start the API server
        App::new()
            //.wrap(jwt_middleware::JwtMiddleware)
            .wrap(cors)
            .wrap(RateLimiter::default())
            .wrap(middleware::DefaultHeaders::new().add(("Cache-Control", "max-age=86400"))) // 1 day cache
            .app_data(limiter)
            //.app_data(Data::new(pool.clone()))
            .app_data(Data::new(conn.clone()))
            //.route("/", web::get().to(actix_redis::info))
            .service(api::api_handler::handler::api_scope())
    })
    .bind("0.0.0.0:8000")?
    //.workers(8)
    .run()
    .await
    
}
