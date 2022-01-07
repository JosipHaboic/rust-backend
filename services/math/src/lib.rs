pub(crate) mod jsonrpc;



pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::service("/api")
            .guard(actix_web::guard::Post())
            .finish(
                (jsonrpc_v2::Server::new()
                    // .with_data(jsonrpc_v2::Data::new(String::from("Hello!")))
                    .with_method("sub", jsonrpc::sub)
                    .with_method("add", jsonrpc::add)
                    .with_method("mul", jsonrpc::mul)
                    .with_method("div", jsonrpc::div)
                    .with_method("timestamp", jsonrpc::timestamp)
                    .finish())
                .into_web_service(),
            ),
    );
}
