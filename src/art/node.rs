#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub children: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Node<T>) {
        self.children.push(child);
    }
}