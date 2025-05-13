use actix_web::{Responder, web};

use crate::domain::{model::user::User, port::primary::user_service_port::UserServicePort};

pub async fn create_user(
    service: web::Data<impl UserServicePort>,
    payload: web::Json<User>,
) -> impl Responder {
}
