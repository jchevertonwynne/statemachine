use std::rc::Rc;

#[derive(PartialEq, Eq, Clone)]
pub enum SharedList<T> {
    Empty,
    Valued { length: usize, node: Rc<Node<T>> },
}

impl<T> Default for SharedList<T> {
    fn default() -> Self {
        SharedList::new()
    }
}

impl<T> SharedList<T> {
    pub fn new() -> SharedList<T> {
        SharedList::Empty
    }

    pub fn push(&self, value: T) -> SharedList<T> {
        match self {
            SharedList::Empty => SharedList::Valued {
                length: 1,
                node: Rc::new(Node::new(value, None)),
            },
            SharedList::Valued { length, node } => SharedList::Valued {
                length: length + 1,
                node: Rc::new(Node::new(value, Some(Rc::clone(node)))),
            },
        }
    }

    pub fn len(&self) -> usize {
        match self {
            SharedList::Empty => 0,
            SharedList::Valued { length, .. } => *length,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Clone> From<SharedList<T>> for Vec<T> {
    fn from(s: SharedList<T>) -> Self {
        let mut res = Vec::with_capacity(s.len());

        if let SharedList::Valued { mut node, .. } = s {
            res.push(node.value.clone());
            while let Some(next_node) = &node.previous {
                res.push(next_node.value.clone());
                node = Rc::clone(next_node);
            }
        }

        res
    }
}

#[derive(PartialEq, Eq)]
pub struct Node<T> {
    value: T,
    previous: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, previous: Option<Rc<Node<T>>>) -> Node<T> {
        Node { value, previous }
    }
}
