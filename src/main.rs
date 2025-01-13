use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    future::Future,
    sync::{Arc, Mutex},
};
use tonic::{self, server::NamedService, transport::Server};

#[tokio::main]
async fn main() {
    let mut server_builder = Server::builder();

    let alpha = Alpha::new(runner);

    let svc = RuntimeServer::new(alpha);
    server_builder.add_service(svc);
}

#[derive(Default, Serialize, Deserialize)]
struct Wrapper {
    #[serde(skip)]
    inner: Option<opendal::Operator>,
}
#[derive(Default, Serialize, Deserialize)]
struct CustomResource;
#[async_trait]
impl ResourceInputBuilder for CustomResource {
    type Input = ();
    type Output = Wrapper;

    async fn build(
        self,
        _factory: &ResourceFactory,
    ) -> Result<Self::Input, shuttle_runtime::Error> {
        todo!()
    }
}

async fn runner() {
    // Doesn't happen if I trivially replace either of these lines like with a todo!() instead of the assigned value
    let x: Wrapper = serde_json::from_slice("".as_bytes()).unwrap();
    let operator: opendal::Operator = x.into_resource1().unwrap();

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
    pub fn new(_runner: R) -> Self {
        todo!()
    }
}

#[async_trait] // No ICE if Runner constraint is removed
impl<R> Runtime for Alpha<R> where R: Runner + Send + 'static {}

#[async_trait]
pub trait Runner {}

#[async_trait]
impl<F, O> Runner for F
where
    F: FnOnce() -> O,
    O: Future<Output = ()> + Send,
{
}

#[async_trait]
pub trait Runtime: Send + Sync + 'static {}

pub struct RuntimeServer<T: Runtime> {
    inner: Arc<T>,
}

impl<T: Runtime> Clone for RuntimeServer<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T: Runtime> NamedService for RuntimeServer<T> {
    const NAME: &'static str = "";
}
impl<T: Runtime> RuntimeServer<T> {
    pub fn new(_inner: T) -> Self {
        todo!()
    }
}

impl<T, B> tower::Service<http::Request<B>> for RuntimeServer<T>
where
    T: Runtime,
    B: tonic::codegen::Body + Send + 'static,
    B::Error: std::error::Error + Sync + Send + 'static,
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
                impl<T: Runtime> tonic::server::UnaryService<()> for LoadSvc<T> {
                    type Response = ();
                    type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                    fn call(&mut self, _request: tonic::Request<()>) -> Self::Future {
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

pub type BoxFuture<T, E> =
    std::pin::Pin<Box<dyn self::Future<Output = Result<T, E>> + Send + 'static>>;

#[async_trait]
pub trait IntoResource<R>: Serialize + DeserializeOwned {
    async fn into_resource(self) -> Result<R, shuttle_runtime::Error>;
}

#[derive(Serialize, Deserialize)]
pub struct ResourceFactory {}

/// want to provision and how to do it on your behalf on the fly.
#[async_trait]
pub trait ResourceInputBuilder: Default {
    type Input: Serialize + DeserializeOwned;

    type Output: Serialize + DeserializeOwned;

    async fn build(self, factory: &ResourceFactory) -> Result<Self::Input, shuttle_runtime::Error>;
}

impl Wrapper {
    fn into_resource1(self) -> Result<opendal::Operator, shuttle_runtime::Error> {
        todo!()
    }
}
