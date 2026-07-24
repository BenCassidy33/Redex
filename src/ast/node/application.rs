use std::fmt::Display;

use anyhow::bail;

use crate::{
    LAMBDA_CHAR,
    ast::{Node, NodeVariant, abstraction::Abstraction, node_ref::NodeRef, variable::Variable},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Application {
    pub(crate) left: NodeRef<Node>,
    pub(crate) right: NodeRef<Node>,
}

impl Application {
    pub fn parse_str(s: &str) -> anyhow::Result<Application> {
        if s.starts_with(LAMBDA_CHAR) {
            let left = Abstraction::parse_str(s, true)?;
            let right = Node::parse_str(&s[left.len()..])?;

            return Ok(Application {
                left: NodeRef::new(Node::Abstraction(left)),
                right: NodeRef::new(right),
            });
        }

        if let Ok(v) = Variable::parse_str(s) {
            if v.len() == s.len() {
                bail!("Variable '{}' matches the length of the string!", v);
            }

            let right = Node::parse_str(&s[v.len()..])?;
            return Ok(Application {
                left: NodeRef::new(Node::Variable(v)),
                right: NodeRef::new(right),
            });
        }

        bail!("Invalid Input for Application: {s}")
    }

    pub fn len(&self) -> usize {
        self.left.borrow().len() + self.right.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn reduce_with(ap: &NodeRef<Node>, with: NodeRef<Node>) {
        let Node::Application(app) = &*ap.borrow() else {
            return;
        };

        match ap.variant() {
            NodeVariant::Application => todo!(),
            NodeVariant::Abstraction => Abstraction::reduce(ap, None, with),
            NodeVariant::Variable => (),
        }
    }

    // TODO: Redo 
    pub fn reduce_self(ap: &NodeRef<Node>) {
        let swap = {
            let Node::Application(app) = &*ap.borrow() else {
                return;
            };

            if app.left.variant() == NodeVariant::Application
                && app.right.variant() == NodeVariant::Application
            {
                Node::reduce(&app.left, None, Some(app.right.clone()));
            }

            if app.left.variant() == NodeVariant::Abstraction {
                Abstraction::reduce(&app.left, None, app.right.clone());
                Some(app.left.clone())
            } else if app.right.variant() == NodeVariant::Abstraction {
                Abstraction::reduce(&app.right, None, app.left.clone());
                Some(app.right.clone())
            } else {
                Node::reduce(&app.left, None, Some(app.right.clone()));
                None
            }
        };

        if let Some(swap) = swap {
            let value = swap.borrow().clone();
            ap.replace(value);
        }
    }
}

impl Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() && self.len() > 2 {
            write!(f, "{:#}({:#})", self.left, self.right)
        } else if self.len() > 2 {
            write!(f, "{}({})", self.left, self.right)
        } else {
            write!(f, "{}{}", self.left, self.right)
        }
    }
}
