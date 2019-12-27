use crate::logic::*;
use crate::epaxos;

impl WriteRequest {
    fn fromGrpc(req: epaxos::WriteRequest) -> Self {
        WriteRequest { key: req.key, value: req.value }
    }
}

//struct WriteRequest {
//    key: String,
//    value: i32,
//}
//
//#[derive(Clone)]
//struct WriteResponse {
//    committed: bool
//}
//
//#[derive(Clone)]
//struct ReadRequest {
//    key: String
//}
//
//#[derive(Clone)]
//struct ReadResponse {
//    value: i32
//}
//
//#[derive(Clone, Copy, PartialEq, Eq, Hash)]
//struct InstanceRef {
//    replica: ReplicaId,
//    slot: usize,
//}
//
//#[derive(Clone)]
//struct Payload {
//    command: WriteRequest,
//    // TODO maybe flatten into key/value
//    seq: usize,
//    dependencies: Vec<InstanceRef>,
//    instance: InstanceRef,
//}
//
//struct PreAccept(Payload);
//
//struct Accept(Payload);
//
//struct Commit(Payload);
//
//struct PreAcceptOK(Payload);
//
//#[derive(Clone)]
//struct AcceptOK {
//    command: WriteRequest,
//    instance: InstanceRef,
//}