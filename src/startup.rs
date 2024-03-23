use std::net::TcpListener;
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;

pub fn run(
    listener: TcpListener,
    //새로운 매개변수
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    // web::Data로 pool을 감싼다. Arc 스마트 포인터로 요약된다.
    let db_pool = web::Data::new(db_pool);
    // 주변 환경으로부터 `connection`을 잡아낸다.
    let server = HttpServer::new(move || {
        App::new()
            // .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // 포인터의 사본을 얻어 애플리케이션 상태에 추가한다.
            .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}