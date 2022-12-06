use new_tree::tree;
use xxhash_rust::xxh3::xxh3_64;

fn main() {
    let data: Vec<u8> = vec![
        25, 157, 69, 71, 75, 202, 234, 61, 191, 103, 196, 111, 216, 29, 139, 85,
    ];
    let test = xxh3_64(&data).to_be_bytes();
    println!("{:?}", test);

    println!("Hello, world!");
    let mut tree = tree::Tree::new();
    let iters = 20000;
    for i in 0..iters {
        let data = format!("data: {}", i);
        tree.insert(data);
    }

    //let pm = tree.get_proof_material(2).unwrap();
    //println!("pm: {:?}", pm);

    tree.update_by_leaves();
    let root_id = tree.root;
    let result = tree.verify_node(root_id, true);
    println!("result: {:?}", result);

    let material = tree.get_proof_material(2).unwrap();
    println!("Proof material: {:?}", material);
}
