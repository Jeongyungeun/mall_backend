use std::env;

use actix_web::{App, HttpServer, middleware, web};
use mall::{
    config::{database, di::AppContainer},
    routes,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env.dev").ok();
    env_logger::init();

    let pool = database::create_db_pool()
        .await
        .expect("Failed to connect to Postgres");

    let container = AppContainer::new(pool.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(container.item_service.clone()))
            .configure(routes::config_app)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
