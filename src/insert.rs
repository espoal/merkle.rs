use crate::node::{Node, NodeId, NodeType};
use crate::tree::Tree;

impl Tree {
    pub fn insert(&mut self, value: String) -> NodeId {
        self.insert_raw(value, true)
    }
    pub fn insert_batch(&mut self, values: Vec<String>) {
        let mut visited_leaves = vec![];

        for value in values {
            visited_leaves.push(self.insert_raw(value, false));
        }

        for leaf_id in visited_leaves {
            self.update_hashes(leaf_id);
        }
    }

    fn insert_raw(&mut self, value: String, should_update: bool) -> NodeId {
        let leaf_id = self.nodes.len() as NodeId;
        let leaf_hash = (self.hasher)(value.as_bytes());
        let mut leaf = Node::new_leaf(leaf_id, value, leaf_hash);
        leaf.children = vec![self.last_leaf];
        self.size += 1;
        self.leaf_count += 1;
        self.last_leaf = leaf_id;

        let mut visited_nodes = vec![leaf_id];

        // If capacity is not enough, grow the tree then insert the leaf
        // Could make the tree balanced by implementing a rotation here
        if self.size > self.capacity {
            self.grow_and_insert(leaf);
            visited_nodes.push(self.root);
            if should_update {
                self.update_node_hash(self.root);
            }
            return leaf_id;
        }

        let parent = self.get_node(self.last_parent).unwrap();
        // Simple case: there is still space in the last parent node
        if parent.children.len() < self.max_width {
            let parent = self.get_node_mut(self.last_parent).unwrap();
            parent.children.push(leaf_id);
            leaf.parent = Some(self.last_parent);
            self.nodes.push(leaf);
            if should_update {
                self.update_hashes(self.last_parent);
            }
            return leaf_id;
        }

        for (idx, child_id) in parent.children.iter().enumerate() {
            let child = self.get_node(*child_id).unwrap();
            if child.node_type == NodeType::Leaf {
                let new_internal_id = leaf_id + 1;
                let mut new_internal = Node::new_node(new_internal_id, self.max_width);
                new_internal.parent = Some(parent.id);

                let is_tail = idx == parent.children.len() - 1;

                if is_tail {
                    new_internal.children = vec![*child_id, leaf_id];
                } else {
                    new_internal.children = parent.children.clone();
                }

                leaf.parent = Some(new_internal_id);

                let child = self.get_node_mut(*child_id).unwrap();
                child.parent = Some(new_internal_id);

                self.nodes.push(leaf);
                self.nodes.push(new_internal);

                let parent = self.get_node_mut(self.last_parent).unwrap();
                if is_tail {
                    parent.children[idx] = new_internal_id;
                } else {
                    parent.children = vec![new_internal_id, leaf_id];
                }

                self.last_parent = if is_tail { new_internal_id } else { parent.id };

                self.update_node_hash(self.last_parent);

                if should_update {
                    self.update_hashes(leaf_id);
                }

                return leaf_id;
            }
        }

        panic!("Should not reach here");
    }
}
