use std::sync::{Arc, Mutex};

use runtime_server::Runtime;
use shuttle_runtime::{
    runtime::*,
    tonic::{transport::Server, Request, Response, Status},
    IntoResource,
    __internals::serde_json,
    async_trait,
    tokio::{
        self,
        sync::{
            broadcast::{self, Sender},
            oneshot,
        },
    },
    Loader, ReceiverStream, ResourceInputBuilder, Runner, RuntimeServer, Service, State,
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
    let loader = |_| async { Ok(vec![]) };

    let mut server_builder = Server::builder();

    let alpha = Alpha::new(loader, runner);

    let svc = RuntimeServer::new(alpha);
    server_builder.add_service(svc);
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

pub struct Alpha<L, R> {
    // Mutexes are for interior mutability
    stopped_tx: Sender<(StopReason, String)>,
    kill_tx: Mutex<Option<oneshot::Sender<String>>>,
    loader: Mutex<Option<L>>,
    runner: Mutex<Option<R>>,
    /// The current state of the runtime, which is used by the ECS task to determine if the runtime
    /// is healthy.
    state: Arc<Mutex<State>>,
}

impl<L, R> Alpha<L, R> {
    pub fn new(loader: L, runner: R) -> Self {
        let (stopped_tx, _stopped_rx) = broadcast::channel(10);

        Self {
            stopped_tx,
            kill_tx: Mutex::new(None),
            loader: Mutex::new(Some(loader)),
            runner: Mutex::new(Some(runner)),
            state: Arc::new(Mutex::new(State::Unhealthy)),
        }
    }
}

#[async_trait]
impl<L, R, S> Runtime for Alpha<L, R>
where
    L: Loader + Send + 'static,
    R: Runner<Service = S> + Send + 'static,
    S: Service + 'static,
{
    async fn load(&self, request: Request<LoadRequest>) -> Result<Response<LoadResponse>, Status> {
        todo!()
    }

    async fn start(
        &self,
        request: Request<StartRequest>,
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

    async fn version(&self, _requset: Request<Ping>) -> Result<Response<VersionInfo>, Status> {
        todo!()
    }

    async fn health_check(&self, _request: Request<Ping>) -> Result<Response<Pong>, Status> {
        todo!()
    }
}
