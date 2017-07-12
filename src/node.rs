use std::rc::Rc;
use errors::Errors;
type Pair = (i32, char);

#[derive(Debug)]
pub struct Node {
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
    key: i32,
    value: char,
}

fn remove_child(node: &Rc<Node>) -> Option<Rc<Node>> {
    match *node.as_ref() {
        Node { left: None, right: None, .. } => {
            None
        },
        Node { left: None, right: Some(ref node), .. } | Node { left: Some(ref node), right: None, .. } => {
            Some(node.clone())
        },
        Node { left: Some(ref left), right: Some(ref right), .. } => {
            let &Node { key, value, ..} = right.get_smallest_child();
            Some(Rc::from(Node {
                key: key,
                value: value,
                left: Some(left.clone()),
                right: if key != right.key{
                    Some(Rc::from(right.remove(key).unwrap().unwrap()))
                } else {
                    None
                },
            }))
        }
    }
}

impl Node {
    pub fn new((key, value): Pair) -> Self {
        Node {
            left: None,
            right: None,
            key: key,
            value: value,
        }
    }

    fn new_with_children((key, value): Pair, left: Option<Rc<Node>>, right: Option<Rc<Node>>) -> Node {
        Node {
            left,
            right,
            key: key,
            value: value,
        }
    }

    pub fn add(&self, (key, value): Pair) -> Result<Node, Errors> {
        if key > self.key {
            if let Some(ref node) = self.right {
                Ok(Node::new_with_children((self.key, self.value), self.left.clone(), Some(Rc::from(node.add((key, value))?))))
            } else {
                Ok(Node::new_with_children((self.key, self.value), self.left.clone(), Some(Rc::from(Node::new((key, value))))))
            }
        } else if key < self.key {
            if let Some(ref node) = self.left {
                Ok(Node::new_with_children((self.key, self.value), Some(Rc::from(node.add((key, value))?)), self.right.clone()))
            } else {
                Ok(Node::new_with_children((self.key, self.value), Some(Rc::from(Node::new((key, value)))), self.right.clone()))
            }
        } else {
            Err(Errors::DuplicateKey)
        }
    }

    fn get_smallest_child(&self) -> &Node {
        if let Some(ref node) = self.left {
            node.get_smallest_child()
        } else {
            self
        }
    }

    pub fn get(&self, key: i32) -> Result<char, Errors> {
        if key > self.key {
            if let Some(ref node) = self.right {
                node.get(key)
            } else {
                Err(Errors::NoKeyFound)
            }
        } else if key < self.key {
            if let Some(ref node) = self.left {
                node.get(key)
            } else {
                Err(Errors::NoKeyFound)
            }
        } else {
            Ok(self.value)
        }
    }

    fn remove(&self, key: i32) -> Result<Option<Node>, Errors> {
        if key > self.key {
            if let Some(ref node) = self.right {
                let new_right = if node.key == key {
                   remove_child(node)
                } else {
                    node.remove(key)?.map(Rc::from)
                };
                Ok(Some(Node {
                    key: self.key,
                    value: self.value,
                    left: self.left.clone(),
                    right: new_right,
                }))
            } else {
                Err(Errors::NoKeyFound)
            }
        } else if key < self.key {
            if let Some(ref node) = self.left {
                let new_left = if node.key == key {
                    remove_child(node)
                } else {
                    node.remove(key)?.map(Rc::from)
                };
                Ok(Some(Node {
                    key: self.key,
                    value: self.value,
                    right: self.right.clone(),
                    left: new_left,
                }))
            } else {
                Err(Errors::NoKeyFound)
            }
        } else {
            Err(Errors::MissedStep)
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
        assert!(node.value == 'a' && node.key == 1);
    }

    #[test]
    fn add_bigger_child() {
        let node = Node::new((1, 'a'));
        let new_tree = node.add((2, 'b')).unwrap();
        assert!(new_tree.right.unwrap().value == 'b');
    }

    #[test]
    fn add_smaller_bigger() {
        let node = Node::new((4, 'a')).add((2, 'b')).unwrap().add((3, 'c')).unwrap();
        assert!(node.left.as_ref().unwrap().right.as_ref().unwrap().value == 'c');
    }

    #[test]
    fn add_bigger_smaller() {
        let node = Node::new((4, 'a')).add((9, 'b')).unwrap().add((7, 'c')).unwrap();
        assert!(node.right.as_ref().unwrap().left.as_ref().unwrap().value == 'c');
    }
    #[test]
    fn add_smaller_child() {
        let node = Node::new((2, 'a')).add((1, 'b')).unwrap();
        assert!(node.left.unwrap().value == 'b');
    }
    #[test]
    fn get_value() {
        let node = Node::new((2, 'a')).add((1, 'b')).unwrap();
        println!("{:?}", &node);
        assert!(node.get(1).unwrap() == 'b');
        assert!(node.get(12).is_err());
    }

    #[test]
    fn remove_leaf() {
        let node = Node::new((2, 'a')).add((1, 'b')).unwrap().remove(1).unwrap();
        assert!(node.unwrap().get(1).is_err());
    }
    #[test]
    fn remove_one_child_branch() {
        let node = Node::new((2, 'a')).add((1, 'b')).unwrap().add((0, 'c')).unwrap().remove(1).unwrap().unwrap();
        assert!(node.get(1).is_err());
        assert!(node.get(0).is_ok());
        assert!(node.left.as_ref().unwrap().value == 'c');
        assert!(node.add((1, 'b')).unwrap().remove(0).unwrap().unwrap().left.unwrap().value == 'b');
    }

    #[test]
    fn remove_two_child_branch() {
        let node = Node::new((2, 'a')).add((6, 'b')).unwrap().add((5, 'c')).unwrap().add((9, 'd')).unwrap();
        assert!(node.remove(6).unwrap().unwrap().right.as_ref().unwrap().value == 'd');
        assert!(node.add((8, 'e')).unwrap().remove(6).unwrap().unwrap().right.as_ref().unwrap().value == 'e');
    }
}
