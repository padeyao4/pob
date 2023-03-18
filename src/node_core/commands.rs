use std::{cell::RefCell, rc::Rc};

use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
pub struct LsCommand {}

pub struct Node {
    children: Option<Rc<RefCell<Vec<Node>>>>,
    parents: Option<Rc<RefCell<Vec<Node>>>>,
    date: u64,
    tag: String,
    name: String,
    content: String,
}
