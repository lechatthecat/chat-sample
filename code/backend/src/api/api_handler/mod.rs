use actix_web::{web, HttpRequest, HttpResponse, Result, Scope};
use crate::api::controller::auth_controller;
use crate::api::controller::user_controller;
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
            web::scope("/users")
                .wrap(JwtMiddleware)
                .route("", web::get().to(user_controller::get_users)) // api/users
                .route("/{user_id}", web::get().to(user_controller::get_user)) // api/users/{user_id}
        )
        .default_service(web::route().to(api_handler))
}
