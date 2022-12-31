pub mod config;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use log::info;
use env_logger::Env;

async fn shortcode(_req: HttpRequest, code: web::Path<String>) -> impl Responder {
    info!(target: "go-links", "Request for shortcode {}", &code);
    format!("shortcode: {}", &code)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::parse_config("config.toml").unwrap();
    
    // load TLS keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();

    builder.set_certificate_chain_file("cert.pem").unwrap();

    let env = Env::default()
        .filter_or("LOG_LEVEL", cfg.log_level)
        .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let server_address = format!("{}:{}", cfg.address, cfg.port);
    
    HttpServer::new(|| App::new().route("/{code}", web::get().to(shortcode)))
        .bind_openssl(server_address, builder)?
	.run()
	.await
}
