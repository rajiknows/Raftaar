/*
 * Raft servers communicate using remote procedure calls
 * (RPCs), and the consensus algorithm requires only two
 * types of RPCs. RequestVote RPCs are initiated by candidates during elections (Section 5.2), and AppendEntriesRPCs
 * RPCs are initiated by leaders to replicate log entries andto
 * to provide a form of heartbeat (Section 5.3). Servers retry
 * RPCs if they do not receive a response in a timely manner,
 * and they issue RPCs in parallel for best performance.
*/

use crate::proto::raft_server::Raft;

#[derive(Debug, Clone)]
pub struct RaftNode {}

impl Raft for RaftNode {
    fn append_entries<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<crate::proto::AppendEntriesRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = std::result::Result<
                        tonic::Response<crate::proto::AppendEntriesResponse>,
                        tonic::Status,
                    >,
                > + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
    }
    fn request_vote<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<crate::proto::RequestVoteRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = std::result::Result<
                        tonic::Response<crate::proto::RequestVoteResponse>,
                        tonic::Status,
                    >,
                > + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
    }
}
