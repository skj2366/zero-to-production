// use zero_to_production::run;

#[tokio::main]
// async fn main() -> std::io::Result<()> {
    // run()?.await
// }
async fn main() {

}

// #[tokio::main]
// async fn main() -> Result<()> {
//     dotenv().ok();
//     let server_url = env::var("SERVER_URL").unwrap();
//     let server_port = env::var("SERVER_PORT").unwrap().parse().unwrap();
//     let welcome_string: String = "👏".to_string();
//     println!("Hello Rust! {}", welcome_string);
//     println!("Running SERVER 🐱‍🏍 Success => http://{}:{}", &server_url, &server_port);
//
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(health_check))
//             .route("/{name}", web::get().to(greet))
//     })
//         .bind((
//             server_url, server_port
//         ))?
//         .run()
//         .await
// }