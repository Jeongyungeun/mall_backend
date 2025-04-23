use std::env;

use actix_web::{App, HttpServer, middleware, web};
use mall::{
    application::item_service::ItemService, config::di::AppContainer,
    driven::item_repository_impl::ItemRepositoryImpl, routes,
};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env.dev").ok();
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE CONNECTION ERROR");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed To connect to Postgres");

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
