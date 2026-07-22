use std::fmt::Display;

use derive_more::IsVariant;

use crate::{LAMBDA_CHAR, node::{abstraction::Abstraction, application::Application, node_ref::NodeRef, variable::Variable}};

pub mod abstraction;
pub mod application;
pub mod variable;
mod node_ref;

#[derive(Debug, IsVariant)]
pub enum Node {
    Application(Application),
    Abstraction(Abstraction),
    Variable(Variable),
}

impl Node {
    pub fn parse_str(s: &str) -> anyhow::Result<Node> {
        if s.starts_with(LAMBDA_CHAR) {
            return Ok(Node::Abstraction(Abstraction::parse_str(s, true)?));
        }

        if let Ok(v) = Variable::parse_str(s) && v.len() == s.len() {
            return Ok(Node::Variable(v))
        }

        println!("calling Node::application with {}", s);
        Ok(Node::Application(Application::parse_str(s)?))
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Variable(v) => v.len(),
            Self::Abstraction(ab) => ab.len(),
            Self::Application(ap) => ap.len()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Abstraction(ab) => write!(f, "{}", ab),
            Self::Application(ap) => write!(f, "{}", ap),
            Self::Variable(v) => write!(f, "{}", v),
        }
    }
}
