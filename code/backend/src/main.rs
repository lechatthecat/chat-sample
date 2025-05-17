use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use google_cloud_pubsub::client::{Client, ClientConfig};
use tokio_stream::StreamExt;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};

mod api;
mod db;
mod library;

use library::logger;

const PROJECT_PATH: &'static str = env!("CARGO_MANIFEST_DIR");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    logger::log(logger::Header::INFO, "API server started");
    
    // Load environment variables from .env file
    dotenv().ok();
    // Create the connection pool
    let pool = db::pool::get_db_pool().await;
    // Broadcasting channel for SSE 
    let (tx, _rx) = broadcast::channel::<String>(2000);
    // PubSubクライアント作成
    let topic_id = std::env::var("TOPIC_NAME").expect("TOPIC_NAME must be set");
    let subscribe_id = std::env::var("SUBSCRIBE_NAME").expect("SUBSCRIBE_NAME must be set");
    let config = ClientConfig::default().with_auth().await.expect("Failed to create PubSub client config");
    let pubsub_client = Client::new(config).await.expect("Failed to create PubSub client");
    pubsub_client.topic(&topic_id);
    let tx_clone = tx.clone();
    let topic_id = std::env::var("TOPIC_NAME").expect("TOPIC_NAME must be set");
    let topic = pubsub_client.topic(&topic_id);
    // Start publisher.
    let publisher = topic.new_publisher(None);
    let subscription = pubsub_client.clone().subscription(&subscribe_id);
    tokio::spawn(async move {
        // Pull型ストリーム
        let mut stream = subscription.subscribe(None).await.expect("failed to subscribe");
        while let Some(message) = stream.next().await {
            if let Ok(data) = String::from_utf8(message.message.data.clone()) {
                let _ = tx_clone.send(data);
            }
            logger::log(logger::Header::INFO, "stream loop");
            match message.ack().await {
                Ok(_) => {}
                Err(e) => {
                    logger::log(logger::Header::ERROR, &format!("Failed to ack message: {}", e));
                    // If ack fails, wait for a while before retrying
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });

    // Redis Cluster
    let redis_url = &std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let actor = api::redis::RedisActor::new(vec![redis_url]).await;
    let addr = actor.start();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            //.allowed_origin("http://localhost")
            .allowed_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Authorization", "Content-Type", "Accept", "Content-Type"])
            .supports_credentials()
            .max_age(60 * 60 * 24); // 1 day
                                    /*
                                    This sets the max_age to 1 day, meaning that
                                    once the browser makes a successful preflight request to the server,
                                    it can cache the results of that request for up to 3 days.
                                    Subsequent requests to the same resource within this time frame won't trigger another preflight request;
                                    the browser will use the cached results instead.
                                    */

        // Start the API server
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(tx.clone()))
            .app_data(Data::new(addr.clone()))
            .app_data(Data::new(publisher.clone()))
            .service(api::api_handler::api_scope())
    })
    .bind("0.0.0.0:8080")?
    .workers(20)
    .run()
    .await
}
