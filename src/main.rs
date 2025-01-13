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
impl SerdeJsonOperator {
    pub async fn read_serialized<T: serde::de::DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<T, opendal::Error> {
        let bytes = self.0.read(key).await?;

        serde_json::from_slice(&bytes.to_vec()).map_err(|e| {
            opendal::Error::new(opendal::ErrorKind::Unexpected, "deserialization error")
                .set_source(e)
        })
    }
    pub async fn write_serialized<T: serde::Serialize>(
        &self,
        key: &str,
        value: &T,
    ) -> Result<(), opendal::Error> {
        let b = serde_json::to_vec(value).map_err(|e| {
            opendal::Error::new(opendal::ErrorKind::Unexpected, "serialization error").set_source(e)
        })?;

        self.0.write(key, b).await
    }
}

#[shuttle_runtime::async_trait]
impl IntoResource<SerdeJsonOperator>
    for <shuttle_shared_db::Postgres as ResourceInputBuilder>::Output
{
    async fn into_resource(self) -> Result<SerdeJsonOperator, shuttle_runtime::Error> {
        Ok(SerdeJsonOperator(self.into_resource().await?))
    }
}
