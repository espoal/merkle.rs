use crate::node::{NodeId, NodeType};
use crate::tree::Tree;
use xxhash_rust::xxh3::{xxh3_64, xxh3_64_with_seed};

pub const HASH_SIZE: usize = 8;
pub type HashBuff = [u8; HASH_SIZE];

pub type Hasher = Box<dyn Fn(&[u8]) -> HashBuff>;

pub fn default_hasher() -> Hasher {
    Box::new(|data: &[u8]| -> HashBuff {
        return xxh3_64(data).to_be_bytes();
    })
}

pub fn default_hasher_with_seed(seed: u64) -> Hasher {
    Box::new(move |data: &[u8]| -> HashBuff {
        return xxh3_64_with_seed(data, seed).to_be_bytes();
    })
}

impl Tree {
    pub fn update_hashes(&mut self, index: NodeId) {
        let path = self.get_root_path(index);

        for node_id in path {
            self.update_node_hash(node_id);
        }
    }

    pub fn update_node_hash(&mut self, index: NodeId) {
        let mut node = self.get_node(index).unwrap();
        match node.node_type {
            NodeType::Leaf => {
                //let mut buff: Vec<u8> = Vec::with_capacity(self.max_width);
                //buff.append(&mut node.value.unwrap().as_bytes().to_vec());
                //node.hash = (self.hasher)(&buff);
            }
            NodeType::Root | NodeType::Internal => {
                let mut buff: Vec<u8> = Vec::with_capacity(self.max_width);
                for child_id in node.children.iter() {
                    let child = self.get_node(*child_id).unwrap();
                    buff.append(&mut child.hash.to_vec());
                }
                let hash = (self.hasher)(&buff);
                let node = self.get_node_mut(index).unwrap();
                node.hash = hash;
            }
        }
    }

    pub fn verify_node(&self, index: NodeId) -> Option<()> {
        let mut node = self.get_node(index)?;
        match node.node_type {
            NodeType::Leaf => {
                let mut buff: Vec<u8> = node.value.as_ref()?.as_bytes().to_vec();
                let hash = (self.hasher)(&buff);
                if hash == node.hash {
                    return Some(());
                } else {
                    return None;
                }
            }
            NodeType::Root | NodeType::Internal => {
                let mut buff: Vec<u8> = Vec::with_capacity(self.max_width);
                for child_id in node.children.iter() {
                    let child = self.get_node(*child_id)?;
                    buff.append(&mut child.hash.to_vec());
                }
                let hash = (self.hasher)(&buff);
                if hash == node.hash {
                    return Some(());
                } else {
                    return None;
                }
            }
        }
    }

    pub fn verify_tree(&self, index: NodeId) -> Option<()> {
        let node = self.get_node(index)?;

        self.verify_node(node.id)?;

        if node.node_type != NodeType::Leaf {
            for child_id in node.children.iter() {
                self.verify_tree(*child_id)?;
            }
        }

        Some(())
    }
}
