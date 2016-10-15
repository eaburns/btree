pub enum Tree<K, V> {
    Node {
        left: Box<Tree<K, V>>,
        right: Box<Tree<K, V>>,
        value: V,
        key: K,
    },
    Nil,
}

impl<K: PartialEq + PartialOrd, V> Tree<K, V> {
    // Returns a new, empty Tree.
    pub fn new() -> Tree<K, V> {
        Tree::Nil
    }

    // Returns whether the Tree is empty.
    pub fn is_empty(&self) -> bool {
        match *self {
            Tree::Nil => true,
            _ => false,
        }
    }

    // Adds a binding between a key and a value, returning the new Tree.
    // If such a binding already existed, the tree is left unmodified.
    pub fn insert(self, key: K, value: V) -> Tree<K, V> {
        match self {
            Tree::Nil => {
                Tree::Node {
                    left: Box::new(Tree::Nil),
                    right: Box::new(Tree::Nil),
                    key: key,
                    value: value,
                }
            }
            Tree::Node { left: l, right: r, key: k, value: v } => {
                if key == k {
                    Tree::Node {
                        left: l,
                        right: r,
                        key: k,
                        value: v,
                    }
                } else if key < k {
                    Tree::Node {
                        left: Box::new(l.insert(key, value)),
                        right: r,
                        key: k,
                        value: v,
                    }
                } else {
                    Tree::Node {
                        left: l,
                        right: Box::new(r.insert(key, value)),
                        key: k,
                        value: v,
                    }
                }
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match *self {
            Tree::Nil => None,
            Tree::Node { value: ref v, key: ref k, .. } if *key == *k => Some(v),
            Tree::Node { left: ref l, key: ref k, .. } if *key < *k => l.get(key),
            Tree::Node { right: ref r, .. } => r.get(key),
        }
    }
}

#[test]
fn empty_tree() {
    let mut t = Tree::new();
    assert!(t.is_empty());
    t = t.insert(8, 8);
    assert!(!t.is_empty());
}

#[test]
fn test_insert() {
    let mut t = Tree::new();
    t = t.insert(8, 8);
    t = t.insert(9, 9);
    assert!(t.get(&8) == Some(&8));
    assert!(t.get(&9) == Some(&9));
    assert!(t.get(&1) == None);
}

#[test]
fn test_playing() {
    let z = Box::new(0);
    let mut t = Tree::new();
    let mut x: &Box<i8> = &z;

    t = t.insert(8, Box::new(8i8));
    if let Some(y) = t.get(&8) {
        x = y
    }
    print!("{}", x)
}
