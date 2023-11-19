use actix_web::{web, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Info {
    june: &'static str,
}

pub async fn get() -> impl Responder {
    web::Json(Info {
        june: env!("CARGO_PKG_VERSION"),
    })
}
