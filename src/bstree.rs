use std::io::Write;

struct BstreeNode<T> {
    pub val: T,
    pub left: Option<Box<BstreeNode<T>>>,
    pub right: Option<Box<BstreeNode<T>>>,
}

impl<T: std::cmp::Ord> BstreeNode<T> {
    pub fn new(val: T) -> Box<BstreeNode<T>> {
        Box::new(BstreeNode {
            val: val,
            left: None,
            right: None,
        })
    }

    pub fn in_sub_tree(node: &Option<Box<BstreeNode<T>>>, val: &T) -> bool {
        let Some(n) = node.as_ref() else {
            return false;
        };

        if val < &n.val {
            BstreeNode::in_sub_tree(&n.left, val)
        } else if val > &n.val {
            BstreeNode::in_sub_tree(&n.right, val)
        } else {
            true
        }
    }

    pub fn insert(node: &mut Option<Box<BstreeNode<T>>>, val: T) -> bool {
        if let Some(n) = node {
            if val < n.val {
                BstreeNode::insert(&mut n.left, val)
            } else if val > n.val {
                BstreeNode::insert(&mut n.right, val)
            } else {
                false
            }
        } else {
            *node = Some(BstreeNode::new(val));

            true
        }
    }

    fn extract_max(node: &mut Option<Box<BstreeNode<T>>>) -> T {
        // if node is none or its right node is none, this node is the max one
        // and_then() returns the value in it or none if input (node.as_ref()) is none
        if node.as_ref().and_then(|n| n.right.as_ref()).is_none() {
            // extract the node input, and clear the option
            let mut max_node = node.take().expect("extract_max called on empty node");

            // rebuild in input node with its left child
            *node = max_node.left.take();

            // the original value of input node is "moved out". No data copy happens here
            max_node.val

            // lifetime of max_node ends
        } else {
            // get the max one on its right node recursively. Note that nothing changes here
            let next = node.as_mut().expect("missing node while extracting max");
            BstreeNode::extract_max(&mut next.right)
        }
    }

    pub fn delete(node: &mut Option<Box<BstreeNode<T>>>, val: &T) -> bool {
        let Some(current) = node.as_mut() else {
            return false;
        };

        if val < &current.val {
            return BstreeNode::delete(&mut current.left, val);
        } else if val > &current.val {
            return BstreeNode::delete(&mut current.right, val);
        }

        // At this point, we've found the node to delete. We detach it from the tree
        // and then reattach its children appropriately based on three cases:
        // If we changed the content in Option, we actually complete the pointer redirection.
        // After this statement, variable "current" is dropped
        let mut target = node.take().unwrap();

        if target.left.is_none() {
            // 1. No left child: replace with right subtree
            *node = target.right.take();
        } else if target.right.is_none() {
            // 2. No right child: replace with left subtree
            *node = target.left.take();
        } else {
            // take the ownership of the max value of left node
            let predecessor_val = BstreeNode::extract_max(&mut target.left);

            target.val = predecessor_val;
            *node = Some(target);
        }

        true
    }

    // 3 (indent 0)
    // \__ 1 (indent 1)
    //     \__ NONE (indent 2)
    //     \__ 2
    // \__ 4
    //     \__ NONE
    //     \__ 5
    pub fn print_sub_tree<W: Write>(
        writer: &mut W,
        node: &Box<BstreeNode<T>>,
        indent: i32,
        tag: &str,
    ) where
        T: std::fmt::Display,
    {
        for _ in 0..indent.saturating_sub(1) {
            write!(writer, "    ").unwrap();
        }
        if indent > 0 {
            write!(writer, "\\__ ").unwrap();
        }
        writeln!(writer, "{}: {}", tag, node.val).unwrap();

        if let Some(left_node) = &node.left {
            BstreeNode::print_sub_tree(writer, &left_node, indent + 1, "left");
        } else {
            for _ in 0..indent {
                write!(writer, "    ").unwrap();
            }
            writeln!(writer, "\\__ left: NONE").unwrap();
        }
        if let Some(right_node) = &node.right {
            BstreeNode::print_sub_tree(writer, &right_node, indent + 1, "right");
        } else {
            for _ in 0..indent {
                write!(writer, "    ").unwrap();
            }
            writeln!(writer, "\\__ right: NONE").unwrap();
        }
    }

    pub fn print_sub_tree_std(node: &Box<BstreeNode<T>>, indent: i32, tag: &str)
    where
        T: std::fmt::Display,
    {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        BstreeNode::print_sub_tree(&mut handle, node, indent, tag);
    }
}

pub struct Bstree<T> {
    root: Option<Box<BstreeNode<T>>>,
}

impl<T: std::cmp::Ord> Bstree<T> {
    pub fn new() -> Self {
        Bstree { root: None }
    }

    pub fn insert(&mut self, val: T) -> bool {
        BstreeNode::insert(&mut self.root, val)
    }

    pub fn exist(&self, val: &T) -> bool {
        BstreeNode::in_sub_tree(&self.root, val)
    }

    pub fn delete(&mut self, val: &T) -> bool {
        BstreeNode::delete(&mut self.root, val)
    }

    pub fn print(&self)
    where
        T: std::fmt::Display,
    {
        if let Some(r) = &self.root {
            BstreeNode::print_sub_tree_std(&r, 0, "root");
        } else {
            println!("NONE");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_find() {
        // init and insert
        let mut root = Some(BstreeNode::new(10));
        BstreeNode::insert(&mut root, 15);
        BstreeNode::insert(&mut root, 5);
        BstreeNode::insert(&mut root, 3);
        BstreeNode::insert(&mut root, 7);
        BstreeNode::insert(&mut root, 12);
        BstreeNode::insert(&mut root, 18);

        assert!(BstreeNode::in_sub_tree(&root, &10));
        assert!(BstreeNode::in_sub_tree(&root, &15));
        assert!(BstreeNode::in_sub_tree(&root, &5));
        assert!(BstreeNode::in_sub_tree(&root, &3));
        assert!(BstreeNode::in_sub_tree(&root, &7));
        assert!(BstreeNode::in_sub_tree(&root, &12));
        assert!(BstreeNode::in_sub_tree(&root, &18));
        assert!(!BstreeNode::in_sub_tree(&root, &10086));

        // delete
        assert!(BstreeNode::delete(&mut root, &10));

        assert!(BstreeNode::in_sub_tree(&root, &15));
        assert!(BstreeNode::in_sub_tree(&root, &5));
        assert!(BstreeNode::in_sub_tree(&root, &3));
        assert!(BstreeNode::in_sub_tree(&root, &7));
        assert!(BstreeNode::in_sub_tree(&root, &12));
        assert!(BstreeNode::in_sub_tree(&root, &18));

        {
            let mut buf: Vec<u8> = Vec::new();
            BstreeNode::print_sub_tree(&mut buf, root.as_ref().unwrap(), 0, "root");
            let output = String::from_utf8(buf).unwrap();
            let lines: Vec<&str> = vec![
                "root: 7",
                "\\__ left: 5",
                "    \\__ left: 3",
                "        \\__ left: NONE",
                "        \\__ right: NONE",
                "    \\__ right: NONE",
                "\\__ right: 15",
                "    \\__ left: 12",
                "        \\__ left: NONE",
                "        \\__ right: NONE",
                "    \\__ right: 18",
                "        \\__ left: NONE",
                "        \\__ right: NONE",
            ];
            let expected = lines.join("\n") + "\n";
            assert_eq!(output, expected);
        }


        //  ====================================================

        // init and insert
        let mut bstree: Bstree<i32> = Bstree::new();
        bstree.insert(10);
        bstree.insert(15);
        bstree.insert(5);
        bstree.insert(3);
        bstree.insert(7);
        bstree.insert(12);
        bstree.insert(18);

        assert!(bstree.exist(&10));
        assert!(bstree.exist(&15));
        assert!(bstree.exist(&5));
        assert!(bstree.exist(&3));
        assert!(bstree.exist(&7));
        assert!(bstree.exist(&12));
        assert!(bstree.exist(&18));
        assert!(!bstree.exist(&10086));

        // delete
        assert!(bstree.delete(&10));

        assert!(bstree.exist(&15));
        assert!(bstree.exist(&5));
        assert!(bstree.exist(&3));
        assert!(bstree.exist(&7));
        assert!(bstree.exist(&12));
        assert!(bstree.exist(&18));
    }

    // tests below are created through AI

    #[test]
    fn test_string_bst() {
        let mut bstree: Bstree<String> = Bstree::new();
        bstree.insert("apple".to_string());
        bstree.insert("banana".to_string());
        bstree.insert("cherry".to_string());
        bstree.insert("date".to_string());

        assert!(bstree.exist(&"apple".to_string()));
        assert!(bstree.exist(&"banana".to_string()));
        assert!(!bstree.exist(&"fig".to_string()));

        bstree.delete(&"banana".to_string());
        assert!(!bstree.exist(&"banana".to_string()));
        assert!(bstree.exist(&"apple".to_string()));
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct User {
        id: u32,
        name: String,
    }

    #[test]
    fn test_struct_bst() {
        let mut bstree = Bstree::new();
        let u1 = User { id: 1, name: "Alice".to_string() };
        let u2 = User { id: 2, name: "Bob".to_string() };
        let u3 = User { id: 3, name: "Charlie".to_string() };

        bstree.insert(u2); 
        bstree.insert(u1); 
        bstree.insert(u3); 

        assert!(bstree.exist(&User { id: 1, name: "Alice".to_string() }));
        assert!(!bstree.exist(&User { id: 4, name: "Dave".to_string() }));
    }

    #[test]
    fn test_complex_tree_structure() {
        let mut bstree = Bstree::new();
        // Insert in a way that creates a specific structure
        //      50
        //    /    \
        //   30     70
        //  /  \   /  \
        // 20  40 60  80
        let values = vec![50, 30, 70, 20, 40, 60, 80];
        for &v in &values {
            bstree.insert(v);
        }

        // Verify all exist
        for &v in &values {
            assert!(bstree.exist(&v));
        }

        // Delete a leaf
        bstree.delete(&20);
        assert!(!bstree.exist(&20));
        
        // Delete a node with one child (let's add one first)
        bstree.insert(90); // 80 -> 90
        bstree.delete(&80);
        assert!(!bstree.exist(&80));
        assert!(bstree.exist(&90));

        // Delete a node with two children
        // Let's re-insert 20
        bstree.insert(20);
        // Now 30 has 20 and 40.
        bstree.delete(&30);
        assert!(!bstree.exist(&30));
        // 30 should be replaced by max of left (20).
        assert!(bstree.exist(&20));
        assert!(bstree.exist(&40));

        // Delete root (50)
        bstree.delete(&50);
        assert!(!bstree.exist(&50));
        assert!(bstree.exist(&40));
        assert!(bstree.exist(&20));
        assert!(bstree.exist(&70));
        assert!(bstree.exist(&60));
        assert!(bstree.exist(&90));
    }
}

