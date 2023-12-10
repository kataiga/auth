mod config;
mod db;

use actix_web::{get, middleware::Logger, web::{self, Data}, App, HttpResponse, HttpServer, Responder};
use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection};
use crate::config::config::Config;
use db::create_pool;
use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: Data<Pool<ConnectionManager<PgConnection>>>,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();

    let config = Config::init();

    
    let db_client = create_pool();

    let app_state: AppState = AppState {
        env: config.clone(),
        db_client,
    };

    println!(
        "{}",
        format!("Server is running on http://localhost:{}", config.port)
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .service(health_checker_handler)
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await?;

    Ok(())
}

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Complete Restful API in Rust";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}