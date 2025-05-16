use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use api::middleware::jwt_middleware;
use dotenv::dotenv;

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

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            //.allowed_origin("http://localhost")
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec!["Authorization", "Content-Type"])
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
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .service(api::api_handler::api_scope())
    })
    .bind("0.0.0.0:8080")?
    .workers(20)
    .run()
    .await
}
