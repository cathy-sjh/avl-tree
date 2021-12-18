#[cfg(test)]
mod tests {
    use an_ok_avl_tree::AVLTree;
    use std::collections::Bound;

    #[test]
    fn insert_delete() {
        /*
                         4
                       /   \
                     2       9
                    / \     /  \
                   1   3   7    10
                            \
                             8
        */
        let mut tree = AVLTree::new();
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(1, 'a');
        tree.insert(4, 'd');
        tree.insert(5, 'e');
        tree.insert(6, 'f');
        tree.insert(7, 'g');
        tree.insert(10, 'j');
        tree.insert(9, 'i');
        tree.insert(8, 'h');
        assert!(tree.is_avl_tree());
        tree.delete(5);
        tree.delete(6);
        assert!(!tree.contains(&5));
        assert!(!tree.contains(&6));
        let res: Vec<(&i32, &char)> = tree.inorder_iter().collect();
        assert_eq!(
            res,
            vec![
                (&1, &'a'),
                (&2, &'b'),
                (&3, &'c'),
                (&4, &'d'),
                (&7, &'g'),
                (&8, &'h'),
                (&9, &'i'),
                (&10, &'j')
            ]
        );
        assert_eq!(tree.successor(&4), Some((&7, &'g')));
        assert_eq!(tree.successor(&5), Some((&7, &'g')));
        assert_eq!(tree.successor(&6), Some((&7, &'g')));
    }

    #[test]
    fn test_empty() {
        let data = 1337;
        let mut t = AVLTree::new();
        assert!(t.is_empty());
        t.insert(1, data);
        t.insert(2, data + 1);
        t.insert(3, data + 2);
        assert!(!t.is_empty());
    }

    #[test]
    fn max_min_get_pair() {
        /*
                         4
                       /   \
                     2       7
                    / \     /  \
                   1   3   6    9
                          /    / \
                         5    8   10
        */
        let mut tree = AVLTree::new();
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(1, 'a');
        tree.insert(4, 'd');
        tree.insert(5, 'e');
        tree.insert(6, 'f');
        tree.insert(7, 'g');
        tree.insert(10, 'j');
        tree.insert(9, 'i');
        tree.insert(8, 'h');
        assert!(tree.is_avl_tree());
        assert_eq!(tree.min_pair(), Some((&1, &'a')));
        assert_eq!(tree.max_pair(), Some((&10, &'j')));
        assert_eq!(tree.get_pair(&4), Some((&4, &'d')));
        assert_eq!(tree.get_pair(&9), Some((&9, &'i')));
        assert_eq!(tree.get_pair(&11), None);
        assert_eq!(tree.get(&4), Some(&'d'));
        assert_eq!(tree.get(&2), Some(&'b'));
        assert_eq!(tree.get(&9), Some(&'i'));
        assert_eq!(tree.get(&10), Some(&'j'));
        assert_eq!(tree.get_or(&5, &'z'), &'e');
        assert_eq!(tree.get_or(&6, &'z'), &'f');
        assert_eq!(tree.get_or(&11, &'z'), &'z');
        assert!(tree.contains(&10));
        assert!(!tree.contains(&12));
    }

    #[test]
    fn successor_predecessor() {
        let mut tree = AVLTree::new();
        tree.insert(3, "3");
        tree.insert(2, "2");
        tree.insert(1, "1");
        tree.insert(4, "4");
        tree.insert(5, "5");
        tree.insert(6, "6");
        tree.insert(7, "7");
        tree.insert(10, "10");
        tree.insert(9, "9");
        tree.insert(8, "8");
        assert!(tree.is_avl_tree());
        assert_eq!(tree.successor(&6), Some((&7, &"7")));
        assert_eq!(tree.successor(&3), Some((&4, &"4")));
        assert_eq!(tree.predecessor(&5), Some((&4, &"4")));
        assert_eq!(tree.successor(&10), None);
        assert_eq!(tree.predecessor(&1), None);
        assert_eq!(tree.successor(&0), Some((&1, &"1")));
        assert_eq!(tree.predecessor(&100), Some((&10, &"10")));
    }

    #[test]
    fn test_traverse_iter() {
        let mut tree = AVLTree::new();
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(1, 'a');
        tree.insert(4, 'd');
        let res: Vec<(&i32, &char)> = tree.preorder_iter().collect();
        assert_eq!(res, vec![(&2, &'b'), (&1, &'a'), (&3, &'c'), (&4, &'d')]);
        let res: Vec<(&i32, &char)> = tree.inorder_iter().collect();
        assert_eq!(res, vec![(&1, &'a'), (&2, &'b'), (&3, &'c'), (&4, &'d')]);
        let res: Vec<(&i32, &char)> = tree.postorder_iter().collect();
        assert_eq!(res, vec![(&1, &'a'), (&4, &'d'), (&3, &'c'), (&2, &'b')]);
        let res: Vec<(&i32, &char)> = tree.levelorder_iter().collect();
        assert_eq!(res, vec![(&2, &'b'), (&1, &'a'), (&3, &'c'), (&4, &'d')]);
    }

    #[test]
    fn range_pair_iter() {
        /*
                         4
                       /   \
                     2       7
                    / \     /  \
                   1  3    6    9
                          /    / \
                         5    8   10
        */
        let mut tree = AVLTree::new();
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(1, 'a');
        tree.insert(4, 'd');
        tree.insert(5, 'e');
        tree.insert(6, 'f');
        tree.insert(7, 'g');
        tree.insert(10, 'j');
        tree.insert(9, 'i');
        tree.insert(8, 'z');
        tree.insert(8, 'h');
        let res: Vec<(&i32, &char)> = tree
            .range_pair_iter(Bound::Unbounded, Bound::Unbounded)
            .collect();
        assert_eq!(
            res,
            vec![
                (&1, &'a'),
                (&2, &'b'),
                (&3, &'c'),
                (&4, &'d'),
                (&5, &'e'),
                (&6, &'f'),
                (&7, &'g'),
                (&8, &'h'),
                (&9, &'i'),
                (&10, &'j')
            ]
        );
        let res: Vec<(&i32, &char)> = tree
            .range_pair_iter(Bound::Included(1), Bound::Included(4))
            .collect();
        assert_eq!(res, vec![(&1, &'a'), (&2, &'b'), (&3, &'c'), (&4, &'d')]);
        let res: Vec<(&i32, &char)> = tree
            .range_pair_iter(Bound::Included(0), Bound::Included(4))
            .collect();
        assert_eq!(res, vec![(&1, &'a'), (&2, &'b'), (&3, &'c'), (&4, &'d')]);
        let res: Vec<(&i32, &char)> = tree
            .range_pair_iter(Bound::Excluded(1), Bound::Excluded(5))
            .collect();
        assert_eq!(res, vec![(&2, &'b'), (&3, &'c'), (&4, &'d')]);
    }

    #[test]
    fn to_string() {
        let mut tree = AVLTree::new();
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(1, 'a');
        tree.insert(4, 'd');
        assert_eq!(tree.to_string(), String::from("[K: 2, V: b, L: [K: 1, V: a, L: Ø, R: Ø], R: [K: 3, V: c, L: Ø, R: [K: 4, V: d, L: Ø, R: Ø]]]"))
    }
}
