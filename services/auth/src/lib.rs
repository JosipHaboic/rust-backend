use actix_web::{guard, web, HttpResponse};
mod db;
mod handlers;


pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/auth")
			.app_data(())
			.service(
				web::resource("/login")
					.name("auth-login")
					.route(
						web::route()
							.guard(guard::Post())
							.guard(guard::Header("Content-Type", "application/json"))
							.to(|| HttpResponse::Ok()),
					),
			)
			.service(
				web::resource("/logout")
					.name("auth-logout")
					.route(
						web::route()
							.guard(guard::Post())
							.guard(guard::Header("Content-Type", "application/json"))
							.to(|| HttpResponse::Ok()),
					),
			)
			.service(
				web::resource("/register")
					.name("auth-register")
					.route(
						web::route()
							.guard(guard::Post())
							.guard(guard::Header("Content-Type", "application/json"))
							.to(|| HttpResponse::Ok()),
					),
			),
	);
}
