use new_tree::tree::Tree;

fn main() {
    println!("Hello, world!");

    let mut tree = Tree::new();
    let iters = 2;
    for i in 0..iters {
        let data = format!("data: {}", i);
        tree.insert(data);
    }

    let result = tree.verify_tree(tree.root);
    println!("result: {:?}", result);
}
