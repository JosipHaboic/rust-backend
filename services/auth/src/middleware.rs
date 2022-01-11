use actix_service::{Service, Transform};
use actix_web::{
	dev::{ServiceRequest, ServiceResponse},
	Error,
};
use futures::{
	future::{ok, Ready},
	Future,
};

#[derive(Debug)]
pub struct User {
	pub id:       String,
	pub username: String,
	pub password: String,
	pub email:    String,
}

pub struct Authenticate;


impl<S, B> Transform<S> for Authenticate
where
	S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Error = Error;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;
	type InitError = ();
	type Request = ServiceRequest;
	type Response = ServiceResponse<B>;
	type Transform = AuthenticationMiddleware<S>;

	fn new_transform(&self, service: S) -> Self::Future {
		ok(AuthenticationMiddleware { service })
	}
}

pub struct AuthenticationMiddleware<S> {
	service: S,
}
