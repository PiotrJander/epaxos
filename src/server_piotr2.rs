use std::cmp;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct ReplicaId(usize);

#[derive(Clone)]
struct WriteRequest {
    key: String,
    value: i32,
}

#[derive(Clone)]
struct WriteResponse {
    committed: bool
}

#[derive(Clone)]
struct ReadRequest {
    key: String
}

#[derive(Clone)]
struct ReadResponse {
    value: i32
}

#[derive(Clone)]
struct InstanceRef {
    replica: ReplicaId,
    slot: usize,
}

#[derive(Clone)]
struct Payload {
    command: WriteRequest,
    // TODO maybe flatten into key/value
    seq: usize,
    dependencies: Vec<InstanceRef>,
    instance: InstanceRef,
}

struct PreAccept(Payload);

struct Accept(Payload);

struct Commit(Payload);

struct PreAcceptOK(Payload);

#[derive(Clone)]
struct AcceptOK {
    command: WriteRequest,
    instance: InstanceRef,
}

enum CommandState {
    PreAccepted,
    Accepted,
    Committed,
}

struct Instance {
    key: String,
    value: i32,
    seq: usize,
    dependencies: Vec<InstanceRef>,
    state: CommandState,
}

type Commands = Vec<Vec<Instance>>;

struct Replica {
    id: ReplicaId,
    commands: Commands,
}

enum Path {
    Slow(Accept),
    Fast(WriteResponse, Commit),
}

const QUORUM: usize = 3;
const REPLICAS_NUM: usize = 5;

impl Replica {
    fn new(id: ReplicaId) -> Replica {
        let mut commands = Vec::new();
        for i in 0..REPLICAS_NUM {
            commands.push(Vec::new());
        }
        Replica { id, commands }
    }

    fn pre_accept_(&self, p: PreAccept) -> PreAcceptOK {
        unimplemented!()
    }

    fn accept_(&self, p: Accept) -> AcceptOK {
        unimplemented!()
    }

    fn commit_(&self, p: Commit) -> () {
        unimplemented!()
    }

//    fn fast_quorum(&self) -> Vec<ReplicaId> {
//        unimplemented!()
//    }

    fn establish_ordering_constraints1(&mut self, write_req: WriteRequest) -> PreAccept {
        let dependencies = self.find_interference(&write_req.key);
        let seq = self.find_next_seq(&dependencies);
        let instance_number = self.commands[self.id.0].len();
        self.commands[self.id.0].push(Instance {
            key: write_req.key.clone(),
            value: write_req.value,
            seq,
            dependencies: dependencies.clone(),
            state: CommandState::PreAccepted,
        });

        PreAccept(Payload {
            command: write_req,
            seq,
            dependencies,
            instance: InstanceRef {
                replica: self.id.clone(),
                slot: instance_number,
            },
        })
    }

    // assume that, unlike in the paper, the `dependencies` field of a PreAcceptOkayResponse
    // contains the difference, not the union, of instance references
    fn establish_ordering_constraints2(&mut self, mut pre_accept_request: PreAccept, mut pre_accept_ok_responses: [PreAcceptOK; QUORUM - 1]) -> Path {
        if pre_accept_ok_responses.iter().all(|response| response.0.seq == pre_accept_request.0.seq && response.0.dependencies.is_empty()) {
            self.commands[self.id.0][pre_accept_request.0.instance.slot].state = CommandState::Committed;
            Path::Fast(WriteResponse { committed: true }, Commit(pre_accept_request.0))
        } else {
            let max_seq = pre_accept_ok_responses.iter().map(|r| r.0.seq).max().unwrap_or(0);
            pre_accept_request.0.seq = std::cmp::max(max_seq, pre_accept_request.0.seq);
            for response in &mut pre_accept_ok_responses {
                pre_accept_request.0.dependencies.append(&mut response.0.dependencies);
            }
            // Paxos-Accept 1
            self.commands[self.id.0][pre_accept_request.0.instance.slot].state = CommandState::Accepted;
            Path::Slow(Accept(pre_accept_request.0))
        }
    }

    fn paxos_accept(&mut self, accept_req: Accept) -> (WriteResponse, Commit) {
        self.commands[self.id.0][accept_req.0.instance.slot].state = CommandState::Committed;
        (WriteResponse { committed: true }, Commit(accept_req.0))
    }

    fn write_(&self, p: WriteRequest) -> WriteResponse {
        unimplemented!()
    }

    fn read_(&self, p: ReadRequest) -> ReadResponse {
        unimplemented!()
    }

    // FIXME we only record write commands - is this okay?
    // FIXME is this correct?
    fn find_interference(&self, key: &String) -> Vec<InstanceRef> {
        let mut acc = Vec::new();
        for (q, row) in self.commands.iter().enumerate() {
            for (j, instance) in row.iter().enumerate() {
                if instance.key == *key {
                    acc.push(InstanceRef { replica: ReplicaId(q), slot: j })
                }
            }
        }
        acc
    }

    fn find_next_seq(&self, deps: &Vec<InstanceRef>) -> usize {
        let mut acc = 0;
        for dep in deps {
            let instance = &self.commands[dep.replica.0][dep.slot];
            acc = cmp::max(acc, instance.seq)
        }
        acc + 1
    }
}

fn main() {}








