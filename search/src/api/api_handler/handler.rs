use actix_web::{
    web, HttpRequest, HttpResponse,
    Result, Scope
};

use super::super::hello;
use super::super::search;

async fn api_handler(req: HttpRequest) -> Result<HttpResponse> {
    // For 404
    let path = req.path();
    Ok(HttpResponse::NotFound().body(format!("This API: '{}' does not exist.", path)))
}

pub fn api_scope() -> Scope {
    // APIs except "login" and "current_user" are protected by JWT middleware
    web::scope("/api")
        .route("/hello", web::get().to(hello::hello))
        .route("/search", web::get().to(search::search))
        .default_service(web::route().to(api_handler)) // catch-all route for /api
}
