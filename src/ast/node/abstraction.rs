use std::{fmt::Display};

use anyhow::bail;

use crate::{
    LAMBDA_CHAR, ast::{Node, NodeVariant, node_ref::NodeRef, variable::Variable},
};

#[derive(Debug, Clone)]
pub struct Abstraction {
    pub(crate) bound: NodeRef<Variable>,
    pub(crate) body: NodeRef<Node>,

    pub(crate) was_curried: bool,
}

impl Abstraction {
    pub fn parse_str(s: &str, starts_with_lambda: bool) -> anyhow::Result<Abstraction> {
        if !s.starts_with(LAMBDA_CHAR) && starts_with_lambda {
            bail!("string does not start with a lambda character")
        }

        let Some(dot_idx) = s.find('.') else {
            bail!("string does not contain '.'")
        };

        let start = if starts_with_lambda { 1 } else { 0 };
        let bound = Variable::parse_str(&s[start..dot_idx + start])?;

        // currying
        if bound.len() + start < dot_idx {
            let body = Abstraction::parse_str(&s[bound.len() + start..], false)?;

            return Ok(Abstraction {
                bound: NodeRef::new(bound),
                body: NodeRef::new(Node::Abstraction(body)),
                was_curried: true,
            });
        }

        let body = Node::parse_str(&s[dot_idx + 1..])?;

        Ok(Abstraction {
            bound: NodeRef::new(bound),
            body: NodeRef::new(body),
            was_curried: false,
        })
    }

    pub fn len(&self) -> usize {
        self.bound.borrow().len() + self.body.borrow().len() + 2
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn reduce_self(ab: &NodeRef<Node>) {
        let Node::Abstraction(ab) = &*ab.borrow() else {
            unreachable!()
        };

        match ab.body.variant() {
            NodeVariant::Application => todo!(),
            NodeVariant::Abstraction => Abstraction::reduce_self(&ab.body.clone()),
            NodeVariant::Variable => todo!(),
        }
    }

    pub fn reduce(ab: &NodeRef<Node>, bound: Option<&NodeRef<Variable>>, replacement: NodeRef<Node>) {
        let swap = match bound {
            Some(bound) => {
                ab.substitute(bound, replacement);
                None
            }

            None => {
                let Node::Abstraction(ab) = &*ab.borrow() else { unreachable!() };
                ab.body.substitute(&ab.bound, replacement);
                Some(ab.body.clone())
            }
        };

        if let Some(swap) = swap {
            ab.swap(&swap);
        }
    }
}

impl Display for Abstraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.was_curried {
            let Node::Abstraction(ab) = &*self.body.borrow() else {
                unreachable!()
            };
            write!(f, "{}{}{}.{}", LAMBDA_CHAR, self.bound, ab.bound, ab.body)
        } else {
            write!(f, "{}{}.{}", LAMBDA_CHAR, self.bound, self.body)
        }
    }
}
