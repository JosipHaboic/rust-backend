#[cfg(test)]// The module is only compiled when testing.
mod test {

	use actix_web::{test, App};
	use math_service::config;
	use serde_json::Value;

	#[actix_rt::test]
	async fn test_sub() {

		let mut app = test::init_service(
			App::new().configure(config)
		).await;

		let request_params = serde_json::json!({
    		"jsonrpc": "2.0",
    		"method": "sub",
    		"params": [3_u64, 2_u64],
    		"id": "1"
		});

		let request = test::TestRequest::post()
			.uri("/api")
			.set_json(&request_params)
			.to_request();

		let response = test::call_service(&mut app, request).await;

		assert!(response.status().is_success());

		let service_data: Value = test::read_body_json(response).await;

		assert_eq!(service_data["error"], Value::Null);
		assert_eq!(service_data["id"], Value::String("1".to_string()));

		println!("{}", service_data["result"]);
		assert_eq!(service_data["result"].as_f64().unwrap(), 1.0_f64);
	}

	#[actix_rt::test]
	async fn test_add() {

		let mut app = test::init_service(
			App::new().configure(config)
		).await;

		let request_params = serde_json::json!({
    		"jsonrpc": "2.0",
    		"method": "add",
    		"params": [3.0_f64, 2.0_f64],
    		"id": "0"
		});

		let request = test::TestRequest::post()
			.uri("/api")
			.set_json(&request_params)
			.to_request();

		let response = test::call_service(&mut app, request).await;

		assert!(response.status().is_success());

		let service_data: Value = test::read_body_json(response).await;

		assert_eq!(service_data["error"], Value::Null);
		assert_eq!(service_data["id"], Value::String("0".to_string()));

		println!("{}", service_data["result"]);
		assert_eq!(service_data["result"].as_f64().unwrap(), 5.0_f64);
	}
}