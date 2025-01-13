use shuttle_runtime::{
    IntoResource,
    __internals::{self, serde_json},
    start, tokio, ResourceFactory, ResourceInputBuilder,
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
    start(loader, runner).await;
}

async fn loader(_factory: ResourceFactory) -> Result<Vec<Vec<u8>>, shuttle_runtime::Error> {
    todo!()
}

async fn runner(_resources: Vec<Vec<u8>>) -> Result<CustomService, shuttle_runtime::Error> {
    // Doesn't happen if I trivially replace either of these lines like with a todo!() instead of the assigned value
    let x: <shuttle_shared_db::Postgres as ResourceInputBuilder>::Output =
        serde_json::from_slice("".as_bytes()).unwrap();
    let operator: opendal::Operator = x.into_resource().await.unwrap();

    // Didn't test
    // operator.delete_stream().await.unwrap();
    // operator.delete_try_stream().await.unwrap();

    // Only happens with any one of the follow async but not the other functions (barring the two mentioned as not tested above)
    operator.check().await.unwrap();
    operator.write("", vec![0; 2]).await.unwrap();
    operator.writer("").await.unwrap();
    operator.write_with("", "").await.unwrap();
    operator
        .delete_try_iter([Ok("")].into_iter())
        .await
        .unwrap();
    operator.deleter().await.unwrap();
    operator.list("").await.unwrap();
    operator.remove_all("").await.unwrap();
    operator.list_with("").await.unwrap();
    operator.lister("").await.unwrap();
    operator.lister_with("").await.unwrap();

    todo!()
}
