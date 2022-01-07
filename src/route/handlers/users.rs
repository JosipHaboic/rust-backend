use actix_web::web;
use actix_web::{Error, HttpRequest, HttpResponse, Result};
use sqlx::Row;
use tera::Context;

use crate::models::user::User;
use crate::state;
use crate::templates::TEMPLATES;

pub async fn show(
    _req: HttpRequest,
    data: web::Data<state::Application>,
) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    context.insert("title", &data.title);
    context.insert("version", &data.version);

    data.database.acquire().await.unwrap();

    let users: Vec<User> = sqlx::query(r#"SELECT * FROM users"#)
        .fetch_all(&data.database)
        .await
        .unwrap()
        .iter()
        .map(|row| -> User {
            User {
                id: row.get("id"),
                username: row.get("username"),
                password: row.get("password"),
            }
        })
        .collect();

    context.insert("users", &users);

    Ok(HttpResponse::Ok().body(TEMPLATES.render("users/show.html", &context).unwrap()))
}
