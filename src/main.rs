use shuttle_runtime::{
    runtime::*,
    IntoResource,
    __internals::serde_json,
    async_trait, http,
    tokio::{self},
    ReceiverStream, ResourceInputBuilder,
};
use std::{
    future::Future,
    sync::{Arc, Mutex},
};
use tonic::{self, server::NamedService, transport::Server, Request, Response, Status};

#[tokio::main]
async fn main() {
    let mut server_builder = Server::builder();

    let alpha = Alpha::new(runner);

    let svc = RuntimeServer::new(alpha);
    server_builder.add_service(svc);
}

async fn runner() {
    // Doesn't happen if I trivially replace either of these lines like with a todo!() instead of the assigned value
    let x: <shuttle_shared_db::Postgres as ResourceInputBuilder>::Output =
        serde_json::from_slice("".as_bytes()).unwrap();
    let operator: opendal::Operator = x.into_resource().await.unwrap();

    // Didn't test
    // operator.delete_stream().await.unwrap();
    // operator.delete_try_stream().await.unwrap();

    // Only happens with any one of the follow async but not the other functions (barring the two mentioned as not tested above)
    operator.check().await.unwrap();
    // operator.write("", vec![0; 2]).await.unwrap();
    // operator.writer("").await.unwrap();
    // operator.write_with("", "").await.unwrap();
    // operator
    //     .delete_try_iter([Ok("")].into_iter())
    //     .await
    //     .unwrap();
    // operator.deleter().await.unwrap();
    // operator.list("").await.unwrap();
    // operator.remove_all("").await.unwrap();
    // operator.list_with("").await.unwrap();
    // operator.lister("").await.unwrap();
    // operator.lister_with("").await.unwrap();

    todo!()
}

#[allow(dead_code)]
pub struct Alpha<R> {
    runner: Mutex<Option<R>>,
}

impl<R> Alpha<R> {
    pub fn new(runner: R) -> Self {
        Self {
            runner: Mutex::new(Some(runner)),
        }
    }
}

#[async_trait]
impl<R> Runtime for Alpha<R>
where
    R: Runner + Send + 'static,
{
    async fn load(&self, _request: Request<LoadRequest>) -> Result<Response<LoadResponse>, Status> {
        todo!()
    }

    async fn start(
        &self,
        _request: Request<StartRequest>,
    ) -> Result<Response<StartResponse>, Status> {
        todo!()
    }

    async fn stop(&self, _request: Request<StopRequest>) -> Result<Response<StopResponse>, Status> {
        todo!()
    }

    type SubscribeStopStream = ReceiverStream<Result<SubscribeStopResponse, Status>>;

    async fn subscribe_stop(
        &self,
        _request: Request<SubscribeStopRequest>,
    ) -> Result<Response<Self::SubscribeStopStream>, Status> {
        todo!()
    }

    async fn version(&self, _request: Request<Ping>) -> Result<Response<VersionInfo>, Status> {
        todo!()
    }

    async fn health_check(&self, _request: Request<Ping>) -> Result<Response<Pong>, Status> {
        todo!()
    }
}

#[async_trait]
pub trait Runner {}

#[async_trait]
impl<F, O> Runner for F
where
    F: FnOnce() -> O + Send,
    O: Future<Output = ()> + Send,
{
}

#[async_trait]
pub trait Runtime: Send + Sync + 'static {
    /// Load a service file to be ready to start it
    async fn load(
        &self,
        request: tonic::Request<LoadRequest>,
    ) -> std::result::Result<tonic::Response<LoadResponse>, tonic::Status>;
    /// Start a loaded service file
    async fn start(
        &self,
        request: tonic::Request<StartRequest>,
    ) -> std::result::Result<tonic::Response<StartResponse>, tonic::Status>;
    /// Stop a started service
    async fn stop(
        &self,
        request: tonic::Request<StopRequest>,
    ) -> std::result::Result<tonic::Response<StopResponse>, tonic::Status>;
    /// Server streaming response type for the SubscribeStop method.
    type SubscribeStopStream: tonic::codegen::tokio_stream::Stream<
            Item = std::result::Result<SubscribeStopResponse, tonic::Status>,
        > + Send
        + 'static;
    /// Channel to notify a service has been stopped
    async fn subscribe_stop(
        &self,
        request: tonic::Request<SubscribeStopRequest>,
    ) -> std::result::Result<tonic::Response<Self::SubscribeStopStream>, tonic::Status>;
    async fn version(
        &self,
        request: tonic::Request<Ping>,
    ) -> std::result::Result<tonic::Response<VersionInfo>, tonic::Status>;
    async fn health_check(
        &self,
        request: tonic::Request<Ping>,
    ) -> std::result::Result<tonic::Response<Pong>, tonic::Status>;
}

pub struct RuntimeServer<T: Runtime> {
    inner: Arc<T>,
}

impl<T: Runtime> Clone for RuntimeServer<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Runtime> NamedService for RuntimeServer<T> {
    const NAME: &'static str = "";
}
impl<T: Runtime> RuntimeServer<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl<T, B> tower::Service<http::Request<B>> for RuntimeServer<T>
where
    T: Runtime,
    B: tonic::codegen::Body + Send + 'static,
    B::Error: Into<StdError> + Send + 'static,
{
    type Response = http::Response<tonic::body::BoxBody>;
    type Error = std::convert::Infallible;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, req: http::Request<B>) -> Self::Future {
        match req.uri().path() {
            "" => {
                #[allow(non_camel_case_types, dead_code)]
                struct LoadSvc<T: Runtime>(pub Arc<T>);
                impl<T: Runtime> tonic::server::UnaryService<LoadRequest> for LoadSvc<T> {
                    type Response = LoadResponse;
                    type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                    fn call(&mut self, _request: tonic::Request<LoadRequest>) -> Self::Future {
                        todo!()
                    }
                }
                let inner = self.inner.clone();
                let fut = async move {
                    let method = LoadSvc(inner);
                    let codec = tonic::codec::ProstCodec::default();
                    let mut grpc = tonic::server::Grpc::new(codec);
                    let res = grpc.unary(method, req).await;
                    Ok(res)
                };
                Box::pin(fut)
            }
            _ => todo!(),
        }
    }
}

pub type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type BoxFuture<T, E> =
    std::pin::Pin<Box<dyn self::Future<Output = Result<T, E>> + Send + 'static>>;
