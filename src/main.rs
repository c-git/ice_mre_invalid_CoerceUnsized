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

async fn runner(resources: Vec<Vec<u8>>) -> Result<CustomService, shuttle_runtime::Error> {
    let mut iter = resources.into_iter();
    let x: <shuttle_shared_db::Postgres as ResourceInputBuilder>::Output =
        serde_json::from_slice(&iter.next().unwrap()).unwrap();
    let operator: SerdeJsonOperator = x.into_resource().await.unwrap();
    operator.0.check().await.unwrap();
    todo!()
}
#[derive(Clone, Debug)]
pub struct SerdeJsonOperator(pub opendal::Operator);
impl SerdeJsonOperator {}

#[shuttle_runtime::async_trait]
impl IntoResource<SerdeJsonOperator>
    for <shuttle_shared_db::Postgres as ResourceInputBuilder>::Output
{
    async fn into_resource(self) -> Result<SerdeJsonOperator, shuttle_runtime::Error> {
        todo!()
    }
}
