use hello_world::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run(TcpListener::bind("127.0.0.1:0")?)?.await
}