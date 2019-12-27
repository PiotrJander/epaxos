use crate::logic::*;
use crate::epaxos;

impl WriteRequest {
    pub fn from_grpc(req: epaxos::WriteRequest) -> Self {
        WriteRequest { key: req.key, value: req.value }
    }

    pub fn to_grpc(&self) -> epaxos::WriteRequest {
        unimplemented!()
    }
}

impl WriteResponse {
    pub fn from_grpc(req: epaxos::ReadResponse) -> Self {
        unimplemented!()
    }

    pub fn to_grpc(&self) -> epaxos::WriteResponse {
        let mut resp = epaxos::WriteResponse::new();
        resp.commit = self.committed;
        resp
    }
}

impl ReadRequest {
    pub fn from_grpc(req: epaxos::ReadRequest) -> Self {
        unimplemented!()
    }

    pub fn to_grpc(&self) -> epaxos::ReadRequest {
        unimplemented!()
    }
}

impl ReadResponse {
    pub fn from_grpc(req: epaxos::ReadResponse) -> Self {
        unimplemented!()
    }

    pub fn to_grpc(&self) -> epaxos::ReadResponse {
        unimplemented!()
    }
}

impl InstanceRef {
    pub fn from_grpc(ref_: epaxos::Instance) -> Self {
        unimplemented!()
    }

    pub fn to_grpc(&self) -> epaxos::Instance {
        unimplemented!()
    }
}

impl Payload {
    pub fn from_grpc(p: epaxos::Payload) -> Self {
        unimplemented!()
    }

    pub fn to_grpc(&self) -> epaxos::Payload {
        unimplemented!()
    }
}

impl AcceptOK {
    pub fn from_grpc(resp: epaxos::AcceptOKPayload) -> Self {
        unimplemented!()
    }

    pub fn to_grpc(&self) -> epaxos::AcceptOKPayload {
        unimplemented!()
    }
}
