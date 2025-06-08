use std::os::unix::net::SocketAddr;

/// 2025 8 june
///
/// This is a toy project of mine that may or may not be used in production in my upcoming project
///
/// RAFT - the distributed systems consensus algorithm
///
/// during this whole implementation i will not use LLMs to code any part of the thing
///
/// i will but refer to the following papers :
///
/// 1. https://www.usenix.org/conference/atc14/technical-sessions/presentation/ongaro
///
///
mod rpc;
/// A Raft cluster contains several servers; five is a typical
/// number, which allows the system to tolerate two failures.
/// At any given time each server is in one of three states:
/// leader, follower, or candidate.
pub trait RaftConsensus {
    fn elect_leader();
    fn replicate_log();
}

pub struct Cluster {
    ///In normal operation there is exactly one leader and
    /// all of the other servers are followers.
    leader: Node,

    /// Followers are passive: they issue no requests on their own but
    /// simply respond to requests from leaders and candidates.
    followers: Vec<Node>,
}

impl RaftConsensus for Cluster {
    fn elect_leader() {}
    fn replicate_log() {}
}

//Raft divides time into terms of arbitrary length, as
//shown in Figure 5. Terms are numbered with consecutive
//integers. Each term begins with an election, in which one
//or more candidates attempt to become leader

// this will tell the servers which term it is currently , who is the leader etc
pub struct Term {
    stale_leaders: Vec<Node>,
}

// Node represents a general node in the cluster , it can be leader or follower
pub struct Node {
    addr: SocketAddr,
    curr_term_no: u16,
}
