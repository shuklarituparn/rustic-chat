use iroh::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Message {
    AboutMe { node_id: NodeId, name: String },
    Message { node_id: NodeId, text: String },
}

impl Message {
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&self).expect("Failed to serialize message")
    }
    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }
}
