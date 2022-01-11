use actix_web::{guard, web, HttpResponse};
pub mod db;
mod handlers;
mod middleware;
mod role;
mod user;


pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::resource("/auth")
			.app_data(user::init_users)
			.route(
				web::route()
					.guard(guard::Any(guard::Get()).or(guard::Put()))
					.guard(guard::Header("Content-Type", "text/plain"))
					.to(|| HttpResponse::Ok()),
			),
	);
}

// "/", web::get().to(handlers::get_users))
//             .route("/users/{id}",
// web::get().to(handlers::get_user_by_id))
// .route("/", web::post().to(handlers::add_user))
// .route("/users/{id}",
// web::delete().to(handlers::delete_user))
