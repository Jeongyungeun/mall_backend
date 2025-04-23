use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status":"정상",
        "message":"서버가 정상적으로 동작 중입니다.",
        "timestamp":chrono::Utc::now().to_rfc3339(),
    }))
}
