use std::cmp::max;
use std::collections::VecDeque;

pub type Link<K, V> = Option<Box<Node<K, V>>>;

pub struct Node<K, V> {
    key: K, //键
    value: V, //值
    height: u32, //树高
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K: PartialOrd + Clone, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    // 判断当前节点是否为叶子节点
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    // 得到当前节点的高度
    fn height(node: &Link<K, V>) -> u32 {
        node.as_ref().map_or(0, |node| node.height)
    }

    // 更新当前节点的高度
    fn update_height(&mut self) {
        self.height = max(Self::height(&self.left), Self::height(&self.right)) + 1;
    }

    //对当前节点进行一次左旋操作，返回旋转后的根节点
    fn left_rotate(mut self) -> Box<Node<K, V>> {
        let mut new_root = self.right.take().expect("AVL broken");
        self.right = new_root.left.take();
        self.update_height();
        new_root.left = Some(Box::new(self));
        new_root.update_height();
        new_root
    }

    //对当前节点进行一次右旋操作，返回旋转后的根节点
    fn right_rotate(mut self) -> Box<Node<K, V>> {
        let mut new_root = self.left.take().expect("AVL broken");
        self.left = new_root.right.take();
        self.update_height();
        new_root.right = Some(Box::new(self));
        new_root.update_height();
        new_root
    }

    //保持左侧平衡。传入的self是一颗不平衡的树，左子树比右子树高2
    fn left_balance(mut self) -> Box<Node<K, V>> {
        let left = self.left.take().expect("AVL broken");
        if Self::height(&left.left) < Self::height(&left.right) {
            let rotated = left.left_rotate();
            self.left = Some(rotated);
            self.update_height();
        } else {
            self.left = Some(left);
        }
        self.right_rotate()
    }

    //保持右侧平衡。传入的self是一颗不平衡的树，右子树比左子树高2
    fn right_balance(mut self) -> Box<Node<K, V>> {
        let right = self.right.take().expect("AVL broken");
        if Self::height(&right.left) > Self::height(&right.right) {
            let rotated = right.right_rotate();
            self.right = Some(rotated);
            self.update_height();
        } else {
            self.right = Some(right);
        }
        self.left_rotate()
    }

    //计算当前节点左右子树的高度差
    fn diff_of_height(&self) -> i32 {
        let l = Self::height(&self.left);
        let r = Self::height(&self.right);
        (l as i32) - (r as i32)
    }

    //判断当前节点是否需要进行旋转调整，返回调整后的根节点
    fn rotate_if_necessary(self) -> Box<Node<K, V>> {
        let diff = self.diff_of_height();
        if -1 <= diff && diff <= 1 {
            Box::new(self)
        } else if diff == -2 {
            self.right_balance()
        } else if diff == 2 {
            self.left_balance()
        } else {
            unreachable!()
        }
    }

    //更新当前根节点，包括高度更新和旋转操作
    fn update_node(mut self) -> Box<Node<K, V>> {
        self.update_height();
        self.rotate_if_necessary()
    }

    //插入新节点，并返回调整后的根节点
    pub fn insert(mut self, key: K, value: V) -> Box<Node<K, V>> {
        if self.key > key {
            match self.left.take() {
                None => {
                    self.left = Some(Box::new(Node::new(key, value)));
                }
                Some(node) => {
                    self.left = Some(node.insert(key, value));
                }
            }
        } else if self.key < key {
            match self.right.take() {
                None => {
                    self.right = Some(Box::new(Node::new(key, value)));
                }
                Some(node) => {
                    self.right = Some(node.insert(key, value));
                }
            }
        } else {
            self.value = value;
            return Box::new(self);
        }
        self.update_node()
    }

    //找出当前树中值最小的节点，返回元组:(除去最小节点后剩下的树，最小节点)
    fn remove_min(mut self) -> (Link<K, V>, Box<Node<K, V>>) {
        match self.left.take() {
            Some(left) => {
                let (new_left, min) = left.remove_min();
                self.left = new_left;
                (Some(self.update_node()), min)
            }
            None => (self.right.take(), Box::new(self)),
        }
    }

    //将两棵子树合并为一棵，合并后仍然满足AVL树的规则，返回新生成树的根节点
    fn combine_two_subtrees(
        left: Node<K, V>,
        right: Node<K, V>,
    ) -> Box<Node<K, V>> {
        // 得到右子树中最小的节点和去除最小节点后剩余的树
        let (remain_tree, min) = right.remove_min();
        // 最小节点作为两个子树的新根节点
        let mut new_root = min;
        new_root.right = remain_tree;
        new_root.left = Some(Box::new(left));
        new_root.update_node()
    }

    //删除当前节点，重构二叉树，并返回新的根节点
    fn delete_root(mut self) -> Link<K, V> {
        // AVL树删除节点的三种情况(包括二叉搜索树)，AVL树的删除还要多一步旋转操作
        // 1.如果是叶子节点，则直接删除
        // 2.如果待删除节点只有左子树或只有右子树，删除该节点，然后将左子树或右子树移动到该节点
        // 3.如果待删除节点左右子树都有，就选取右子树中最小的节点代替待删除节点的位置(或者取左子树中最大节点代替也可以)。
        match (self.left.take(), self.right.take()) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(left), Some(right)) => Some(Self::combine_two_subtrees(*left, *right)),
        }
    }

    //删除节点key，并保持改树仍为AVL树，返回的新生成的树的根节点
    pub fn delete(mut self, key: K) -> Link<K, V> {
        if self.key < key {
            if let Some(succ) = self.right.take() {
                self.right = succ.delete(key);
                return Some(self.update_node());
            }
        } else if self.key > key {
            if let Some(succ) = self.left.take() {
                self.left = succ.delete(key);
                return Some(self.update_node());
            }
        } else {
            return self.delete_root();
        }
        // 没有找到待删除节点则直接返回
        Some(Box::new(self))
    }

    // 返回第一个大于key的键值对,key可以不存在树中
    pub fn successor(&self, key: &K) -> Option<(&K, &V)> {
        if self.key > *key {
            match self.left {
                None => Some((&self.key, &self.value)),
                Some(ref succ) => succ.successor(key).or(Some((&self.key, &self.value))),
            }
        } else if self.key < *key {
            self.right.as_ref().and_then(|right| right.successor(key))
        } else {
            self.right.as_ref().map(|right| right.min_pair())
        }
    }

    // 返回第一个小于key的键值对,key可以不存在树中
    pub fn predecessor(&self, key: &K) -> Option<(&K, &V)> {
        if self.key < *key {
            match self.right {
                None => Some((&self.key, &self.value)),
                Some(ref succ) => succ.predecessor(key).or(Some((&self.key, &self.value))),
            }
        } else if self.key > *key {
            self.left.as_ref().and_then(|left| left.predecessor(key))
        } else {
            self.left.as_ref().map(|left| left.max_pair())
        }
    }

    // 前序遍历
    pub fn prev_order(root: &Link<K, V>, buf: &mut Vec<K>) {
        if let Some(node) = root {
            buf.push(node.key.clone());
            Self::prev_order(&node.left, buf);
            Self::prev_order(&node.right, buf);
        }
    }

    // 中序遍历
    pub fn in_order(root: &Link<K, V>, buf: &mut Vec<K>) {
        if let Some(node) = root {
            Self::in_order(&node.left, buf);
            buf.push(node.key.clone());
            Self::in_order(&node.right, buf);
        }
    }

    // 后序遍历
    pub fn post_order(root: &Link<K, V>, buf: &mut Vec<K>) {
        if let Some(node) = root {
            Self::post_order(&node.left, buf);
            Self::post_order(&node.right, buf);
            buf.push(node.key.clone());
        }
    }

    // 层序遍历
    pub fn level_order(root: &Link<K, V>, buf: &mut Vec<K>) {
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node);
        }
        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                buf.push(node.key.clone());
                if let Some(left) = node.left.as_ref() {
                    queue.push_back(left);
                }
                if let Some(right) = node.right.as_ref() {
                    queue.push_back(right);
                }
            }
        }
    }

    // 返回查找的键值对的不可变借用
    pub fn search_pair(&self, key: &K,) -> Option<(&K, &V)> {
        if self.key < *key {
            self.right
                .as_ref()
                .and_then(|right| right.search_pair(key))
        } else if self.key > *key {
            self.left.as_ref().and_then(|left| left.search_pair(key))
        } else {
            Some((&self.key, &self.value))
        }
    }

    // 根据键查找对应的值
    pub fn search(&self, key: &K) -> Option<&V> {
        self.search_pair(key).map(|(_, v)| v)
    }

    // 返回AVL树中的最小键值对
    pub fn min_pair(&self) -> (&K, &V) {
        self.left
            .as_ref()
            .map_or((&self.key, &self.value), |left| left.min_pair())
    }

    // 返回AVL树中的最大键值对
    pub fn max_pair(&self) -> (&K, &V) {
        self.right
            .as_ref()
            .map_or((&self.key, &self.value), |right| right.max_pair())
    }

    // 判断节点是否满足AVL树的性质
    fn is_avl_node(&self) -> bool {
        if self.is_leaf() {
            return true;
        }
        if !self.left.as_ref().map_or(true, |succ| succ.key < self.key) {
            return false;
        }
        if !self.right.as_ref().map_or(true, |succ| succ.key > self.key) {
            return false;
        }
        let balance = self.diff_of_height();
        if balance > 1 || balance < -1 {
            return false;
        }
        true
    }

    // 判断是否为AVL树
    pub fn is_avl_tree(root: &Link<K, V>) -> bool {
        match root {
            None => true,
            Some(node) => {
                if !node.is_avl_node() {
                    return false;
                }
                Self::is_avl_tree(&node.left) && Self::is_avl_tree(&node.right)
            }
        }
    }
}

impl<K: PartialOrd + ToString, V: ToString> ToString for Node<K, V> {
    fn to_string(&self) -> String {
        format!(
            "[K: {}, V: {}, L: {}, R: {}]",
            self.key.to_string(),
            self.value.to_string(),
            to_string(&self.left),
            to_string(&self.right)
        )
    }
}

fn to_string<K: PartialOrd + ToString, V: ToString>(node: &Link<K, V>) -> String {
    match node {
        None => "Ø".to_string(),
        Some(box_node) => box_node.to_string(),
    }
}