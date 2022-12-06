use crate::hash::{default_hasher, default_hasher_with_seed, Hasher};
use crate::node::{Node, NodeType};
use crate::tree::Tree;

pub struct TreeOptions {
    pub max_width: usize,
    pub hasher: Option<Hasher>,
}

impl Default for TreeOptions {
    fn default() -> Self {
        TreeOptions {
            max_width: 2,
            hasher: None,
        }
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::default()
    }
}

// Creation methods
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
            None => default_hasher(),
        };

        let root = Node::new(0, NodeType::Root, option.max_width, None);
        // The starting of the tree capacity is the root + max_width children
        let mut nodes = Vec::with_capacity(1 + option.max_width);
        nodes.push(root);

        Self {
            height: 1,
            size: 1,
            capacity: option.max_width + 1,
            root: 0,
            max_width: option.max_width,
            hasher,
            nodes,
            last_parent: 0,
            last_leaf: 0,
            leaf_count: 0,
        }
    }
}
