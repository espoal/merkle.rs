# merkle.rs
An example of a merkle b-tree in Rust

The tree has variable branching factor, so it's a b-tree.
The tree is not balanced, but searches are `log(n)+log(2)`, 
so the `O(log(n))` asymptotic limit is respected, and it 
allows for some optimizations:
- recent transactions are cheaper to verify (roughly `log(log(n))`)
- the tree is stable, i.e. it respects insertion order (think of a CRDT)

The tree uses a `Vec` arena, which is guaranteed to do at
most `O(log(n))` allocations. An alternative implementation
could use an `HashMap`, in exchange for slightly worse 
performance.