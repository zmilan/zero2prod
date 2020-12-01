use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the pool in an Arc smart pointer
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            // A new entry in our routing table for POST /subscriptions requests
            .route("/subscriptions", web::post().to(subscribe))
            // Our pool is already wrapped in an Arc pointer:
            // using .data would add another Arc pointer on top
            // of the existing one - an unnecessary indirection.
            // .app_data instead does not perform an additional layer of wrapping.
            .data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    // No .await here!
    Ok(server)
}
