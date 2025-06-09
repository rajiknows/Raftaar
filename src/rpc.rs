/*
 * Raft servers communicate using remote procedure calls
 * (RPCs), and the consensus algorithm requires only two
 * types of RPCs. RequestVote RPCs are initiated by candidates during elections (Section 5.2), and AppendEntriesRPCs
 * RPCs are initiated by leaders to replicate log entries andto
 * to provide a form of heartbeat (Section 5.3). Servers retry
 * RPCs if they do not receive a response in a timely manner,
 * and they issue RPCs in parallel for best performance.
*/

pub type currTerm = u16;

#[derive(Debug, Clone)]
pub struct AppendEntriesStruct {
    curr_term: currTerm,
    key: Option<String>,
    val: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RequestVoteStruct {
    curr_term: currTerm,
}

#[derive(Debug, Clone)]
pub enum Requests {
    RequestVote(RequestVoteStruct),
    AppendEntries(AppendEntriesStruct),
}
