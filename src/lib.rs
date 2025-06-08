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

pub trait RaftConsensus {
    fn elect_leader();
    fn replicate_log();
}

pub struct Raft {
    leader: SocketAddr,
    deciples: Vec<SocketAddr>,
}

impl RaftConsensus for Raft {
    fn elect_leader() {}
    fn replicate_log() {}
}
