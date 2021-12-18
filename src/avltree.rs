use crate::iterator::{RangePairIter, TraverseIter};
use crate::node::{Node, Link};
use std::collections::{Bound, VecDeque};

pub struct AVLTree<K, V> {
    root: Link<K, V>,
}

impl<K: PartialOrd + Clone, V> AVLTree<K, V> {
    /// 构建一棵空的AVL树
    /// # Examples
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree: AVLTree<i32, i32> = AVLTree::new();
    /// ```
    pub fn new() -> Self {
        Self { root: None }
    }

    /// 向AVL树中插入键值对，如果键已经存在，则替换旧值为新值
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.get(&1), Some(&'a'));
    /// tree.insert(2, 'b');
    /// assert_eq!(tree.get(&2), Some(&'b'));
    /// ```
    pub fn insert(&mut self, key: K, value: V) {
        match self.root.take() {
            None => self.root = Some(Box::new(Node::new(key, value))),
            Some(node) => self.root = Some(node.insert(key, value)),
        }
    }

    /// 从AVL树中删除键值对，如果找不到键值对，则忽略
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// tree.insert(1, 'a');
    /// tree.delete(1);
    /// assert!(tree.is_empty());
    /// tree.delete(2);
    /// assert!(tree.is_empty());
    /// ```
    pub fn delete(&mut self, key: K) {
        if let Some(node) = self.root.take() {
            self.root = node.delete(key)
        }
    }

    /// 判断当前AVL树是否为空
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree: AVLTree<i32, i32> = AVLTree::new();
    /// assert!(tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// 根据键获取相应键值对
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.get_pair(&1), Some((&1, &'a')));
    /// ```
    pub fn get_pair(&self, key: &K) -> Option<(&K, &V)> {
        self.root.as_ref().and_then(|node| node.search_pair(key))
    }

    /// 根据键查找对应的值，找不到返回None，返回值的不可变借用
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.get(&1), Some(&'a'));
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        self.root.as_ref().and_then(|node| node.search(key))
    }

    /// 据键查找对应的值，找不到返回默认值
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.get_or(&1, &'z'), &'a');
    /// assert_eq!(tree.get_or(&2, &'z'), &'z');
    /// ```
    pub fn get_or<'a>(&'a self, key: &K, default: &'a V) -> &'a V {
        self.get(key).map_or(default, |data| data)
    }

    /// 查找是否存在键值对
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.contains(&1), true);
    /// assert_eq!(tree.contains(&2), false);
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// 返回AVL树中的最小键值对
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.min_pair(), Some((&1, &'a')));
    /// ```
    pub fn min_pair(&self) -> Option<(&K, &V)> {
        self.root.as_ref().map(|node| node.min_pair())
    }

    /// 返回AVL树中的最大键值对
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.max_pair(), Some((&3, &'c')));
    /// ```
    pub fn max_pair(&self) -> Option<(&K, &V)> {
        self.root.as_ref().map(|node| node.max_pair())
    }

    /// 判断是否为AVL树，空树不算AVL树
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// assert!(!tree.is_avl_tree());
    /// tree.insert(1, 'a');
    /// assert!(tree.is_avl_tree());
    /// ```
    pub fn is_avl_tree(&self) -> bool {
        if self.root.is_none() {
            return false;
        }
        Node::is_avl_tree(&self.root)
    }

    ///返回第一个大于key的键值对
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.successor(&1), Some((&2, &'b')));
    /// assert_eq!(tree.successor(&0), Some((&1, &'a')));
    /// assert_eq!(tree.successor(&3), None);
    /// ```
    pub fn successor(&self, key: &K) -> Option<(&K, &V)> {
        self.root.as_ref().and_then(|node| node.successor(key))
    }

    ///返回第一个小于key的键值对
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.predecessor(&3), Some((&2, &'b')));
    /// assert_eq!(tree.predecessor(&5), Some((&3, &'c')));
    /// assert_eq!(tree.predecessor(&1), None);
    /// ```
    pub fn predecessor(&self, key: &K) -> Option<(&K, &V)> {
        self.root.as_ref().and_then(|node| node.predecessor(key))
    }

    /// 范围迭代器
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// use std::collections::Bound;
    /// let mut tree = AVLTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// let res: Vec<(&i32, &char)> = tree.range_pair_iter(Bound::Unbounded, Bound::Unbounded).collect();
    /// assert_eq!(res, vec![(&1, &'a'), (&2, &'b'), (&3, &'c')]);
    /// let res: Vec<(&i32, &char)> = tree.range_pair_iter(Bound::Included(0), Bound::Included(2)).collect();
    /// assert_eq!(res, vec![(&1, &'a'), (&2, &'b')]);
    /// let res: Vec<(&i32, &char)> = tree.range_pair_iter(Bound::Excluded(1), Bound::Excluded(3)).collect();
    /// assert_eq!(res, vec![(&2, &'b')]);
    /// ```
    pub fn range_pair_iter(&self, min: Bound<K>, max: Bound<K>) -> RangePairIter<K, V> {
        RangePairIter::new(self, min, max)
    }

    /// 前序遍历迭代器
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// let res: Vec<(&i32, &char)> = tree.preorder_iter().collect();
    /// assert_eq!(res, vec![(&2, &'b'), (&1, &'a'), (&3, &'c')]);
    /// ```
    pub fn preorder_iter(&self) -> TraverseIter<K, V> {
        let pre_order = self.prev_order();
        let mut queue = VecDeque::new();
        for key in pre_order {
            if let Some(p) = self.get_pair(&key) {
                queue.push_back(p);
            }
        }
        TraverseIter::new(queue)
    }

    /// 中序遍历迭代器
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// let res: Vec<(&i32, &char)> = tree.inorder_iter().collect();
    /// assert_eq!(res, vec![(&1, &'a'), (&2, &'b'), (&3, &'c')]);
    /// ```
    pub fn inorder_iter(&self) -> TraverseIter<K, V> {
        let in_order = self.in_order();
        let mut queue = VecDeque::new();
        for key in in_order {
            if let Some(p) = self.get_pair(&key) {
                queue.push_back(p);
            }
        }
        TraverseIter::new(queue)
    }

    /// 后序遍历迭代器
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// let res: Vec<(&i32, &char)> = tree.postorder_iter().collect();
    /// assert_eq!(res, vec![(&1, &'a'), (&3, &'c'), (&2, &'b')]);
    /// ```
    pub fn postorder_iter(&self) -> TraverseIter<K, V> {
        let post_order = self.post_order();
        let mut queue = VecDeque::new();
        for key in post_order {
            if let Some(p) = self.get_pair(&key) {
                queue.push_back(p);
            }
        }
        TraverseIter::new(queue)
    }

    /// 层序遍历迭代器
    /// # Example
    /// ```
    /// use an_ok_avl_tree::AVLTree;
    /// let mut tree = AVLTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// let res: Vec<(&i32, &char)> = tree.levelorder_iter().collect();
    /// assert_eq!(res, vec![(&2, &'b'), (&1, &'a'), (&3, &'c')]);
    /// ```
    pub fn levelorder_iter(&self) -> TraverseIter<K, V> {
        let level_order = self.level_order();
        let mut queue = VecDeque::new();
        for key in level_order {
            if let Some(p) = self.get_pair(&key) {
                queue.push_back(p);
            }
        }
        TraverseIter::new(queue)
    }

    ///前序遍历
    fn prev_order(&self) -> Vec<K> {
        let mut buf = Vec::new();
        Node::prev_order(&self.root, &mut buf);
        buf
    }

    ///中序遍历
    fn in_order(&self) -> Vec<K> {
        let mut buf = Vec::new();
        Node::in_order(&self.root, &mut buf);
        buf
    }

    ///后序遍历
    fn post_order(&self) -> Vec<K> {
        let mut buf = Vec::new();
        Node::post_order(&self.root, &mut buf);
        buf
    }

    ///层序遍历
    fn level_order(&self) -> Vec<K> {
        let mut buf = Vec::new();
        Node::level_order(&self.root, &mut buf);
        buf
    }
}

/// 将AVL树打印成字符串
/// # Example
/// ```
/// use an_ok_avl_tree::AVLTree;
/// let mut tree = AVLTree::new();
/// tree.insert(1, 'a');
/// assert_eq!(tree.to_string(), "[K: 1, V: a, L: Ø, R: Ø]".to_string());
/// ```
impl<K: PartialOrd + ToString, V: ToString> ToString for AVLTree<K, V> {
    fn to_string(&self) -> String {
        self.root
            .as_ref()
            .map_or(String::from("None"), |node| node.to_string())
    }
}

impl<K: PartialOrd + Clone, V> Default for AVLTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
