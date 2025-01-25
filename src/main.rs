mod args;
mod message;
mod ticket;
mod utils;

use crate::args::Args;
use crate::ticket::Ticket;
use crate::utils::{input_loop, subscribe_loop};
use anyhow::Result;
use clap::Parser;
use iroh::protocol::Router;
use iroh::{Endpoint,  RelayMode, SecretKey};
use iroh_gossip::net::{ Gossip};
use iroh_gossip::proto::TopicId;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::{ str::FromStr};

#[tokio::main]
async fn main() -> Result<()> {
    let args: args::Args = Args::parse();

    let (topic, peers) = match &args.command {
        args::Command::Open => {
            let topic = TopicId::from_bytes(rand::random());
            println!("> creating a chat room for topic {topic}");
            (topic, vec![])
        }
        args::Command::Join { ticket } => {
            let Ticket { topic, peers } = Ticket::from_str(ticket)?;
            println!("> joining the following chat room: {topic}");
            (topic, peers)
        }
    };

    let relay_mode = match args.no_relay {
        true => RelayMode::Disabled,
        false => RelayMode::Default,
    };
    let secret_key = SecretKey::generate(rand::rngs::OsRng);

    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .relay_mode(relay_mode)
        .bind_addr_v4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0))
        .bind()
        .await?;

    let gossip = Gossip::builder().spawn(endpoint.clone()).await?;
    let ticket = {
        let me = endpoint.node_addr().await?;
        let peers = peers.iter().cloned().chain([me]).collect();
        Ticket { topic, peers }
    };

    println!("> ticket to join us: {ticket}");

    let router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn()
        .await?;

    let peer_ids = peers.iter().map(|p| p.node_id).collect();
    if peers.is_empty() {
        println!("> no peers to join us");
    } else {
        println!("> connected to following {} peers...", peers.len());
        for peer in peers.into_iter() {
            endpoint.add_node_addr(peer)?;
        }
    };

    let (sender, receiver) = gossip.subscribe_and_join(topic, peer_ids).await?.split();
    println!("> connected");
    if let Some(name) = args.name {
        let message = message::Message::AboutMe {
            node_id: endpoint.node_id(),
            name,
        };
        let encoded_message = message.to_bytes();
        sender.broadcast(encoded_message.into()).await?;
    }

    tokio::spawn(subscribe_loop(receiver));

    let (line_tx, mut line_rx) = tokio::sync::mpsc::channel(1);
    std::thread::spawn(move || input_loop(line_tx));

    println!("> waiting for a message to become leader");
    while let Some(text) = line_rx.recv().await {
        let message = message::Message::Message {
            node_id: endpoint.node_id(),
            text: text.clone(),
        };
        let encoded_message = message.to_bytes();
        sender.broadcast(encoded_message.into()).await?;
        println!("> sent {}", text);
    }

    router.shutdown().await?;
    Ok(())
}
