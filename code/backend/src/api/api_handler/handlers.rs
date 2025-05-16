use actix_web::{
    web, HttpRequest, HttpResponse,
    Result, Scope
};
use super::super::auth;
use super::super::menu;

async fn api_handler(req: HttpRequest) -> Result<HttpResponse> {
    // For 404
    let path = req.path();
    Ok(HttpResponse::NotFound().body(format!("This API: '{}' does not exist.", path)))
}

pub fn api_scope() -> Scope {
    // APIs except "login" and "current_user" are protected by JWT middleware
    web::scope("/api")
        .route("/auth/login", web::post().to(auth::login))
        .route("/auth/current_user", web::get().to(auth::current_user))
        .route("/menu", web::get().to(menu::get_menus))
        .route("/menu/{menu_id}", web::get().to(menu::get_menu))
        .default_service(web::route().to(api_handler)) // catch-all route for /api
}
