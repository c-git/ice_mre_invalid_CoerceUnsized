use async_trait::async_trait;
use serde::Deserialize;
use std::future::Future;
use tonic::{self, server::NamedService, transport::Server};

#[tokio::main]
async fn main() {
    let mut server_builder = Server::builder();

    let svc = RuntimeServer::new(runner);
    server_builder.add_service(svc);
}

async fn runner() {
    // Doesn't happen if I trivially replace either of these lines like with a todo!() instead of the assigned value
    let x: Wrapper = serde_json::from_slice("".as_bytes()).unwrap();
    let operator: opendal::Operator = x.inner.unwrap();

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
}

pub struct RuntimeServer<T: 'static + Clone> {
    runner: T,
}

#[derive(Default, Deserialize)]
struct Wrapper {
    #[serde(skip)]
    inner: Option<opendal::Operator>,
}

pub type BoxFuture<T, E> =
    std::pin::Pin<Box<dyn self::Future<Output = Result<T, E>> + Send + 'static>>;

#[async_trait]
pub trait Runner: Send + Sync + Clone {}

#[async_trait]
impl<T> Runner for RuntimeServer<T> where T: Runner + Send + 'static {}

#[async_trait]
impl<F, O> Runner for F
where
    F: FnOnce() -> O + Send + Sync + Clone,
    O: Future<Output = ()> + Send,
{
}

impl<T: Runner> Clone for RuntimeServer<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T: Runner> NamedService for RuntimeServer<T> {
    const NAME: &'static str = "";
}
impl<T: Runner> RuntimeServer<T> {
    pub fn new(_inner: T) -> Self {
        todo!()
    }
}

impl<T, B> tower::Service<http::Request<B>> for RuntimeServer<T>
where
    T: Runner,
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
                struct LoadSvc<T: Runner>(pub T);
                impl<T: Runner> tonic::server::UnaryService<()> for LoadSvc<T> {
                    type Response = ();
                    type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                    fn call(&mut self, _request: tonic::Request<()>) -> Self::Future {
                        todo!()
                    }
                }
                let inner = self.runner.clone();
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
