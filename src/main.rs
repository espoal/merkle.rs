use merkle_rs::tree;

fn main() {
    println!("Hello, world!");
    let mut tree = tree::Tree::new();
    for i in 0..8 {
        let data = format!("data: {}", i);
        tree.insert(data);
    }

    println!("tree: {:?}", tree);
}
