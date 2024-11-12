use rwf::prelude::*;
use rwf::http::Server;

#[derive(Default)]
struct IndexController;

#[async_trait]
impl Controller for IndexController {
    async fn handle(&self, request: &Request) -> Result<Response, Error> {
        Ok(Response::new().html("<h1>Hey Rwf!</h1>"))
    }
}

#[tokio::main]
async fn main() {
    Server::new(vec![
        route!("/" => IndexController),
    ])
    .launch("0.0.0.0:8082")
    .await
    .unwrap();
}