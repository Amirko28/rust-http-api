use actix_web::{App, HttpServer, middleware};
use routers::users::users_router;

pub mod routers;
pub mod business_logic;
pub mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .service(users_router())
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}