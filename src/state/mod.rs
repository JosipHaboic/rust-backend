use std::collections::HashMap;

use jsonrpc_v2_client::ServiceAddress;

pub struct Application {
	pub title:    String,
	pub version:  String,
	pub services: HashMap<String, ServiceAddress>,
}
