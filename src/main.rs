use actix_web::{middleware::Logger, web, App, HttpServer};
mod api;
use api::task::{delete, get, get_all, health, index, set};
use rusqlite::Connection;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = Connection::open("redis.db");
    let _ = conn.expect("error").execute(
        "CREATE TABLE redis (
            id   INTEGER PRIMARY KEY,
            key TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL
        )",
        (), // empty list of parameters.
    );
    HttpServer::new(|| {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(index)
            .service(health)
            .route("/set", web::post().to(set))
            .service(get_all)
            .service(get)
            .service(delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
