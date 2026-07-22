use crate::node::{Node, NodeRef};

pub struct Application {
    left: NodeRef<Node>,
    right: NodeRef<Node>
}

impl Application {
    pub fn parse_str(s: &str) -> Result<NodeRef<Application>, ()> {
        todo!()
    }
}
