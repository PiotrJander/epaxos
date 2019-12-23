extern crate epaxos_rs;
extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate protobuf;

use epaxos_rs::epaxos::*;
use epaxos_rs::epaxos_grpc::*;
use grpc::{ClientStub, RequestOptions, SingleResponse};
use std::{
    cmp,
    collections::{HashMap, HashSet},
    env,
    sync::{Arc, Mutex},
    thread,
};
use grpc::rt::ServerServiceDefinition;

const QUORUM: u32 = 3;
const REPLICAS_NUM: u32 = 5;
const LOCALHOST: &str = "localhost";
static REPLICA_INTERNAL_PORTS: &'static [u16] = &[10000, 10001, 10002, 10003, 10004];
static REPLICA_EXTERNAL_PORTS: &'static [u16] = &[10000, 10001, 10002, 10003, 10004];

struct InstanceId(u32);

enum CommandState {
    PreAccepted,
    Accepted,
    Committed
}

struct Command {
    key: String,
    value: i32,
    seq: u32,
    dependencies: Vec<Instance>,
    state: CommandState
}

// TODO do we risk deadlocks by having mutexes and cloning?
#[derive(Clone)]
struct Epaxos {
    id: Arc<Mutex<InstanceId>>,
    //    store: Arc<Mutex<HashMap<String, i32>>>,
    commands: Arc<Mutex<Vec<Vec<Command>>>>,
    instance_number: Arc<Mutex<u32>>,
    replicas: Arc<Mutex<HashMap<InstanceId, InternalClient>>>,
}

impl Epaxos {

    fn new(id: InstanceId) -> Epaxos {

        let mut commands = Vec::new();
        let mut replicas = HashMap::new();

        for i in 0..REPLICAS_NUM {
            commands.push(Vec::new());

            if i != id.0 {
                let internal_client =
                    grpc::Client::new_plain(LOCALHOST, REPLICA_INTERNAL_PORTS[i], Default::default()).unwrap();
                let replica = InternalClient::with_client(grpc_replica1);
                replicas.insert(InstanceId(i), replica)
            }
        }

        return Epaxos {
            id: Arc::new(Mutex::new(id)),
            commands: Arc::new(Mutex::new(commands)),
            instance_number: Arc::new(Mutex::new(0)),
            replicas: Arc::new(Mutex::new(replicas)),
        };
    }

    fn consensus(&self, write_req: &WriteRequest) {
        unimplemented!()
    }
}

impl Internal for Epaxos {
    fn pre_accept(&self, o: RequestOptions, p: Payload) -> SingleResponse<Payload> {
        unimplemented!()
    }

    fn accept(&self, o: RequestOptions, p: Payload) -> SingleResponse<AcceptOKPayload> {
        unimplemented!()
    }

    fn commit(&self, o: RequestOptions, p: Payload) -> SingleResponse<Empty> {
        unimplemented!()
    }
}

impl External for Epaxos {
    fn write(&self, o: RequestOptions, p: WriteRequest) -> SingleResponse<WriteResponse> {
        self.consensus(&p);
        let mut r = WriteResponse::new();
        r.set_commit(true);
        grpc::SingleResponse::completed(r)
    }

    fn read(&self, o: RequestOptions, p: ReadRequest) -> SingleResponse<ReadResponse> {
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

    let id = args[1].parse().unwrap();
    let internal_port = REPLICA_INTERNAL_PORTS[id];
    let external_port = REPLICA_EXTERNAL_PORTS[id];

    let epaxos = Epaxos::new(InstanceId(id));
    start_server(InternalServer::new_service_def(epaxos), internal_port);
    start_server(ExternalServer::new_service_def(epaxos.clone()), external_port);

    // Blocks the main thread forever
    loop {
        thread::park();
    }
}