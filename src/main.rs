use mtree::tree::Tree;

fn main() {
    println!("Hello, world!");

    let mut tree = Tree::new();
    let iters = 8;
    for i in 0..iters {
        let data = format!("data: {}", i);
        tree.insert(data);
    }

    let result = tree.verify_tree(tree.root);
    println!("result: {:?}", result);

    let leaf = tree.find_leaf("data: 4").unwrap();
    println!("leaf: {:?}", leaf);

    let openings = tree.get_opening(leaf.id);
    println!("openings: {:?}", openings);
}
