use std::fmt::Display;

type TreeNode<K, V> = Option<Box<Node<K, V>>>;
#[derive(Debug)]
struct Node<K, V: Clone + Display> {
    left: TreeNode<K, V>,
    right: TreeNode<K, V>,
    key: K,
    value: V,
}

trait BinaryTree<K, V> {
    fn pre_order(&self) -> Vec<V>;
    fn in_order(&self) -> Vec<V>;
    fn pos_order(&self) -> Vec<V>;
}
trait BinarySearchTree<K: PartialOrd, V>: BinaryTree<K, V> {
    fn insert(&mut self, key: K, value: V);
}

impl<K, V: Display + Clone> Node<K, V> {
    fn new(key: K, value: V) -> Node<K, V> {
        Node {
            left: None,
            right: None,
            key: key,
            value: value,
        }
    }
}

impl<K: PartialOrd, V: Display + Clone> BinarySearchTree<K, V> for Node<K, V> {
    fn insert(&mut self, key: K, value: V) {
        if self.key < key {
            if let Some(ref mut right) = self.right {
                right.insert(key, value);
            } else {
                self.right = Some(Box::new(Node::new(key, value)));
            }
        } else {
            if let Some(ref mut left) = self.left {
                left.insert(key, value);
            } else {
                self.left = Some(Box::new(Node::new(key, value)))
            }
        }
    }
}

impl<K, V: Display + Clone> BinaryTree<K, V> for Node<K, V> {
    fn pre_order(&self) -> Vec<V> {
        let mut result = Vec::new();
        result.push(self.value.clone());
        if let Some(ref left) = self.left {
            result.append(&mut left.pre_order());
        }
        if let Some(ref right) = self.right {
            result.append(&mut right.pre_order());
        }
        result
    }

    fn in_order(&self) -> Vec<V> {
        let mut result = Vec::new();
        if let Some(ref left) = self.left {
            result.append(&mut left.pre_order());
        }
        result.push(self.value.clone());
        if let Some(ref right) = self.right {
            result.append(&mut right.pre_order());
        }
        result
    }

    fn pos_order(&self) -> Vec<V> {
        let mut result = Vec::new();
        if let Some(ref left) = self.left {
            result.append(&mut left.pre_order());
        }
        if let Some(ref right) = self.right {
            result.append(&mut right.pre_order());
        }
        result.push(self.value.clone());
        result
    }
}

//     3
//   2   4
// 1       5
//           6
#[test]
fn test_binary_tree_operation() {
    let mut root = Node::<i32, &str>::new(3, "3-4");
    root.insert(2, "2-3");
    root.insert(4, "4-6");
    root.insert(5, "5-5");
    root.insert(6, "6-6");
    root.insert(1, "1-8");

    if let Some(ref left) = root.left {
        assert_eq!(2, left.key);
        assert_eq!("2-3", left.value);
    }

    if let Some(ref right) = root.right {
        assert_eq!(4, right.key);
        assert_eq!("4-6", right.value);
        if let Some(ref right) = right.right {
            assert_eq!(5, right.key);
            assert_eq!("5-5", right.value);
        }
    }

    let pre_order = vec!["3-4", "2-3", "1-8", "4-6", "5-5", "6-6"];
    let in_order = vec!["2-3", "1-8", "3-4", "4-6", "5-5", "6-6"];
    let pos_order = vec!["2-3", "1-8", "4-6", "5-5", "6-6", "3-4"];
    assert_eq!(pre_order, root.pre_order());
    assert_eq!(in_order, root.in_order());
    assert_eq!(pos_order, root.pos_order());
}
