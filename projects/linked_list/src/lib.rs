pub struct LinkedList<T> {
    head: Option<Node<T>>,
    tail: Option<Node<T>>,
    size: usize,
}

impl<T> LinkedList<T> {
    fn new(data: T) -> LinkedList<T> {
        LinkedList {
            item: Node::new(data),
            next: None,
        }
    }
}

struct Node<T> {
    field: T,
    next: Option<&Node<T>>,
}

impl<T> Node<T> {
    fn new(field: T) -> Node<T> {
        Node { field, next: None }
    }

    fn field(&self) -> &T {
        &self.field
    }

    fn set_field(&mut self, field: T) {
        self.field = field;
    }
}
