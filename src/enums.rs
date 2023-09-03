use crate::rec_res::{FileRequest, FileResponse};
use clap::Parser;
use libp2p::{
    kad::{record::store::MemoryStore, Kademlia, KademliaEvent},
    request_response::{self, ResponseChannel},
    swarm::NetworkBehaviour,
};
use std::path::PathBuf;

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "ComposedEvent")]
pub struct ComposedBehaviour {
    pub request_response: request_response::cbor::Behaviour<FileRequest, FileResponse>,
    pub kademlia: Kademlia<MemoryStore>,
}

#[derive(Debug, Parser)]
pub enum CliArgument {
    Provide {
        #[clap(long)]
        path: PathBuf,
        #[clap(long)]
        name: String,
    },
    Get {
        #[clap(long)]
        name: String,
    },
}
#[derive(Debug)]
pub enum ComposedEvent {
    RequestResponse(request_response::Event<FileRequest, FileResponse>),
    Kademlia(KademliaEvent),
}

impl From<request_response::Event<FileRequest, FileResponse>> for ComposedEvent {
    fn from(event: request_response::Event<FileRequest, FileResponse>) -> Self {
        ComposedEvent::RequestResponse(event)
    }
}

impl From<KademliaEvent> for ComposedEvent {
    fn from(event: KademliaEvent) -> Self {
        ComposedEvent::Kademlia(event)
    }
}
#[derive(Debug)]
pub(crate) enum Event {
    InboundRequest {
        request: String,
        channel: ResponseChannel<FileResponse>,
    },
}
