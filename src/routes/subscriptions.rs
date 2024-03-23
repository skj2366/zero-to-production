use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// 단순하게 시작하자: 항상 200 OK를 반환 한다.
pub async fn subscribe(
    form: web::Form<FormData>, pool: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
        // `get_ref`를 사용해서 `web::Data`로 감싸진 `PgConnection`에 대한 불변참조 (immutable reference)를 얻는다.
        .execute(pool.get_ref())
        .await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query : {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}