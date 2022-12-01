use new_tree::tree;

fn main() {
    println!("Hello, world!");
    let mut tree = tree::Tree::new();
    let iters = 2000000;
    for i in 0..iters {
        let data = format!("data: {}", i);
        tree.insert(data);
    }

    println!("tree_new: {:?}", tree);
}
