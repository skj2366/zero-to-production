use std::io::{Result};
use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;

// `impl Responder`를 초반에 반환한다.
// `actix-web`에 익숙해졌으므로, 주어진 타입을 명시적으로 기술한다.
// 이로 인한 성능의 차이는 없다. 그저 스타일과 관련된 선택일 뿐이다.
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

// 단순하게 시작하자: 항상 200 OK를 반환한다.
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            // POST /subscriptions 요청에 대한 라우팅 테이블의 새 엔트리 포인트
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();
    Ok(server)
}