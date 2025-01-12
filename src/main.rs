struct CustomService;
#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CustomService {
    async fn bind(mut self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        todo!()
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] operator: shuttle_shared_db::SerdeJsonOperator,
) -> Result<CustomService, shuttle_runtime::Error> {
    operator.0.check().await.unwrap();
    todo!()
}