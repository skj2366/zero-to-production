// use std::env;
use std::io::Result;
use std::net::TcpListener;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
// use dotenv::dotenv;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server> {
    // dotenv().ok();
    // let server_url = env::var("SERVER_URL").unwrap();
    // let server_port = env::var("SERVER_PORT").unwrap().parse().unwrap();
    // let welcome_string: String = "👏".to_string();
    // println!("Hello Rust! {}", welcome_string);
    // println!("Running SERVER 🐱‍🏍 Success => http://{}:{}", &server_url, &server_port);

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
        .listen(listener)?
        .run();
    Ok(server)
}