use std::{
	pin::Pin,
	task::{Context, Poll},
};

use actix_service::{Service, Transform};
use actix_web::{
	dev::{ServiceRequest, ServiceResponse},
	Error,
};
use futures::{
	future::{ok, Ready},
	Future,
};

pub type AuthenticationInfo = Rc<AuthenticationResult>;

pub struct AuthenticationMiddleware<S> {
	auth_data: Rc<AuthenticationData>,
	service:   Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static
{
	type Error = Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
	type Response = ServiceResponse<B>;

	actix_service::forward_ready!(service);

	fn call(&self, req: ServiceRequest) -> Self::Future {
		let srv = self
			.service
			.clone();
		let auth_data = self
			.auth_data
			.clone();

		async move {
			let id = req.get_identity();

			let auth = auth_data
				.authenticate(id, &req)
				.await?;

			if let Some(auth) = auth {
				req.extension_mut()
					.insert::<AuthenticationInfo>(Rc::new(auth));
			}

			let res = srv
				.call(req)
				.await?;

			Ok(res);
		}
		.boxed_local()
	}
}

pub struct AuthenticationMiddlewareFactory {
	auth_data: Rc<AuthData>,
}
impl AuthenticateMiddlewareFactory {
	pub fn new(auth_data: AuthData) -> Self {
		AuthenticationMiddlewareFactory {
			auth_data: Rc::new(auth_data),
		}
	}
}
impl<S, B> Transform<S, ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static {
	type Error = Error;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;
	type InitError = ();
	type Response = ServiceResponse<B>;
	type Transform = AuthenticationMiddleware<S>;

	fn new_transform(&self, service: S) -> Self::Future {
		ready(Ok(AuthenticationMiddleware {
			auth_data: self
				.auth_data
				.clone(),
			service:   Rc::new(service),
		}))
	}
}


// #[derive(Debug)]
// pub struct User {
// 	pub id:       String,
// 	pub username: String,
// 	pub password: String,
// 	pub email:    String,
// }

// pub struct AuthenticationData;

// pub type AuthenticationInfo = Rc<AuthenticationResult>;

// impl<S, B> Transform<S> for AuthenticationData
// where
// 	S: Service<Request = ServiceRequest, Response =
// ServiceResponse<B>, Error = Error>, 	S::Future: 'static,
// 	B: 'static,
// {
// 	type Error = Error;
// 	type Future = Ready<Result<Self::Transform,
// Self::InitError>>; 	type InitError = ();
// 	type Request = ServiceRequest;
// 	type Response = ServiceResponse<B>;
// 	type Transform = AuthenticationMiddleware<S>;

// 	fn new_transform(&self, service: S) -> Self::Future {
// 		ok(AuthenticationMiddleware { service })
// 	}
// }


// pub struct AuthenticationMiddleware<S> {
// 	service: S,
// }

// impl<S, B> Service for AuthenticationMiddleware<S>
// where
// 	S: Service<Request = ServiceRequest, Response =
// ServiceResponse<B>, Error = Error>, 	S::Future: 'static,
// 	B: 'static,
// {
// 	type Error = Error;
// 	type Future = Pin<Box<dyn Future<Output =
// Result<Self::Response, Self::Error>>>>; 	type Request =
// ServiceRequest; 	type Response = ServiceResponse<B>;

// 	fn poll_ready(&mut self, cx: &mut Context<'_>) ->
// Poll<Result<(), Self::Error>> { 		self.service
// 			.poll_ready(cx)
// 	}

// 	fn call(&mut self, req: ServiceRequest) -> Self::Future {
// 		println!("Hi from start. You requested: {}", req.path());

// 		let fut = self
// 			.service
// 			.call(req);

// 		Box::pin(async move {
// 			let res = fut.await?;

// 			println!("Hi from response");

// 			Ok(res)
// 		})
// 	}
// }
