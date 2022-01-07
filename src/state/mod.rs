use jsonrpc_v2_client::ServiceAddress;
use std::collections::HashMap;

pub struct Application {
    pub title: String,
    pub version: String,
    pub services: HashMap<String, ServiceAddress>,
}
