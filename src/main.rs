use std::net::TcpListener;

use sqlx::{Connection, PgPool};

use zero_to_production::configuration::get_configuration;
use zero_to_production::startup::run;

#[tokio::main]
async fn main() ->std::io::Result<()> {
    // 구성을 읽을 수 없으면 패닉에 빠진다.
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("Failed to connect to Postgres.");
    // 하드 코딩했던 `8000`을 제거했다. 해당 값은 세팅에서 읽는다.
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    run(listener, connection_pool)?.await
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
//             .routes("/", web::get().to(health_check))
//             .routes("/{name}", web::get().to(greet))
//     })
//         .bind((
//             server_url, server_port
//         ))?
//         .run()
//         .await
// }