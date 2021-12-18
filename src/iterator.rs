use crate::AVLTree;
use std::collections::{Bound, VecDeque};

// 范围迭代器
pub struct RangePairIter<'a, K: PartialOrd + Clone, V> {
    tree: &'a AVLTree<K, V>, // AVL树的借用
    from: Bound<K>, // 范围的起点
    to: Bound<K>, //范围的终点
    prev: Option<&'a K>, // 前一次迭代时输出的key
}

impl<'a, K: PartialOrd + Clone, V> RangePairIter<'a, K, V> {
    pub fn new(tree: &'a AVLTree<K, V>, lower: Bound<K>, upper: Bound<K>) -> Self {
        Self {
            tree,
            from: lower,
            to: upper,
            prev: None,
        }
    }

    // 获取迭代器中的下一个键值对，检查上下边界
    fn get_next_key_under(&mut self) -> Option<(&'a K, &'a V)> {
        let res = self
            .get_next_pair()
            .and_then(|cur| self.check_upper_bound(cur));
        if let Some((key, _)) = res {
            self.prev = Some(key);
        }
        res
    }

    // 获取迭代器中的下一个键值对，检查下边界
    fn get_next_pair(&mut self) -> Option<(&'a K, &'a V)> {
        match self.prev {
            None => self.get_lower_bound_pair(),
            Some(key) => self.tree.successor(key),
        }
    }

    // 获取下边界对应的键值对
    fn get_lower_bound_pair(&self) -> Option<(&'a K, &'a V)> {
        match self.from {
            Bound::Included(ref key) => {
                self.tree.get_pair(key).or_else(|| self.tree.successor(key))
            }
            Bound::Excluded(ref key) => self.tree.successor(key),
            Bound::Unbounded => self.tree.min_pair(),
        }
    }

    // 检查是否超过上边界，超过则返回None
    fn check_upper_bound(&self, current: (&'a K, &'a V)) -> Option<(&'a K, &'a V)> {
        let ok = match self.to {
            Bound::Included(ref key) => current.0 <= key,
            Bound::Excluded(ref key) => current.0 < key,
            Bound::Unbounded => true,
        };
        if ok {
            Some(current)
        } else {
            None
        }
    }
}

impl<'a, K: PartialOrd + Clone, V> Iterator for RangePairIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_key_under()
    }
}

//遍历迭代器，包括前序、中序、后序、层序
pub struct TraverseIter<'a, K, V> {
    data: VecDeque<(&'a K, &'a V)>,
}

impl<'a, K, V> TraverseIter<'a, K, V> {
    pub fn new(queue: VecDeque<(&'a K, &'a V)>) -> Self {
        TraverseIter { data: queue }
    }
}

impl<'a, K: PartialOrd + Clone, V> Iterator for TraverseIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop_front()
    }
}
