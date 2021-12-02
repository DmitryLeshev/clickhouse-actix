mod api;
mod config;
// mod errors;

use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use clickhouse_rs::Pool;
use dotenv::dotenv;
use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};
use tracing::{self as log};

#[derive(Debug)]
pub struct AppData {
    pub thread_id: u16,
    pub pool: Pool,
}

async fn index(req: HttpRequest) -> String {
    format!("Welcome! path: {}", req.path())
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.db.create_pool();
    config.log.init();

    let address = format!("{}:{}", config.server.host, config.server.port);
    log::debug!("The server is running on http://{}", address);

    let thread_counter = Arc::new(AtomicU16::new(1));
    HttpServer::new(move || {
        let thread_index = thread_counter.fetch_add(1, Ordering::SeqCst);
        log::trace!("Starting thread {}", thread_index);

        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(AppData {
                pool: pool.clone(),
                thread_id: thread_index,
            }))
            .wrap(cors)
            .route("/", web::get().to(greet))
            .configure(api::service)
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!(
            "🔥Couldn't start the server in port🔥 {}: {:?}",
            config.server.port, err
        )
    })
    .run()
    .await
}
