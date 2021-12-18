use std::time::Instant;
use an_ok_avl_tree::AVLTree;

fn main() {
    let now  = Instant::now();
    let mut tree = AVLTree::new();
    for i in 0..10000 {
        tree.insert(i, i);
    }
    let elapsed_time = now.elapsed();
    println!("AVL Tree insert 10000 times took {} ms.", elapsed_time.as_millis());
}