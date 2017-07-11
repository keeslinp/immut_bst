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

    pub fn add(&self, (key, value): Pair) -> Node {
        if key > self.key {
            if let &Some(ref node) = &self.right {
                Node::new_with_children((self.key, self.value), self.left.clone(), Some(Rc::from(node.add((key, value)))))
            } else {
                Node::new_with_children((self.key, self.value), self.left.clone(), Some(Rc::from(Node::new((key, value)))))
            }
        } else {
            if let &Some(ref node) = &self.left {
                Node::new_with_children((self.key, self.value), Some(Rc::from(node.add((key, value)))), self.right.clone())
            } else {
                Node::new_with_children((self.key, self.value), Some(Rc::from(Node::new((key, value)))), self.right.clone())
            }
        }
    }

    pub fn get(&self, key: i32) -> Result<char, Errors> {
        if key > self.key {
            if let &Some(ref node) = &self.right {
                node.get(key)
            } else {
                Err(Errors::NoKeyFound)
            }
        } else if key < self.key {
            if let &Some(ref node) = &self.left {
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
            if let &Some(ref node) = &self.right {
                let new_right = if node.key == key {
                    None
                } else {
                    node.remove(key)?
                };
                Ok(Some(Node {
                    key: self.key,
                    value: self.value,
                    left: self.left.clone(),
                    right: new_right.map(|val| Rc::from(val)),
                }))
            } else {
                Err(Errors::NoKeyFound)
            }
        } else if key < self.key {
            if let &Some(ref node) = &self.left {
                let new_left = if node.key == key {
                    None
                } else {
                    node.remove(key)?
                };
                Ok(Some(Node {
                    key: self.key,
                    value: self.value,
                    right: self.right.clone(),
                    left: new_left.map(|val| Rc::from(val)),
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
        let new_tree = node.add((2, 'b'));
        assert!(new_tree.right.unwrap().value == 'b');
    }

    #[test]
    fn add_smaller_bigger() {
        let node = Node::new((4, 'a')).add((2, 'b')).add((3, 'c'));
        assert!(node.left.as_ref().unwrap().right.as_ref().unwrap().value == 'c');
    }

    #[test]
    fn add_bigger_smaller() {
        let node = Node::new((4, 'a')).add((9, 'b')).add((7, 'c'));
        assert!(node.right.as_ref().unwrap().left.as_ref().unwrap().value == 'c');
    }
    #[test]
    fn add_smaller_child() {
        let node = Node::new((2, 'a')).add((1, 'b'));
        assert!(node.left.unwrap().value == 'b');
    }
    #[test]
    fn get_value() {
        let node = Node::new((2, 'a')).add((1, 'b'));
        println!("{:?}", &node);
        assert!(node.get(1).unwrap() == 'b');
        assert!(node.get(12).is_err());
    }

    #[test]
    fn remove_leaf() {
        let node = Node::new((2, 'a')).add((1, 'b')).remove(1).unwrap();
        assert!(node.unwrap().get(1).is_err());
    }
}
