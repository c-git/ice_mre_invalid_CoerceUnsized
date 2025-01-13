use shuttle_runtime::{
    IntoResource,
    __internals::{self, serde_json},
    tokio, ResourceFactory, ResourceInputBuilder,
};

struct CustomService;
#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CustomService {
    async fn bind(mut self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    __internals::start(loader, runner).await;
}

async fn loader(_factory: ResourceFactory) -> Result<Vec<Vec<u8>>, shuttle_runtime::Error> {
    todo!()
}

async fn runner(_resources: Vec<Vec<u8>>) -> Result<CustomService, shuttle_runtime::Error> {
    let x: <shuttle_shared_db::Postgres as ResourceInputBuilder>::Output =
        serde_json::from_slice("".as_bytes()).unwrap();
    let operator: opendal::Operator = x.into_resource().await.unwrap();
    operator.check().await.unwrap();
    todo!()
}
