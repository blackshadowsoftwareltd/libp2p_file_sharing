use crate::enums::CliArgument;
use clap::Parser;
use libp2p::core::Multiaddr;

#[derive(Parser, Debug)]
#[clap(name = "libp2p file sharing example")]
pub struct Opt {
    /// Fixed value to generate deterministic peer ID.
    #[clap(long)]
    pub secret_key_seed: Option<u8>,

    #[clap(long)]
    pub peer: Option<Multiaddr>,

    #[clap(long)]
    pub listen_address: Option<Multiaddr>,

    #[clap(subcommand)]
    pub argument: CliArgument,
}
