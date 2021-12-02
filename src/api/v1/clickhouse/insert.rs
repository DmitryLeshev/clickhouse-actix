use crate::AppData;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use clickhouse_rs::Block;
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertHits {
    os: String,
    browser: String,
    browser_name: String,
    timezone: String,
    cookies: String,
    prefer: String,
    session_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertResponse {
    status: String,
}

#[instrument(skip(app_data, req))]
pub async fn insert(
    req: HttpRequest,
    app_data: web::Data<AppData>,
    json: web::Json<InsertHits>,
) -> impl Responder {
    println!("Thread Id: {}", app_data.thread_id);
    let headers = req.headers();
    let host = headers.get("host");
    let (ip, port) = match host {
        Some(_host) => ("ip", "port"),
        None => ("ip", "port"),
    };
    let ddl = r"
    CREATE TABLE IF NOT EXISTS hits_v1 (
        session_id      String,
        ip              String,
        port            String,
        os              String,
        browser         String,
        browser_name    String,
        timezone        String,
        cookies         String,
        prefer          String
    ) Engine=Memory";
    let block = Block::new()
        .column("session_id", vec![json.session_id.clone()])
        .column("ip", vec![ip.clone()])
        .column("port", vec![port.clone()])
        .column("os", vec![json.os.clone()])
        .column("browser", vec![json.browser.clone()])
        .column("browser_name", vec![json.browser_name.clone()])
        .column("timezone", vec![json.timezone.clone()])
        .column("cookies", vec![json.cookies.clone()])
        .column("prefer", vec![json.prefer.clone()]);
    let mut client = app_data.pool.get_handle().await.unwrap();
    client.execute(ddl).await.unwrap();
    client.insert("hits_v1", block).await.unwrap();
    HttpResponse::Ok().json(InsertResponse {
        status: "Ok".to_string(),
    })
}
