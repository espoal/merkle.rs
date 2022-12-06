use crate::node::{Node, NodeId, NodeType};
use crate::tree::Tree;

impl Tree {
    pub(crate) fn grow_and_insert(&mut self, leaf: Node) -> NodeId {
        let mut leaf = leaf;
        let leaf_id = leaf.id;

        // Create tree_new root
        let new_root_id = self.nodes.len() as NodeId + 1;
        let mut new_root = Node::new_root(new_root_id, self.max_width);

        // Update old root
        let old_root_id = self.root;
        let old_root = self.get_node_mut(old_root_id).unwrap();
        old_root.node_type = NodeType::Internal;

        // Update relationships
        old_root.parent = Some(new_root_id);
        new_root.children = vec![old_root.id, leaf_id];
        leaf.parent = Some(new_root_id);

        // Update tree_new
        self.height += 1;
        let additional_capacity = self.max_width.pow(self.height as u32);
        self.nodes.reserve(additional_capacity);
        self.nodes.push(leaf);
        self.nodes.push(new_root);
        //self.size += 1;
        self.capacity *= self.max_width;
        self.root = new_root_id;
        self.last_parent = new_root_id;

        //let visited_nodes = vec![new_root_id, leaf_id];
        //self.update_hash(old_root_id);
        //self.update_hash(new_root_id);

        leaf_id
    }
}
