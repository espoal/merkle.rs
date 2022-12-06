use crate::hash::{HashBuff, Hasher};
use crate::node::{Node, NodeId, NodeType};
use std::fmt::Debug;

pub struct Tree {
    pub root: NodeId,
    pub last_parent: NodeId,
    pub last_leaf: NodeId,
    pub nodes: Vec<Node>,
    pub max_width: usize,
    pub height: usize,
    pub capacity: usize,
    pub size: usize,
    pub leaf_count: usize,
    pub hasher: Hasher,
}

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tree")
            .field("height", &self.height)
            .field("nodes", &self.nodes)
            .finish()
    }
}

// Utility methods
impl Tree {
    // Getters allow us to abstract away the internal representation of the tree
    pub fn get_root(&self) -> &Node {
        self.nodes.get(self.root as usize).unwrap()
    }
    pub fn get_root_mut(&mut self) -> &mut Node {
        self.nodes.get_mut(self.root as usize).unwrap()
    }
    pub fn get_node(&self, index: NodeId) -> Option<&Node> {
        self.nodes.get(index as usize)
    }
    pub fn get_node_mut(&mut self, index: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(index as usize)
    }
    pub fn find_leaf(&self, value: &str) -> Option<Node> {
        let mut node = self.get_node(self.last_leaf)?;

        loop {
            if node.node_type != NodeType::Leaf {
                return None;
            }

            let node_value = node.value.as_ref()?;
            if node_value == value {
                return Some(node.clone());
            }

            // In the leaves we store 1 children as the pointer to the previous leaf
            node = self.get_node(node.children[0])?;
        }
    }

    pub fn get_root_path(&self, node_id: NodeId) -> Vec<NodeId> {
        let mut path = Vec::with_capacity(self.height);
        let mut node = self.get_node(node_id).unwrap();
        loop {
            path.push(node.id);
            if node.node_type == NodeType::Root {
                break;
            }
            node = self.get_node(node.parent.unwrap()).unwrap();
        }
        path
    }

    pub fn get_opening(&self, node_id: NodeId) -> Vec<Vec<HashBuff>> {
        let root_path = self.get_root_path(node_id);

        let mut openings = Vec::with_capacity(self.height);

        for node_id in root_path {
            let node = self.get_node(node_id).unwrap();
            if node.node_type == NodeType::Leaf {
                continue;
            }
            let mut opening = Vec::with_capacity(node.children.len());
            for child_id in node.children.iter() {
                let child = self.get_node(*child_id).unwrap();
                opening.push(child.hash);
            }
            openings.push(opening);
        }

        openings
    }
}
