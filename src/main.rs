use actix_web::{  App, HttpServer, middleware::Logger};
mod entity;
mod controller;
mod api;
mod mongo;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(api::v1) })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}