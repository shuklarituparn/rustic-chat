use crate::message;
use futures_lite::StreamExt;
use iroh_gossip::net::{Event, GossipEvent, GossipReceiver};
use std::collections::HashMap;

pub async fn subscribe_loop(mut recieve: GossipReceiver) -> anyhow::Result<()> {
    let mut names = HashMap::new();
    while let Some(event) = recieve.try_next().await? {
        if let Event::Gossip(GossipEvent::Received(msg)) = event {
            let message = message::Message::from_bytes(&msg.content)?;
            match message {
                message::Message::AboutMe { node_id, name } => {
                    names.insert(node_id, name.clone());
                    println!("> {} is now known as {}", node_id.fmt_short(), name);
                }
                message::Message::Message { node_id, text } => {
                    let name = names
                        .get(&node_id)
                        .map_or_else(|| node_id.fmt_short(), String::to_string);
                    println!("{} : {}", name, text);
                }
            }
        }
    }
    Ok(())
}

pub fn input_loop(line_tx: tokio::sync::mpsc::Sender<String>) -> anyhow::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    loop {
        stdin.read_line(&mut buffer)?;
        line_tx.blocking_send(buffer.clone())?;
        buffer.clear();
    }
}
