mod handlers;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    HttpServer::new(move || {
        App::new()
            .service(handlers::get_users)
            .service(handlers::get_user_by_id)
            .service(handlers::add_user)
            .service(handlers::delete_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
