use actix_web::{
    web,
    Error,
    HttpResponse,
};
use futures_util::StreamExt;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use crate::{
    api::requests::publish_request::PublishRequest,
    library::logger,
};
use google_cloud_pubsub::publisher::Publisher;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;

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
    publisher: web::Data<Publisher>,
    pool: web::Data<sqlx::PgPool>
) -> HttpResponse {
    // https://crates.io/crates/google-cloud-pubsub
    // https://crates.io/crates/google-cloud-googleapis
    let msg = PubsubMessage {
        data: req.into_inner().msg.into(),
        // Set ordering_key if needed (https://cloud.google.com/pubsub/docs/ordering)
        ordering_key: "order".into(),
        ..Default::default()
     };
    //broadcaster.send(req.into_inner().msg).unwrap();
    // Send a message. There are also `publish_bulk` and `publish_immediately` methods.
    let awaiter = publisher.publish(msg).await;
    // The get method blocks until a server-generated ID or an error is returned for the published message.
    match awaiter.get().await {
        Ok(_) => {
            HttpResponse::Ok()
            .insert_header(("Cache-Control", "no-cache"))
            .insert_header(("Content-Type", "text/event-stream"))
            .insert_header(("Access-Control-Allow-Origin", "*")) // TODO: ここは要修正
            .body("Message broadcasted")
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &format!("Failed to publish message: {}", err));
            HttpResponse::InternalServerError()
                .insert_header(("Cache-Control", "no-cache"))
                .insert_header(("Content-Type", "text/event-stream"))
                .insert_header(("Access-Control-Allow-Origin", "*")) // TODO: ここは要修正
                .body("Failed to publish message")
        },
    }
}
