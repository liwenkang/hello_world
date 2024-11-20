use hello_world::configuration::get_configuration;
use hello_world::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    run(listener, connection_pool)?.await
}