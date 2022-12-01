use crate::hash::{default_hasher, Hasher};
use crate::new::TreeOptions;
use crate::node::{Node, NodeId, NodeType};
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

pub struct Tree {
    height: usize,
    max_width: usize,
    root: NodeId,
    hasher: Hasher,
    nodes: Vec<Node>,
    size: usize,
    capacity: usize,
    leaf_count: usize,
    last_parent: NodeId,
}

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tree")
            .field("height", &self.height)
            .field("nodes", &self.nodes)
            .finish()
    }
}

impl Tree {
    pub fn new_with_opts(option: TreeOptions) -> Self {
        let hasher = match option.hasher {
            Some(hasher) => hasher,
            None => default_hasher,
        };

        let root = Node::new(NodeType::Root, 0, None);
        let mut nodes = Vec::with_capacity(3);
        nodes.push(root);

        Self {
            height: 1,
            size: 0,
            capacity: 2,
            leaf_count: 0,
            root: 0,
            max_width: option.max_width,
            hasher,
            nodes,
            last_parent: 0,
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
    pub fn get(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.nodes.get_mut(index)
    }
    pub fn get_by_value(&self, value: &str) -> Option<NodeId> {
        None
    }

    pub fn insert(&mut self, value: String) -> NodeId {
        let leaf_id = self.nodes.len() as NodeId;
        let mut leaf = Node::new(NodeType::Leaf, leaf_id, Some(value.to_string()));
        leaf.hash = (self.hasher)(&value);
        self.size += 1;

        // If capacity is not enough, grow the tree then insert the leaf
        // Could make the tree balanced by implementing a rotation here
        if self.size == self.capacity + 1 {
            self.grow_and_insert(leaf);
            return leaf_id;
        }

        let parent = self.get(self.last_parent as usize).unwrap();

        // Simple case: there is still space in the last parent node
        if parent.children.len() < self.max_width {
            let parent = self.get_mut(self.last_parent as usize).unwrap();
            parent.children.push(leaf_id);
            leaf.parent = Some(self.last_parent);
            self.nodes.push(leaf);

            return leaf_id;
        }

        // Hard case, we need to create a new internal node before inserting
        for (idx, child_id) in parent.children.iter().enumerate() {
            let child = self.get(*child_id as usize).unwrap();
            if child.node_type == NodeType::Leaf {
                let new_internal_id = leaf_id + 1;
                let mut new_internal = Node::new(NodeType::Internal, new_internal_id, None);
                new_internal.parent = Some(parent.id);

                let is_tail = idx >= parent.children.len() - 1;
                let last_parent = if is_tail { new_internal_id } else { parent.id };

                if is_tail {
                    new_internal.children = vec![*child_id, leaf_id];
                } else {
                    new_internal.children = parent.children.clone();
                }

                let child = self.get_mut(*child_id as usize).unwrap();
                child.parent = Some(new_internal_id);
                leaf.parent = Some(new_internal_id);

                self.nodes.push(leaf);
                self.nodes.push(new_internal);

                let parent = self.get_mut(self.last_parent as usize).unwrap();
                if is_tail {
                    parent.children[idx] = new_internal_id;
                } else {
                    parent.children = vec![new_internal_id, leaf_id];
                }

                //let free_parent = self.find_free_parent().unwrap();
                self.last_parent = last_parent;

                //println!("free_parent: {:?}", free_parent);
                //println!("last_parent: {:?}", last_parent);

                return leaf_id;
            }
        }

        panic!("Should not reach here");

        0 as NodeId
    }

    pub fn get_proof_material(&self, index: usize) -> Option<Vec<&Node>> {
        None
    }

    fn grow_and_insert(&mut self, leaf: Node) -> NodeId {
        let mut leaf = leaf;
        let leaf_id = leaf.id;

        // Create tree_new root
        let new_root_id = self.nodes.len() as NodeId + 1;
        let mut new_root = Node::new(NodeType::Root, new_root_id, None);

        // Update old root
        let old_root = self.get_mut_root();
        old_root.node_type = NodeType::Internal;

        // Update relationships
        old_root.parent = Some(new_root_id);
        new_root.children = vec![old_root.id, leaf_id];
        leaf.parent = Some(new_root_id);

        // Update tree_new
        // TODO: Resize array
        self.nodes.push(leaf);
        self.nodes.push(new_root);
        //self.size += 1;
        self.capacity += self.capacity;
        self.root = new_root_id;
        self.height += 1;
        self.last_parent = new_root_id;

        //let visited_nodes = vec![new_root_id, leaf_id];

        leaf_id
    }

    // Ugly width first search to find a free parent
    // Used only for debugging
    // Could make it efficient by using recursion
    fn find_free_parent(&self) -> Option<NodeId> {
        let mut parent = self.get(self.root as usize).unwrap();
        let mut children = Vec::from(parent.children.clone());

        loop {
            let mut new_children: Vec<NodeId> = Vec::with_capacity(children.len() * self.max_width);
            for child_id in children.iter() {
                let child = self.get(*child_id as usize).unwrap();
                if child.node_type == NodeType::Leaf {
                    return Some(child.parent.unwrap());
                }
                new_children.extend(child.children.clone());
            }
            children = new_children;
        }

        None
    }

    // TODO: to be called in grow_and_insert to rebalance the tree
    fn rotate(&mut self) {}
}
