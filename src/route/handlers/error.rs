use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, http, HttpResponse, Result};
use tera::Context;

use crate::templates;

pub fn error_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let mut context = Context::new();
    context.insert("title", &"");
    context.insert("version", &"");

    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );

    let response = dev::ServiceResponse::new(
        res.request().clone(),
        HttpResponse::Ok()
            .body(
                templates::TEMPLATES
                    .render("error/500.html", &context)
                    .unwrap(),
            )
            .into_body(),
    );

    Ok(ErrorHandlerResponse::Response(response))
}

pub fn error_404<B>(
    mut res: dev::ServiceResponse<B>,
    // data: web::Data<state::Application>
) -> Result<ErrorHandlerResponse<B>> {
    let mut context = Context::new();
    context.insert("title", &"");
    context.insert("version", &"");

    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Not found"),
    );

    let response = dev::ServiceResponse::new(
        res.request().clone(),
        HttpResponse::Ok()
            .body(
                templates::TEMPLATES
                    .render("error/404.html", &context)
                    .unwrap(),
            )
            .into_body(),
    );

    Ok(ErrorHandlerResponse::Response(response))
}
