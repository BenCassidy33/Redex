use anyhow::bail;

use crate::{
    LAMBDA_CHAR,
    node::{Node, node_ref::NodeRef, variable::Variable},
};

#[derive(Debug)]
pub struct Abstraction {
    bound: NodeRef<Node>,
    body: NodeRef<Node>,
}

impl Abstraction {
    pub fn parse_str(s: &str) -> anyhow::Result<Abstraction> {
        if !s.starts_with(LAMBDA_CHAR) {
            bail!("string does not start with a lambda character")
        }

        let Some(dot_idx) = s.find('.') else {
            bail!("string does not contain '.'")
        };

        let bound = Variable::parse_str(&s[1..dot_idx])?;

        if bound.len() != dot_idx - 1 {
            bail!("bound variable is not one single variable")
        }

        let body = Node::parse_str(&s[dot_idx + 1..])?;

        Ok(Abstraction {
            bound: NodeRef::new(Node::Variable(bound)),
            body: NodeRef::new(body),
        })
    }

    pub fn len(&self) -> usize {
        self.bound.borrow().len() + self.body.borrow().len() + 2
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
