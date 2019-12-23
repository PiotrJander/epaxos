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

pub const QUORUM: u16 = 2;
pub const REPLICAS_NUM: u16 = 3;
pub const REPLICA1_PORT: u16 = 10000;
pub const REPLICA2_PORT: u16 = 10001;
pub const REPLICA3_PORT: u16 = 10002;
pub const REPLICA4_PORT: u16 = 10003;
pub const REPLICA5_PORT: u16 = 10004;
pub const LOCALHOST: &str = "localhost";

struct Command {
    key: String,
    value: i32,
}

// TODO do we risk deadlocks by having mutexes and cloning?
#[derive(Clone)]
struct Epaxos {
    id: u32,
    //    store: Arc<Mutex<HashMap<String, i32>>>,
    commands: Arc<Mutex<Vec<Vec<Command>>>>,
    instance_number: Arc<Mutex<u32>>,
    replicas: Arc<Mutex<Vec<InternalClient>>>,
}

impl Epaxos {
    fn new(id: u32) -> Epaxos {
        let mut commands = Vec::new();
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
        unimplemented!()
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

    let id: u32 = args[1].parse().unwrap();
    let internal_port: u16 = args[2].parse().unwrap();
    let external_port: u16 = args[3].parse().unwrap();

    let epaxos = Epaxos::new(id);
    start_server(InternalServer::new_service_def(epaxos), internal_port);
    start_server(ExternalServer::new_service_def(epaxos.clone()), external_port);

    // Blocks the main thread forever
    loop {
        thread::park();
    }
}