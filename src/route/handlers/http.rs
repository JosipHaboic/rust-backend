use actix_web::web;
use actix_web::{Error, HttpRequest, HttpResponse, Result};
use tera::Context;
use jsonrpc_v2_client;

use crate::state;
use crate::templates;



pub async fn index(
    _req: HttpRequest,
    data: web::Data<state::Application>,
) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    context.insert("title", &data.title);
    context.insert("version", &data.version);

    Ok(HttpResponse::Ok().body(
        templates::TEMPLATES
            .render("http/index.html", &context)
            .unwrap(),
    ))
}

pub async fn about(
    _req: HttpRequest,
    data: web::Data<state::Application>,
) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    context.insert("title", &data.title);
    context.insert("version", &data.version);

    let api_key = jsonrpc_v2_client::APIKey::new("X-API-KEY", "1q2w3e4r5t");

    let jsonrpc_response = jsonrpc_v2_client::Request::new(
        "mul",
        jsonrpc_v2_client::Params([10.3, 10.1]),
        "0"
    ).send(
        data.services.get("math").unwrap(),
        Some(&api_key)
    );

    context.insert("jsonrpc", &jsonrpc_response["result"]);

    Ok(HttpResponse::Ok().body(
        templates::TEMPLATES
            .render("http/about.html", &context)
            .unwrap(),
    ))
}
