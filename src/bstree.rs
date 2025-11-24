// pub struct BstreeNode<T> {
//     pub val: T,
//     pub left: Option<Box<BstreeNode<T>>>,
//     pub right: Option<Box<BstreeNode<T>>>,
// }




// 3 (indent 0)
// \__ 1 (indent 1)
//     \__ NONE (indent 2)
//     \__ 2
// \__ 4
//     \__ NONE
//     \__ 5
// 

// pub fn print_bstree<W: Write, T: std::fmt::Display>(writer: &mut W, root: &BstreeNode<T>, indent: usize, tag: &str) {
//     for _ in 0..indent.saturating_sub(1) {
//         write!(writer, "    ").unwrap();
//     }
//     if indent > 0 {
//         write!(writer, "\\__ ").unwrap();
//     }
//     writeln!(writer, "{}: {}", tag, root.val).unwrap();

//     if let Some(left_node) = &root.left {
//         print_bstree(writer, &left_node, indent + 1, "left");
//     } else {
//         for _ in 0..indent {
//             write!(writer, "    ").unwrap();
//         }
//         writeln!(writer, "\\__ left: NONE").unwrap();
//     }
//     if let Some(right_node) = &root.right {
//         print_bstree(writer, &right_node, indent + 1, "right");
//     } else {
//         for _ in 0..indent {
//             write!(writer, "    ").unwrap();
//         }
//         writeln!(writer, "\\__ right: NONE").unwrap();
//     }
// }

// pub fn print_bstree_stdout<T: std::fmt::Display>(root: &BstreeNode<T>, indent: usize, tag: &str) {
//     let stdout = std::io::stdout();
//     let mut handle = stdout.lock();
//     print_bstree(&mut handle, root, indent, tag);
// }

// pub fn insert_bstree_node<T: std::cmp::PartialOrd + std::fmt::Display>(root: &mut BstreeNode<T>, val: T) {
//     if val < root.val {
//         if let Some(left_node) = &mut root.left {
//             insert_bstree_node(left_node, val);
//         } else {
//             root.left = Some(create_bstree_node(val));
//         }
//     } else if val > root.val {
//         if let Some(right_node) = &mut root.right {
//             insert_bstree_node(right_node, val);
//         } else {
//             root.right = Some(create_bstree_node(val));
//         }
//     } else {
//         println!("Value {} already exists in the BSTree.", val);
//     }
// }

// pub fn value_exists_in_bstree<T: std::cmp::PartialOrd>(root: & BstreeNode<T>, val: T) -> bool{
// 	if val < root.val {
// 		if let Some(left) = &root.left {
// 			value_exists_in_bstree(left, val)
// 		} else {
// 			false
// 		}
// 	} else if val > root.val {
// 		if let Some(right) = &root.right {
// 			value_exists_in_bstree(right, val)
// 		} else {
// 			false
// 		}
// 	} else {
// 		true
// 	}
// }
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
        if let Some(n) = node{
            if val < &n.val {
                BstreeNode::in_sub_tree(&n.left, val)
            } else if val > &n.val {
                BstreeNode::in_sub_tree(&n.right, val)
            } else {
                true
            }
        } else {
            false
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
}

impl<T: std::cmp::Ord + Clone> BstreeNode<T> {
    pub fn delete(node: &mut Option<Box<BstreeNode<T>>>, val: &T) -> bool {
        if let Some(n) = node {
            if val < &n.val {
                BstreeNode::delete(&mut n.left, val)
            } else if val > &n.val {
                BstreeNode::delete(&mut n.right, val)
            } else {
                if n.left.is_none() && n.right.is_none() {
                    *node = None;
                    true
                } else if n.left.is_some() {
                    let mut cur = n.left.as_mut().unwrap().as_mut();
                    while cur.right.is_some() {
                        cur = cur.right.as_mut().unwrap().as_mut();
                    }

                    n.val = cur.val.clone();
                    cur.val = val.clone();
                    BstreeNode::delete(& mut n.left, val)
                } else {
                    let mut cur = n.right.as_mut().unwrap().as_mut();
                    while cur.left.is_some() {
                        cur = cur.left.as_mut().unwrap().as_mut();
                    }

                    n.val = cur.val.clone();
                    cur.val = val.clone();
                    BstreeNode::delete(& mut n.right, val)
                }
            }
        } else {
            false
        }
    }

    pub fn print_sub_tree<W: Write>(writer: &mut W, node: &Box<BstreeNode<T>>, indent: i32, tag: &str)
        where T: std::fmt::Display
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

    pub fn print_sub_tree_std(node: &Box<BstreeNode<T>>, indent: i32, tag: &str) where T: std::fmt::Display {
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
}

impl<T: std::cmp::Ord + Clone> Bstree<T> {
    pub fn delete(&mut self, val: &T) -> bool {
        BstreeNode::delete(&mut self.root, val)
    }

    pub fn print(&self) where T: std::fmt::Display {
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

        BstreeNode::print_sub_tree_std(root.as_ref().unwrap(), 0, "root");

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

        bstree.print();
    }
}