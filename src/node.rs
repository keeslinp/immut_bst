#![allow(dead_code)]
use std::rc::Rc;
type Pair = (i32, char);

#[derive(Debug)]
pub struct Node {
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
    key: Option<i32>,
    value: Option<char>,
}

impl Node{
    pub fn new((key, value): Pair) -> Self {
        Node {
            left: None,
            right: None,
            key: Some(key),
            value: Some(value),
        }
    }

    pub fn new_with_children((key, value): Pair, left: Option<Rc<Node>>, right: Option<Rc<Node>>) -> Node {
        Node {
            left,
            right,
            key: Some(key),
            value: Some(value),
        }
    }

    pub fn add(&self, (key, value): Pair) -> Node {
        if Some(key) > self.key {
            if let &Some(ref node) = &self.right {
                Node::new_with_children((self.key.unwrap(), self.value.unwrap()), self.left.clone(), Some(Rc::from(node.add((key, value)))))
            } else {
                Node::new_with_children((self.key.unwrap(), self.value.unwrap()), self.left.clone(), Some(Rc::from(Node::new((key, value)))))
            }
        } else {
            if let &Some(ref node) = &self.left {
                Node::new_with_children((self.key.unwrap(), self.value.unwrap()), Some(Rc::from(node.add((key, value)))), self.right.clone())
            } else {
                Node::new_with_children((self.key.unwrap(), self.value.unwrap()), Some(Rc::from(Node::new((key, value)))), self.right.clone())
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use ::node::*;
    #[test]
    fn it_works() {
    }

    #[test]
    fn new_node() {
        let node = Node::new((1, 'a'));
        assert!(node.value == Some('a') && node.key == Some(1));
    }

    #[test]
    fn add_bigger_child() {
        let node = Node::new((1, 'a'));
        let new_tree = node.add((2, 'b'));
        assert!(new_tree.right.unwrap().value == Some('b'));
    }

    #[test]
    fn add_smaller_bigger() {
        let node = Node::new((4, 'a')).add((2, 'b')).add((3, 'c'));
        assert!(node.left.as_ref().unwrap().right.as_ref().unwrap().value == Some('c'));
    }

    #[test]
    fn add_bigger_smaller() {
        let node = Node::new((4, 'a')).add((9, 'b')).add((7, 'c'));
        assert!(node.right.as_ref().unwrap().left.as_ref().unwrap().value == Some('c'));
    }
    #[test]
    fn add_smaller_child() {
        let node = Node::new((2, 'a')).add((1, 'b'));
        assert!(node.left.unwrap().value == Some('b'));
    }
}
