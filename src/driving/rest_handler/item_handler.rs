use actix_web::{HttpResponse, Responder, ResponseError, web};

use crate::{
    domain::{model::item::Item, port::primary::item_service_port::ItemServicePort},
    errors::AppError,
};

pub async fn create_item(
    service: web::Data<impl ItemServicePort>,
    payload: web::Json<Item>,
) -> impl Responder {
    match service.create_item(payload.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(domain_error) => {
            let app_error: AppError = domain_error.into();
            app_error.error_response()
        }
    }
}
