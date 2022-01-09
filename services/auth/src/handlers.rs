use actix_web::Responder;


pub async fn get_users() -> impl Responder {
	format!("hello from get_users")
}

pub async fn get_user_by_id() -> impl Responder {
	format!("hello from get_user_by_id")
}

pub async fn add_user() -> impl Responder {
	format!("hello from add_user")
}

pub async fn delete_user() -> impl Responder {
	format!("hello from delete_user")
}
