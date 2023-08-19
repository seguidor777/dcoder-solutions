use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::rc::Rc;

#[derive(Clone, Debug)]
enum NodePos {
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    value: i16,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn find_node(&self, value: i16) -> (Option<Rc<RefCell<Node>>>, Vec<NodePos>) {
        // DFS recursive algorithm
        let mut path = Vec::new();

        if let Some(ref left) = self.left {
            path.push(NodePos::Left);

            if left.borrow().value == value {
                return (Some(left.clone()), path);
            }

            if let (Some(node), sub_path) = left.borrow().find_node(value) {
                path.extend(sub_path);

                return (Some(node), path);
            }

            path.pop();
        }

        if let Some(ref right) = self.right {
            path.push(NodePos::Right);

            if right.borrow().value == value {
                return (Some(right.clone()), path);
            }

            if let (Some(node), sub_path) = right.borrow().find_node(value) {
                path.extend(sub_path);

                return (Some(node), path);
            }

            path.pop();
        }

        (None, path)
    }

    fn insert(&mut self, value: i16, node_pos: NodePos) {
        let child_node = Some(Rc::from(RefCell::from(Node {
            value: value,
            left: None,
            right: None,
        })));

        match node_pos {
            NodePos::Left => self.left = child_node,
            NodePos::Right => self.right = child_node,
        }
    }
}

#[derive(Debug)]
struct Tree {
    root: Rc<RefCell<Node>>,
}

impl Tree {
    fn new(value: i16) -> Self {
        Self {
            root: Rc::new(RefCell::new(Node {
                value,
                left: None,
                right: None,
            })),
        }
    }

    fn find_node(&self, value: i16) -> (Option<Rc<RefCell<Node>>>, Vec<NodePos>) {
        if self.root.borrow().value == value {
            return (Some(self.root.clone()), vec![]);
        }

        self.root.borrow().find_node(value)
    }

    fn find_mirror(&self, path: &Vec<NodePos>) -> Option<Rc<RefCell<Node>>> {
        let mut node = Some(self.root.clone());

        for node_pos in path {
            match node {
                Some(n) => {
                    match node_pos {
                        NodePos::Left => node = n.borrow().right.clone(),
                        NodePos::Right => node = n.borrow().left.clone(),
                    }
                }
                None => return None
            }
        }

        node
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap().expect("cannot read line");
    let mut params = line.split_whitespace();
    let n: usize = params.next().unwrap().parse().expect("cannot parse n");
    let q: usize = params.next().unwrap().parse().expect("cannot parse q");
    let x: i16 = params.next().unwrap().parse().expect("cannot parse x");
    let tree = Tree::new(x);
    let mut paths = HashMap::new();

    paths.insert(x, vec![]);

    for _ in 0..n {
        let line = lines.next().unwrap().expect("cannot read line");
        let mut params = line.split_whitespace();
        let parent: i16 = params.next().unwrap().parse().expect("cannot parse parent");
        let child: i16 = params.next().unwrap().parse().expect("cannot parse child");
        let pos: char = params.next().unwrap().parse().expect("cannot parse pos");
        let node_pos = match pos {
            'L' => NodePos::Left,
            'R' => NodePos::Right,
            _ => panic!()
        };

        if let (Some(parent_node), mut path) = tree.find_node(parent) {
            parent_node.borrow_mut().insert(child, node_pos.clone());
            path.push(node_pos);
            paths.insert(child, path);
        }
    }

    for _ in 0..q {
        let line = lines.next().unwrap().expect("cannot read line");
        let query: i16 = line.parse().expect("cannot parse query");

        match tree.find_mirror(&paths[&query]) {
            Some(mirror) => println!("{}", mirror.borrow().value),
            None => println!("Not Exist"),
        }
    }
}
