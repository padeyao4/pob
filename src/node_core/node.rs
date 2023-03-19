use std::{cell::RefCell, rc::Rc};

pub struct Node {
    children: Option<Rc<RefCell<Vec<Node>>>>,
    parents: Option<Rc<RefCell<Vec<Node>>>>,
    timestamp: u64,
    tag: String,
    name: String,
    content: String,
}

pub struct Model {
    today: u64,
    current: u64,
    data: Vec<Node>,
}

impl Node {
    pub fn days(self: &Self) -> u64 {
        self.timestamp / 86400
    }
}
