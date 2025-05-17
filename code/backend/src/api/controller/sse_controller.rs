use actix_web::{
    get,
    web,
    App,
    Error,
    HttpResponse,
    HttpServer
};
use futures_util::stream::Stream;
use futures_util::StreamExt;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use crate::{
    api::requests::publish_request::PublishRequest,
    library::logger,
};

pub async fn events(
    broadcaster: web::Data<broadcast::Sender<String>>,
) -> Result<HttpResponse, Error> {
    // クライアントごとに新しいReceiverを生成
    let rx = broadcaster.subscribe();

    // tokioのBroadcastStreamをfutures-utilのStreamに変換
    let stream = BroadcastStream::new(rx)
        .map(|msg| match msg {
            Ok(msg) => Ok::<_, std::convert::Infallible>(web::Bytes::from(format!("data: {}\n\n", msg))),
            Err(err) => {
                logger::log(logger::Header::ERROR, &err.to_string());
                Ok::<_, std::convert::Infallible>(web::Bytes::from("data: Error\n\n"))
            },
        });

    Ok(HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Content-Type", "text/event-stream"))
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .streaming(stream))
}

pub async fn publish(
    req: web::Json<PublishRequest>,
    broadcaster: web::Data<broadcast::Sender<String>>,
    pool: web::Data<sqlx::PgPool>
) -> HttpResponse {
    broadcaster.send(req.into_inner().msg).unwrap();
    HttpResponse::Ok()
    .insert_header(("Cache-Control", "no-cache"))
    .insert_header(("Content-Type", "text/event-stream"))
    .insert_header(("Access-Control-Allow-Origin", "*"))
    .body("Message broadcasted")
}
