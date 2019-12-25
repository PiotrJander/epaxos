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
use crate::CommandState::PreAccepted;

const QUORUM: usize = 3;
const REPLICAS_NUM: usize = 5;
const LOCALHOST: &str = "localhost";
static REPLICA_INTERNAL_PORTS: &'static [u16] = &[10000, 10001, 10002, 10003, 10004];
static REPLICA_EXTERNAL_PORTS: &'static [u16] = &[10010, 10011, 10012, 10013, 10014];

#[derive(PartialEq, Eq, Hash, Clone)]
struct ReplicaId(usize);

enum CommandState {
    PreAccepted,
    Accepted,
    Committed,
}

struct InstanceRef {
    replica: ReplicaId,
    slot: usize,
}

struct Instance {
    key: String,
    value: i32,
    seq: usize,
    dependencies: Vec<InstanceRef>,
    state: CommandState,
}

type Commands = Vec<Vec<Instance>>;

// TODO always acquire in the same order and acquire the first mutex to have atomicity
#[derive(Clone)]
struct Epaxos {
    id: ReplicaId,
    //    store: Arc<Mutex<HashMap<String, i32>>>,
    commands: Arc<Mutex<Commands>>,
    //    instance_number: Arc<Mutex<u32>>, // maybe not needed due to using vectors
    replicas: Arc<Mutex<HashMap<ReplicaId, InternalClient>>>,
}

impl Epaxos {
    fn new(id: ReplicaId) -> Epaxos {
        let mut commands = Vec::new();
        let mut replicas = HashMap::new();

        for i in 0..REPLICAS_NUM {
            commands.push(Vec::new());

            if i != id.0 {
                let internal_client =
                    grpc::Client::new_plain(LOCALHOST, REPLICA_INTERNAL_PORTS[i], Default::default()).unwrap();
                let replica = InternalClient::with_client(Arc::new(internal_client));
                replicas.insert(ReplicaId(i), replica);
            }
        }

        return Epaxos {
            id,
            commands: Arc::new(Mutex::new(commands)),
            replicas: Arc::new(Mutex::new(replicas)),
        };
    }

    fn pre_accept_(&self, p: Payload) -> Payload {
        unimplemented!()
    }

    fn accept_(&self, p: Payload) -> AcceptOKPayload {
        unimplemented!()
    }

    fn commit_(&self, p: Payload) -> () {
        unimplemented!()
    }

    fn fast_quorum(&self) -> Vec<ReplicaId> {
        unimplemented!()
    }

    fn establish_ordering_constraints(&self, key: String, value: i32) {
        let mut commands = self.commands.lock().unwrap();
        let dependencies = find_interference(&commands, &key);
        let seq = find_next_seq(&commands, &dependencies);
        commands[self.id.0].push(Instance {
            key,
            value,
            seq,
            dependencies,
            state: PreAccepted,
        });
    }

    fn write_(&self, p: WriteRequest) -> WriteResponse {
        self.establish_ordering_constraints(p.key, p.value);

        // TODO send internal messages and continue

        new_write_response(true)
    }

    fn read_(&self, p: ReadRequest) -> ReadResponse {
        unimplemented!()
    }
}

fn new_write_response(committed: bool) -> WriteResponse {
    let mut r = WriteResponse::new();
    r.set_commit(committed);
    r
}

// FIXME we only record write commands - is this okay?
// FIXME is this correct?
fn find_interference(commands: &Commands, key: &String) -> Vec<InstanceRef> {
    let mut acc = Vec::new();
    for (q, row) in commands.iter().enumerate() {
        for (j, instance) in row.iter().enumerate() {
            if instance.key == *key {
                acc.push(InstanceRef { replica: ReplicaId(q), slot: j })
            }
        }
    }
    acc
}

fn find_next_seq(commands: &Commands, deps: &Vec<InstanceRef>) -> usize {
    let mut acc = 0;
    for dep in deps {
        let instance = &commands[dep.replica.0][dep.slot];
        acc = cmp::max(acc, instance.seq)
    }
    acc + 1
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
        grpc::SingleResponse::completed(self.write_(p))
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

    let epaxos = Epaxos::new(ReplicaId(id));
    start_server(InternalServer::new_service_def(epaxos.clone()), internal_port);
    start_server(ExternalServer::new_service_def(epaxos.clone()), external_port);

    // Blocks the main thread forever
    loop {
        thread::park();
    }
}