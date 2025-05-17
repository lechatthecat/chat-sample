use actix_web::{web, HttpRequest, HttpResponse, Result, Scope};
use crate::api::controller::{
    auth_controller,
    user_controller,
    sse_controller,
};
use crate::api::middleware::jwt_middleware::JwtMiddleware;

async fn api_handler(req: HttpRequest) -> Result<HttpResponse> {
    let path = req.path();
    Ok(HttpResponse::NotFound().body(format!("This API: '{}' does not exist.", path)))
}

pub fn api_scope() -> Scope {
    web::scope("/api")
        .route("/auth/login", web::post().to(auth_controller::login))
        .route("/auth/current_user", web::get().to(auth_controller::current_user))
        // ↓ このスコープ（/api/user...）だけJWTミドルウェアをwrap
        .service(
            web::scope("")
                //.wrap(JwtMiddleware)
                .route("/users", web::get().to(user_controller::get_users)) // api/users
                .route("/users/{user_id}", web::get().to(user_controller::get_user)) // api/users/{user_id}
                .route("/sse/events", web::get().to(sse_controller::events)) // api/users
                .route("/sse/publish", web::post().to(sse_controller::publish)) // api/users/{user_id}
        )
        .default_service(web::route().to(api_handler))
}
