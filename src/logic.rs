use std::cmp;
use std::collections::HashMap;

// FIXME maybe delete
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    command: WriteRequest,
    seq: usize,
    dependencies: Vec<InstanceRef>,
    state: CommandState,
}

// TODO use HashMap instead
struct Commands(HashMap<InstanceRef, Instance>);

impl Commands {
    fn new() -> Self {
        Commands(HashMap::new())
    }
}

impl std::ops::Index<InstanceRef> for Commands {

    type Output = Instance;

    fn index(&self, index: InstanceRef) -> &Self::Output {
        &self.0[&index]
    }
}

impl std::ops::IndexMut<InstanceRef> for Commands {

    // unsafe: requires that the key is present
    fn index_mut(&mut self, index: InstanceRef) -> &mut Self::Output {
        self.0.get_mut(&index).unwrap()
    }
}

struct Replica {
    id: ReplicaId,
    commands: Commands,
    instance_number: usize,
}

enum Path {
    Slow(Accept),
    Fast(WriteResponse, Commit),
}

const QUORUM: usize = 3;
const REPLICAS_NUM: usize = 5;

impl Replica {
    fn new(id: ReplicaId) -> Replica {
        Replica { id, commands: Commands::new(), instance_number: 0 }
    }

    fn pre_accept_(&mut self, pre_accept_req: PreAccept) -> PreAcceptOK {
        let Payload { command, seq, mut dependencies, instance } =
            pre_accept_req.0;
        let dependencies1 = self.find_interference(&command.key);
        let seq_candidate = self.find_next_seq(&dependencies1);
        let seq1 = cmp::max(seq, seq_candidate);
        let mut dep_diff = Vec::new();
        for dep in dependencies1 {
            // FIXME contains inefficient for a vector, but it doesn't matter
            if !dependencies.contains(&dep) {
                dep_diff.push(dep);
            }
        }

        // FIXME use some kind of idiom here
        // find the union
        for dep in &dep_diff {
            dependencies.push(*dep)
        }

        // unlike in the paper, PreAcceptOK contains the difference, not the union
        self.commands.0.insert(instance, Instance {
            command: command.clone(),
            seq: seq1,
            dependencies,
            state: CommandState::PreAccepted
        });

        PreAcceptOK(Payload {
            command,
            seq: seq1,
            dependencies: dep_diff,
            instance
        })
    }

    fn accept_(&mut self, accept_req: Accept) -> AcceptOK {
        let Payload { command, seq, mut dependencies, instance } = accept_req.0;
        self.commands[instance] = Instance {
            command: command.clone(),
            seq,
            dependencies,
            state: CommandState::Accepted
        };
        AcceptOK {
            command: command.clone(),
            instance
        }
    }

    fn commit_(&mut self, commit_req: Commit) -> () {
        let Payload { command, seq, mut dependencies, instance } = commit_req.0;
        self.commands[instance] = Instance {
            command: command.clone(),
            seq,
            dependencies,
            state: CommandState::Accepted
        };
    }

    //    fn fast_quorum(&self) -> Vec<ReplicaId> {
    //        unimplemented!()
    //    }

    fn current_instance_ref(&self) -> InstanceRef {
        InstanceRef {
            replica: self.id,
            slot: self.instance_number,
        }
    }

    fn write1(&mut self, write_req: WriteRequest) -> PreAccept {

        let dependencies = self.find_interference(&write_req.key);
        let seq = self.find_next_seq(&dependencies);
        self.instance_number += 1;
        self.commands.0.insert(self.current_instance_ref(), Instance {
            command: write_req.clone(),
            seq,
            dependencies: dependencies.clone(),
            state: CommandState::PreAccepted
        });

        PreAccept(Payload {
            command: write_req,
            seq,
            dependencies,
            instance: InstanceRef {
                replica: self.id,
                slot: self.instance_number,
            },
        })
    }

    // assume that, unlike in the paper, the `dependencies` field of a PreAcceptOkayResponse
    // contains the difference, not the union, of instance references
    fn write2(
        &mut self,
        mut pre_accept_req: PreAccept,
        mut pre_accept_resps: [PreAcceptOK; QUORUM - 1],
    ) -> Path {
        let fast_path = pre_accept_resps.iter().all(|response| {
            response.0.seq == pre_accept_req.0.seq && response.0.dependencies.is_empty()
        });
        if fast_path {
            // no need to update seq or dependencies
            self.commands[pre_accept_req.0.instance].state = CommandState::Committed;
            Path::Fast(WriteResponse { committed: true }, Commit(pre_accept_req.0))
        } else {
            let max_seq =
                pre_accept_resps.iter().map(|r| r.0.seq).max().unwrap_or(0);
            pre_accept_req.0.seq = std::cmp::max(max_seq, pre_accept_req.0.seq);
            for response in &mut pre_accept_resps {
                pre_accept_req.0.dependencies.append(&mut response.0.dependencies);
            }
            // Paxos-Accept 1
            self.commands[pre_accept_req.0.instance].state = CommandState::Accepted;
            Path::Slow(Accept(pre_accept_req.0))
        }
    }

    fn leader_commit(&mut self, payload: Payload) -> (WriteResponse, Commit) {
        let Payload { command, seq, mut dependencies, instance } = payload;
        self.commands[payload.instance] = Instance {
            command: command.clone(),
            seq,
            dependencies: dependencies.clone(),
            state: CommandState::Committed
        };
        (WriteResponse { committed: true }, Commit(Payload { command, seq, dependencies, instance } ))
    }

    fn read_(&self, p: ReadRequest) -> ReadResponse {
        unimplemented!()
    }

    // FIXME we only record write commands - is this okay?
    // FIXME is this correct?
    fn find_interference(&self, key: &String) -> Vec<InstanceRef> {
        let mut acc: Vec<InstanceRef> = Vec::new();
        for (ref_, instance) in &self.commands.0 {
            if instance.command.key == *key {
                acc.push(*ref_);
            }
        }
        acc
    }

    fn find_next_seq(&self, deps: &Vec<InstanceRef>) -> usize {
        let mut acc = 0;
        for dep in deps {
            let instance = &self.commands[*dep];
            acc = cmp::max(acc, instance.seq)
        }
        acc + 1
    }
}

fn main() {}







