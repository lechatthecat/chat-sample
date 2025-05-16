use actix_web::{HttpResponse, Responder, HttpRequest, web, http::StatusCode};
use bcrypt::verify;
use serde::{Deserialize, Serialize};

use crate::{
    api::jwt::jwt,
    db::repository::user_repository::UserDataRepository,
    db::model::user::UserData,
    library::logger
};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    name: String,
    password: String,
}

// DIする場合はリポジトリもweb::Dataで渡す想定
pub async fn login(
    req: web::Json<LoginRequest>,
    repo: web::Data<UserDataRepository>
) -> impl Responder {
    // nameでユーザーとハッシュ取得
    let user_result = repo.find_with_password_by_name(&req.name).await;

    let (user_data, hashed_password) = match user_result {
        Ok(Some((user_data, hashed_password))) => (user_data, hashed_password),
        Ok(None) => return HttpResponse::Unauthorized().finish(),
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    // パスワード検証
    match verify(&req.password, &hashed_password) {
        Ok(true) => {
            // JWT生成
            match jwt::create_token(&user_data.name) {
                Ok(_token) => {
                    // 必要ならtokenをUserDataに追加
                    HttpResponse::Ok().json(user_data)
                }
                Err(err) => {
                    logger::log(logger::Header::ERROR, &err.to_string());
                    return HttpResponse::InternalServerError().finish();
                }
            }
        }
        _ => HttpResponse::Unauthorized().finish(),
    }
}

pub async fn current_user(req: HttpRequest) -> impl Responder {
    match jwt::verify(&req) {
        Ok(user_info) => HttpResponse::Ok().json(user_info),
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::Unauthorized().finish()
        },
    }
}
