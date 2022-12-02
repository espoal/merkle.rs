use new_tree::tree;

fn main() {
    println!("Hello, world!");
    let mut tree = tree::Tree::new();
    let iters = 2;
    for i in 0..iters {
        let data = format!("data: {}", i);
        tree.insert(data);
    }

    println!("tree_new: {:?}", tree);

    let material = tree.get_proof_material(2).unwrap();
    println!("Proof material: {:?}", material);
}
