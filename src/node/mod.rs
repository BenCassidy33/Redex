use derive_more::IsVariant;

use crate::node::{abstraction::Abstraction, node_ref::NodeRef, variable::Variable};

pub mod abstraction;
pub mod application;
pub mod variable;
mod node_ref;

#[derive(Debug, IsVariant)]
pub enum Node {
    Application,
    Abstraction(Abstraction),
    Variable(Variable),
}

impl Node {
    pub fn parse_str(s: &str) -> anyhow::Result<Node> {
        todo!()
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Variable(v) => v.len(),
            Self::Abstraction(ab) => ab.len(),
            _ => todo!(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
