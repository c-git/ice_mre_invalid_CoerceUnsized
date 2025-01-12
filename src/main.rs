use shuttle_runtime::tokio;

struct CustomService;
#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CustomService {
    async fn bind(mut self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    ::shuttle_runtime::__internals::start(__loader, __runner).await;
}

async fn __loader(
    factory: ::shuttle_runtime::ResourceFactory,
) -> ::std::result::Result<
    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
    ::shuttle_runtime::Error,
> {
    use ::shuttle_runtime::ResourceInputBuilder;
    let mut inputs = Vec::new();
    let input: <shuttle_shared_db::Postgres as ResourceInputBuilder>::Input =
        shuttle_shared_db::Postgres::default()
            .build(&factory)
            .await?;
    let json = ::shuttle_runtime::__internals::serde_json::to_vec(&input).unwrap();
    inputs.push(json);
    Ok(inputs)
}

async fn __runner(
    resources: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
) -> Result<CustomService, shuttle_runtime::Error> {
    use ::shuttle_runtime::{IntoResource, ResourceInputBuilder};
    let mut iter = resources.into_iter();
    let x: <shuttle_shared_db::Postgres as ResourceInputBuilder>::Output =
        ::shuttle_runtime::__internals::serde_json::from_slice(&iter.next().unwrap()).unwrap();
    let operator: shuttle_shared_db::SerdeJsonOperator = x.into_resource().await.unwrap();
    __shuttle_main(operator).await
}

async fn __shuttle_main(
    operator: shuttle_shared_db::SerdeJsonOperator,
) -> Result<CustomService, shuttle_runtime::Error> {
    operator.0.check().await.unwrap();
    todo!()
}
