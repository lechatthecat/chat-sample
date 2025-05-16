use actix_web::{HttpResponse, Responder, HttpRequest, web};
use crate::{
    db::repository::user_repository::UserDataRepository,
    db::model::user::UserData,
    library::logger,
};

pub async fn get_users(
    _req: HttpRequest,
    pool: web::Data<sqlx::PgPool>
) -> impl Responder {
    let repo = UserDataRepository::new(pool.get_ref().clone()); // <- ここでリポジトリ作成
    match repo.list().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_user(
    _req: HttpRequest,
    user_id: web::Path<i32>,
    pool: web::Data<sqlx::PgPool>
) -> impl Responder {
    let repo = UserDataRepository::new(pool.get_ref().clone()); // <- ここでリポジトリ作成
    match repo.find(user_id.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}
