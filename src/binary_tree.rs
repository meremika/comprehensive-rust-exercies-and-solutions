/// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: Subtree::new(),
            right: Subtree::new(),
        }
    }
}

/// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

impl<T: Ord> Subtree<T> {
    fn new() -> Self {
        Self(None)
    }

    fn insert(&mut self, value: T) {
        match &mut self.0 {
            None => self.0 = Some(Box::new(Node::new(value))),
            Some(node) => match value.cmp(&node.value) {
                std::cmp::Ordering::Less => node.left.insert(value),
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => node.right.insert(value),
            },
        }
    }

    fn has(&self, value: &T) -> bool {
        match &self.0 {
            None => false,
            Some(node) => match value.cmp(&node.value) {
                std::cmp::Ordering::Less => node.left.has(value),
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Greater => node.right.has(value),
            },
        }
    }

    fn len(&self) -> usize {
        match &self.0 {
            Some(node) => node.left.len() + node.right.len() + 1,
            None => 0,
        }
    }
}

/// A container storing a set of values, using a binary tree.
///
/// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            root: Subtree::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    pub fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Ord> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Implement `new`, `insert`, `len`, and `has` for `Subtree`.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
        tree.insert(3);
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}
