use std::ptr;

pub struct Tree {
    pub count: usize,
    pub root: *mut Node,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            count: 0,
            root: std::ptr::null_mut(),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub data: i32,
    pub left: *mut Node,
    pub right: *mut Node,
    pub parent: *mut Node,
}

impl Tree {
    pub fn new() -> Self {
        Self::default()
    }

    // Return tree count
    pub fn node_count(&self) -> usize {
        assert!(self.count != 0 || self.root.is_null());
        self.count
    }

    // Insert a new node in tree, return true if suceeded otherwise return false
    pub fn insert(&mut self, data: i32) -> bool {
        if self.root.is_null() {
            self.root = Node::new(data);
        } else {
            unsafe {
                if !insert_node(self.root, data) {
                    return false;
                }
            }
        }

        self.count += 1;
        true
    }

    pub fn find(&self, data: i32) -> bool {
        unsafe { !find_node(self.root, data).is_null() }
    }
}

impl Node {
    pub fn new(data: i32) -> *mut Self {
        Box::into_raw(Box::new(Self {
            data,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            parent: ptr::null_mut(),
        }))
    }

    pub fn new_with_parent(data: i32, parent: *mut Node) -> *mut Self {
        Box::into_raw(Box::new(Node {
            data,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            parent,
        }))
    }
}

/// # Safety
///
/// There has no safety.
pub unsafe fn insert_node(node: *mut Node, data: i32) -> bool {
    use std::cmp::Ordering::*;

    match (*node).data.cmp(&data) {
        Less => {
            if (*node).left.is_null() {
                (*node).left = Node::new_with_parent(data, node);
                true
            } else {
                insert_node((*node).left, data)
            }
        }
        Equal => false,
        Greater => {
            if (*node).right.is_null() {
                (*node).right = Node::new_with_parent(data, node);
                true
            } else {
                insert_node((*node).right, data)
            }
        }
    }
}

/// # Safety
pub unsafe fn find_node(node: *mut Node, data: i32) -> *mut Node {
    use std::cmp::Ordering::*;

    if node.is_null() {
        return node;
    }
    match (*node).data.cmp(&data) {
        Less => find_node((*node).left, data),
        Equal => node,
        Greater => find_node((*node).right, data),
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn insert_root_test() {
        let mut tree = Tree::new();
        assert!(tree.count == 0);

        tree.insert(1);
        assert!(tree.count == 1);

        tree.insert(1);
        assert!(tree.count == 1);

        tree.insert(2);
        assert!(tree.count == 2);
    }

    #[test]
    fn find_data_test() {
        let mut tree = Tree::new();

        tree.insert(1);
        tree.insert(2);
        tree.insert(42);

        assert!(tree.find(42));
        assert!(!tree.find(24));
    }
}
