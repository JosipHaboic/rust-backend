use actix_session::CookieSession;
use actix_web::{http, middleware, middleware::errhandlers::ErrorHandlers, web, App, HttpServer};
use math_service;

pub mod route;
pub mod state;
pub mod templates;
// pub mod fbs;

pub struct ServiceInfo {
	pub name:    String,
	pub address: String,
}
impl ServiceInfo {
	pub fn new(name: &str, address: &str) -> ServiceInfo {
		ServiceInfo {
			name:    name.to_owned(),
			address: address.to_owned(),
		}
	}
}

pub type ServiceRegister = std::collections::HashMap<String, ServiceInfo>;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// logging
	std::env::set_var("RUST_LOG", "actix_web=warn");
	log4rs::init_file("log/config.yaml", Default::default()).unwrap();

	let address = "localhost";

	let port_0 = "8080";
	let port_1 = "8081";

	let services = ServiceRegister::from([
		(
			"app".to_string(),
			ServiceInfo::new("app", &format!("{}:{}", address, port_0)),
		),
		(
			"math-service".to_string(),
			ServiceInfo::new("math-service", &format!("{}:{}", address, port_1)),
		),
	]);

	for (service_name, service_info) in &services {
		log::info!(
			"[INFO] starting {} http://{}",
			&service_name,
			&service_info.address,
		);

		println!(
			"[INFO] starting {} http://{}",
			&service_name, &service_info.address,
		);
	}

	let app_server = HttpServer::new(move || {
		App::new()
			.data(state::Application {
				title:    String::from("BitFields"),
				version:  String::from("0.0.1"),
				services: [("math".to_owned(), jsonrpc_v2_client::ServiceAddress {
					url:      "127.0.0.1:8081".to_owned(),
					endpoint: "/api".to_owned(),
				})]
				.iter()
				.cloned()
				.collect(),
			})
			.wrap(CookieSession::signed(&[0; 32]).secure(true))
			.wrap(middleware::Logger::new("%a %{User-Agent}i"))
			.wrap(middleware::Compress::default())
			.wrap(ErrorHandlers::new().handler(
				http::StatusCode::INTERNAL_SERVER_ERROR,
				route::handlers::error::error_500,
			))
			.wrap(ErrorHandlers::new().handler(
				http::StatusCode::NOT_FOUND,
				route::handlers::error::error_404,
			))
			.service(actix_files::Files::new("/assets", "./assets").use_last_modified(true))
			.route("/", web::get().to(route::handlers::http::index))
			.route("/about", web::get().to(route::handlers::http::about))
	})
	.bind(
		&services
			.get("app")
			.unwrap()
			.address,
	)
	.unwrap()
	.run();

	let math_service_server = HttpServer::new(move || App::new().configure(math_service::config))
		.bind(
			&services
				.get("math-service")
				.unwrap()
				.address,
		)
		.unwrap()
		.run();

	futures::future::join_all(vec![app_server, math_service_server]).await;

	Ok(())
}
