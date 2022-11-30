use crate::hash::{default_hasher, default_hasher_with_seed};
use crate::node::{Node, NodeId, NodeType};
use std::fmt::Debug;
use std::os::unix::process::parent_id;

pub type Hasher = fn(&str) -> u64;

pub struct Tree {
    height: usize,
    max_width: usize,
    root: NodeId,
    hasher: Hasher,
    nodes: Vec<Node>,
    size: usize,
    leaf_size: usize,
    last_leaf: NodeId,
}

pub struct TreeOptions {
    max_width: usize,
    hasher: Option<Hasher>,
}

impl Default for TreeOptions {
    fn default() -> Self {
        TreeOptions {
            max_width: 2,
            hasher: None,
        }
    }
}

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tree")
            .field("height", &self.height)
            .field("nodes", &self.nodes)
            .finish()
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::default()
    }
}

impl Tree {
    pub fn new() -> Self {
        let default_opts = TreeOptions {
            max_width: TreeOptions::default().max_width,
            hasher: None,
        };

        Self::new_with_opts(default_opts)
    }

    pub fn new_with_seed(seed: u64) -> Self {
        let hasher = default_hasher_with_seed(seed);

        let default_opts = TreeOptions {
            //seed: Some(seed),
            hasher: Some(hasher),
            max_width: TreeOptions::default().max_width,
        };

        Self::new_with_opts(default_opts)
    }

    pub fn new_with_hasher(hasher: Hasher) -> Self {
        let default_opts = TreeOptions {
            //seed: None,
            hasher: Some(hasher),
            max_width: TreeOptions::default().max_width,
        };

        Self::new_with_opts(default_opts)
    }

    pub fn new_with_opts(option: TreeOptions) -> Self {
        let hasher = match option.hasher {
            Some(hasher) => hasher,
            None => default_hasher,
        };

        let root = Node::new(NodeType::Root, 0, None);
        let nodes = vec![root];

        Self {
            height: 0,
            size: 0,
            leaf_size: 0,
            root: 0,
            max_width: option.max_width,
            hasher,
            nodes,
            last_leaf: 0,
        }
    }
}

impl Tree {
    pub fn get_root(&self) -> &Node {
        self.nodes.get(self.root as usize).unwrap()
    }
    pub fn get_mut_root(&mut self) -> &mut Node {
        self.nodes.get_mut(self.root as usize).unwrap()
    }
    pub fn get_value(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }
    pub fn get_mut_value(&mut self, index: usize) -> Option<&mut Node> {
        self.nodes.get_mut(index)
    }

    pub fn insert(&mut self, data: String) -> (NodeId, Vec<NodeId>) {
        let leaf_id = self.nodes.len() as NodeId;
        let mut leaf = Node::new(NodeType::Leaf, leaf_id, Some(data.to_string()));
        leaf.hash = (self.hasher)(&data).to_be_bytes();
        self.leaf_size += 1;

        // Optimization: if the tree is full, resize it first
        let real_capacity = usize::pow(self.max_width, self.height as u32 + 1);
        if self.leaf_size == real_capacity + 1 {
            // Create new root
            let new_root_id = leaf_id + 1;
            let mut new_root = Node::new(NodeType::Root, new_root_id, None);

            // Update old root
            let old_root = self.get_mut_root();
            old_root.node_type = NodeType::Internal;

            // Update relationships
            old_root.parent = Some(new_root_id);
            new_root.children = vec![old_root.id, leaf_id];
            leaf.parent = Some(new_root_id);

            // Update tree
            self.nodes.push(leaf);
            self.nodes.push(new_root);
            self.size += 2;
            self.root = new_root_id;
            self.height += 1;
            self.last_leaf = leaf_id;

            let visited_nodes = vec![new_root_id, leaf_id];
            return (leaf_id, visited_nodes);
        }

        let last_leaf = self.get_value(self.last_leaf as usize).unwrap();
        let mut parent: &Node;
        if last_leaf.node_type == NodeType::Root {
            parent = last_leaf;
        } else {
            parent = self.get_value(last_leaf.parent.unwrap() as usize).unwrap();
        }

        if parent.children.len() < self.max_width {
            let parent_id = parent.id;
            leaf.parent = Some(parent.id as NodeId);
            self.nodes.push(leaf);
            self.last_leaf = leaf_id;
            self.size += 1;
            let parent = self.get_mut_value(parent_id as usize).unwrap();
            parent.children.push(leaf_id);
            return (leaf_id, vec![parent_id, leaf_id]);
        }

        let mut parent_id = -1;
        let mut child_in_parent = -1;
        let mut sibling_id = -1;

        for (idx, child_id) in parent.children.iter().enumerate() {
            let child = self.get_value(*child_id as usize).unwrap();
            if child.node_type == NodeType::Leaf {
                parent_id = parent.id as i32;
                child_in_parent = idx as i32;
                sibling_id = *child_id as i32;

                break;
            }
        }

        let new_internal_id = self.nodes.len() as NodeId + 1;
        leaf.parent = Some(new_internal_id as NodeId);
        self.last_leaf = leaf_id;
        self.nodes.push(leaf);
        let mut new_internal_node = Node::new(NodeType::Internal, new_internal_id, None);
        new_internal_node.parent = Some(parent_id as NodeId);
        new_internal_node.children = vec![sibling_id as NodeId, leaf_id];
        let mut sibling = self.get_mut_value(sibling_id as usize).unwrap();
        sibling.parent = Some(new_internal_id as NodeId);
        self.nodes.push(new_internal_node);
        let mut parent = self.get_mut_value(parent_id as usize).unwrap();
        parent.children[child_in_parent as usize] = new_internal_id as NodeId;
        self.size += 2;

        (leaf_id, vec![])
    }
    pub fn insert_batch(&mut self, data: &[&str]) -> Vec<usize> {
        vec![]
    }

    pub fn get_proof_material(&self, index: usize) -> Option<Vec<&Node>> {
        None
    }

    fn grow_tree(&mut self) {}
}
