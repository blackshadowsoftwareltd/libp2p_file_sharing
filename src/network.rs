// use futures::channel::mpsc;
// use futures::prelude::*;

// use libp2p::{
//     identity,
//     kad::{self, record::store::MemoryStore, Kademlia},
//     noise,
//     request_response::{self, ProtocolSupport},
//     swarm::SwarmBuilder,
//     tcp, yamux, Transport,
// };

// use libp2p::core::upgrade::Version;
// use libp2p::StreamProtocol;

// use std::error::Error;

// use crate::enums::{ComposedBehaviour, Event};
// use crate::event_loop::EventLoop;
// use crate::models::Client;

// /// Creates the network components, namely:
// ///
// /// - The network client to interact with the network layer from anywhere
// ///   within your application.
// ///
// /// - The network event stream, e.g. for incoming requests.
// ///
// /// - The network task driving the network itself.
// pub(crate) async fn new(
//     secret_key_seed: Option<u8>,
// ) -> Result<(Client, impl Stream<Item = Event>, EventLoop), Box<dyn Error>> {
//     // Create a public/private key pair, either random or based on a seed.
//     let id_keys = match secret_key_seed {
//         Some(seed) => {
//             let mut bytes = [0u8; 32];
//             bytes[0] = seed;
//             identity::Keypair::ed25519_from_bytes(bytes).unwrap()
//         }
//         None => identity::Keypair::generate_ed25519(),
//     };
//     let peer_id = id_keys.public().to_peer_id();

//     let transport = tcp::async_io::Transport::default()
//         .upgrade(Version::V1Lazy)
//         .authenticate(noise::Config::new(&id_keys)?)
//         .multiplex(yamux::Config::default())
//         .boxed();

//     // Build the Swarm, connecting the lower layer transport logic with the
//     // higher layer network behaviour logic.
//     let mut swarm = SwarmBuilder::with_async_std_executor(
//         transport,
//         ComposedBehaviour {
//             kademlia: Kademlia::new(peer_id, MemoryStore::new(peer_id)),
//             request_response: request_response::cbor::Behaviour::new(
//                 [(
//                     StreamProtocol::new("/file-exchange/1"),
//                     ProtocolSupport::Full,
//                 )],
//                 request_response::Config::default(),
//             ),
//         },
//         peer_id,
//     )
//     .build();

//     swarm
//         .behaviour_mut()
//         .kademlia
//         .set_mode(Some(kad::Mode::Server));

//     let (command_sender, command_receiver) = mpsc::channel(0);
//     let (event_sender, event_receiver) = mpsc::channel(0);

//     Ok((
//         Client {
//             sender: command_sender,
//         },
//         event_receiver,
//         EventLoop::new(swarm, command_receiver, event_sender),
//     ))
// }
