use chrono::prelude::*;
use jsonrpc_v2::{Error, Params};

#[derive(serde::Deserialize)]
pub struct TwoNumbers {
    a: f32,
    b: f32,
}

pub async fn add(Params(params): Params<TwoNumbers>) -> Result<f32, Error> {
    Ok(params.a + params.b)
}

pub async fn sub(Params(params): Params<(f32, f32)>) -> Result<f32, Error> {
    Ok(params.a - params.b)
}

pub async fn mul(Params(params): Params<(f32, f32)>) -> Result<f32, Error> {
    Ok(params.a * params.b)
}

pub async fn div(Params(params): Params<(f32, f32)>) -> Result<f32, Error> {
    Ok(params.a / params.b)
}

pub async fn timestamp() -> Result<String, Error> {
    Ok(Utc::now().to_string())
}
