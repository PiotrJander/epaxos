extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate protobuf;

use futures::prelude::*;
use futures::future::join_all;

use crate::epaxos_grpc::*;
use grpc::{ClientStub, RequestOptions, SingleResponse};
use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
    thread,
};
use grpc::rt::ServerServiceDefinition;
use crate::logic::*;
use crate::conversions::*;
use crate::epaxos;
use std::sync::MutexGuard;

const QUORUM: usize = 3;
const REPLICAS_NUM: usize = 5;
const LOCALHOST: &str = "localhost";
static REPLICA_INTERNAL_PORTS: &'static [u16] = &[10000, 10001, 10002, 10003, 10004];
static REPLICA_EXTERNAL_PORTS: &'static [u16] = &[10010, 10011, 10012, 10013, 10014];

// always acquire in the same order to avoid deadlocks
#[derive(Clone)]
struct Server {
    replica: Arc<Mutex<Replica>>,
    internal_clients: Arc<Mutex<HashMap<ReplicaId, InternalClient>>>,
}

impl Server {
    fn new(id: ReplicaId) -> Self {
        let mut internal_clients = HashMap::new();
        for i in 0..REPLICAS_NUM {
            if i != id.0 {
                let client =
                    grpc::Client::new_plain(LOCALHOST, REPLICA_INTERNAL_PORTS[i], Default::default()).unwrap();
                let internal_client = InternalClient::with_client(Arc::new(client));
                internal_clients.insert(ReplicaId(i), internal_client);
            }
        }
        Server {
            replica: Arc::new(Mutex::new(Replica::new(id))),
            internal_clients: Arc::new(Mutex::new(internal_clients)),
        }
    }

    // TODO how do we choose the quorum? can it always be the same? how do we deal with replica failures?
    fn fast_quorum(&self) -> [ReplicaId; QUORUM - 1] {
        unimplemented!()
    }

    fn slow_path(&self, mut replica: MutexGuard<Replica>, accept_req: Accept) -> (WriteResponse, Commit) {
        // TODO send accept and obtain responses
        replica.leader_commit(accept_req.0)
    }
}

impl Internal for Server {
    fn pre_accept(&self, o: RequestOptions, p: epaxos::Payload) -> SingleResponse<epaxos::Payload> {
        let mut replica = self.replica.lock().unwrap();
        let request = PreAccept(Payload::from_grpc(&p));
        let response = replica.pre_accept_(request);
        grpc::SingleResponse::completed(response.0.to_grpc())
    }

    fn accept(&self, o: RequestOptions, p: epaxos::Payload) -> SingleResponse<epaxos::AcceptOKPayload> {
        let mut replica = self.replica.lock().unwrap();
        let request = Accept(Payload::from_grpc(&p));
        let response = replica.accept_(request);
        grpc::SingleResponse::completed(response.to_grpc())
    }

    fn commit(&self, o: RequestOptions, p: epaxos::Payload) -> SingleResponse<epaxos::Empty> {
        let mut replica = self.replica.lock().unwrap();
        let request = Commit(Payload::from_grpc(&p));
        replica.commit_(request);
        grpc::SingleResponse::completed(epaxos::Empty::new())
    }
}

impl External for Server {

    fn write(&self, o: RequestOptions, p: epaxos::WriteRequest) -> SingleResponse<epaxos::WriteResponse> {
        let mut replica = self.replica.lock().unwrap();
        let mut internal_clients = self.internal_clients.lock().unwrap();
        let request = WriteRequest::from_grpc(&p);
        let pre_accept_request = replica.write1(request);

        // make pre_accept requests in parallel
        let quorum = self.fast_quorum();
        let eventual_responses = quorum.iter().map(|id| {
            internal_clients[id].pre_accept(RequestOptions::new(), pre_accept_request.0.to_grpc()).drop_metadata()
        });
        let pre_accept_responses_grpc = join_all(eventual_responses).wait().unwrap(); // the parallel bit
        let pre_accept_responses: Vec<PreAcceptOK> =
            pre_accept_responses_grpc.iter().map(|resp| PreAcceptOK(Payload::from_grpc(resp))).collect();

        let (write_resp, commit_req) = match replica.write2(pre_accept_request, pre_accept_responses) {
            Path::Fast(write_resp, commit_req) => (write_resp, commit_req),
            Path::Slow(accept_req) => self.slow_path(replica, accept_req)
        };
        // TODO spawn async threads with commit
        grpc::SingleResponse::completed(write_resp.to_grpc())
    }

    fn read(&self, o: RequestOptions, p: epaxos::ReadRequest) -> SingleResponse<epaxos::ReadResponse> {
        unimplemented!()
    }
}

fn start_server(service: ServerServiceDefinition, port: u16) -> () {
    let mut server_builder = grpc::ServerBuilder::new_plain();
    server_builder.add_service(service);
    server_builder.http.set_port(port);
    let server = server_builder.build().expect("build");
    println!("Server started on address {}", server.local_addr());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let id: usize = args[1].parse().unwrap();
    let internal_port = REPLICA_INTERNAL_PORTS[id];
    let external_port = REPLICA_EXTERNAL_PORTS[id];

    let server = Server::new(ReplicaId(id));
    start_server(InternalServer::new_service_def(server.clone()), internal_port);
    start_server(ExternalServer::new_service_def(server.clone()), external_port);

    // Blocks the main thread forever
    loop {
        thread::park();
    }
}