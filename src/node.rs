use crate::hash::{HashBuff, HASH_SIZE};

pub type NodeId = u64;
#[derive(Debug, PartialEq)]
pub enum NodeType {
    Root,
    Internal,
    Leaf,
}

#[derive(Debug)]
pub struct Node {
    pub id: NodeId,
    pub node_type: NodeType,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub value: Option<String>,
    pub hash: HashBuff,
}

impl Node {
    pub fn new(node_type: NodeType, id: NodeId, value: Option<String>) -> Self {
        Self {
            node_type,
            id,
            parent: None,
            children: Vec::new(),
            value,
            hash: [0; HASH_SIZE],
        }
    }
}
