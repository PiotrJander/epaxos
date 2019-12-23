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
    Committed
}

struct InstanceRef {
    replica: ReplicaId,
    slot: usize
}

struct Instance {
    key: String,
    value: i32,
    seq: usize,
    dependencies: Vec<InstanceRef>,
    state: CommandState
}

// TODO do we risk deadlocks by having mutexes and cloning?
#[derive(Clone)]
struct Epaxos {
    id: ReplicaId,
    //    store: Arc<Mutex<HashMap<String, i32>>>,
    commands: Arc<Mutex<Vec<Vec<Instance>>>>,
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

    // FIXME we only record write commands - is this okay?
    // FIXME is this correct?
    // Piotr to Pi: your `find_interference` was wrong because it only considered
    // the current replica's row.
    fn find_interference(&self, key: &String) -> Vec<InstanceRef> {
        let mut acc = Vec::new();
        let commands = self.commands.lock().unwrap();
        for q in 0..commands.len() {
            for j in 0..commands[q].len() {
                if commands[q][j].key == *key {
                    acc.push(InstanceRef { replica: ReplicaId(q), slot: j })
                }
            }
        }
        acc
    }

    // FIXME how can we avoid acquiring locks on mutexes all the time?
    fn find_seq(&self, deps: &Vec<InstanceRef>) -> usize {
        let mut acc = 0;
        for dep in deps {
            let commands = self.commands.lock().unwrap();
            let instance = &commands[dep.replica.0][dep.slot];
            acc = cmp::max(acc, instance.seq)
        }
        acc + 1
    }

    fn consensus(&self, write_req: &WriteRequest) {
        // TODO how to guarantee that the below action are atomic?
        let deps = self.find_interference(&write_req.key);
        let seq = self.find_seq(&deps);
        let mut commands = self.commands.lock().unwrap();
        commands[self.id.0].push(Instance {
            key: write_req.key.clone(),
            value: write_req.value,
            seq,
            dependencies: deps,
            state: PreAccepted
        })
        // end TODO

        // TODO send internal messages and continue
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

    let epaxos = Epaxos::new(ReplicaId(id));
    start_server(InternalServer::new_service_def(epaxos.clone()), internal_port);
    start_server(ExternalServer::new_service_def(epaxos.clone()), external_port);

    // Blocks the main thread forever
    loop {
        thread::park();
    }
}