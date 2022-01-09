use serde::{Deserialize, Serialize};


#[derive(Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub pw: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
