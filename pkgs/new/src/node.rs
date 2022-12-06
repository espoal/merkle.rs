use crate::hash::{HashBuff, HASH_SIZE};

pub type NodeId = u64;

#[derive(Debug, PartialEq, Eq)]
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
    pub fn new(id: NodeId, node_type: NodeType, max_width: usize, value: Option<String>) -> Self {
        Self {
            id,
            node_type,
            parent: None,
            children: Vec::with_capacity(max_width),
            value,
            hash: [0; HASH_SIZE],
        }
    }
    pub fn new_root(id: NodeId, max_width: usize) -> Self {
        Self {
            id,
            node_type: NodeType::Root,
            parent: None,
            children: Vec::with_capacity(max_width),
            value: None,
            hash: [0; HASH_SIZE],
        }
    }
    pub fn new_node(id: NodeId, max_width: usize) -> Self {
        Self {
            id,
            node_type: NodeType::Internal,
            parent: None,
            children: Vec::with_capacity(max_width),
            value: None,
            hash: [0; HASH_SIZE],
        }
    }
    pub fn new_leaf(id: NodeId, value: String, hash: HashBuff) -> Self {
        Self {
            id,
            node_type: NodeType::Leaf,
            parent: None,
            children: Vec::with_capacity(1),
            value: Some(value),
            hash,
        }
    }
}
