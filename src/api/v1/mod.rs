mod clickhouse;
use crate::AppData;

use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use tracing::instrument;

#[instrument(skip(app_data))]
async fn api_methods(app_data: web::Data<AppData>) -> HttpResponse {
    tracing::info!("api_methods");
    HttpResponse::Ok()
        .append_header(("thread-id", app_data.thread_id.to_string()))
        .body("api_methods")
}

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("", web::get().to(api_methods))
            .configure(clickhouse::service),
    );
}
