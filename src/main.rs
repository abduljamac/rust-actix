mod api;
mod model;
mod repository;

use api::task::create_task;
use repository::db::MongoRepo;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    let port = 3000;
    println!("Starting server at port {}", port);
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(db_data.clone())
            .service(create_task)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
