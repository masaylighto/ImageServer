use std::env;
use actix_web::{get, post,web::{self},App, HttpResponse, HttpServer, Responder};
mod web_server;

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    web_server::web_server::start_the_server()
}
