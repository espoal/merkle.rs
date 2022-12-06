use mtree::tree::Tree;

fn main() {
    println!("Hello, world!");

    let mut tree = Tree::new();
    let iters = 40000;
    /*for i in 0..iters {
        let data = format!("data: {}", i);
        tree.insert(data);
    }*/
    let mut data = Vec::with_capacity(iters);
    for i in 0..iters {
        data.push(format!("data: {}", i));
    }
    tree.insert_batch(data);

    let result = tree.verify_tree(tree.root);
    println!("result: {:?}", result);

    let leaf = tree.find_leaf("data: 200").unwrap();
    println!("leaf: {:?}", leaf);
}
