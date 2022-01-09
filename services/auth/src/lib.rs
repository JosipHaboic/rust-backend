use actix_web::{web, guard, HttpResponse};
mod handlers;
mod user;
mod role;
mod login;



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .app_data(user::init_users)
            .route(
                web::route()
                .guard(guard::Any(guard::Get()).or(guard::Put()))
                .guard(guard::Header("Content-Type", "text/plain"))
                .to(|| HttpResponse::Ok())
            )
    );
}

// "/", web::get().to(handlers::get_users))
//             .route("/users/{id}", web::get().to(handlers::get_user_by_id))
//             .route("/", web::post().to(handlers::add_user))
//             .route("/users/{id}", web::delete().to(handlers::delete_user))