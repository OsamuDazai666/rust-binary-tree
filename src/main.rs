#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: std::cmp::PartialOrd,
{
    fn new(init: T) -> Node<T> {
        Node {
            value: init,
            left: None,
            right: None,
        }
    }
    fn insert_left(&self, new_val: T) -> Option<Box<Node<T>>> {
        Some(Box::new(Node::new(new_val)))
    }
    fn insert_right(&self, new_val: T) -> Option<Box<Node<T>>> {
        Some(Box::new(Node::new(new_val)))
    }

    fn append(&mut self, new_value: T) {
        if new_value < self.value {
            match self.left {
                None => {
                    self.left = self.insert_left(new_value);
                }
                Some(ref mut left_child) => {
                    left_child.append(new_value);
                }
            }
        } else {
            match self.right {
                None => {
                    self.right = self.insert_right(new_value);
                }
                Some(ref mut right_child) => {
                    right_child.append(new_value);
                }
            }
        }
    }
}

fn main() {
    let mut node_a = Node::new(69);

    node_a.append(20);
    node_a.append(19);
    node_a.append(71);
    node_a.append(90);

    println!("{node_a:#?}");
}
