use actix_web::web;

use crate::driving::rest_handler::health_check_handler;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("health")
            .service(web::resource("").route(web::get().to(health_check_handler::health_check))),
    );
}
