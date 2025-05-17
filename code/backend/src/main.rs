use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use tokio::sync::broadcast;

mod api;
mod db;
mod library;

use library::logger;
use api::redis;

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
    let (tx, rx) = broadcast::channel::<String>(100);
    
    
    let nodes = vec![
        "redis://127.0.0.1:6379/",
    ];
    let actor = redis::RedisActor::new(nodes).await;
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
            .service(api::api_handler::api_scope())
    })
    .bind("0.0.0.0:8080")?
    .workers(20)
    .run()
    .await
}
