mod insert;
use crate::index;
use actix_web::web::{self, ServiceConfig};

use self::insert::insert;

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/clickhouse")
            .route("", web::get().to(index))
            .route("", web::post().to(insert)),
    );
}
