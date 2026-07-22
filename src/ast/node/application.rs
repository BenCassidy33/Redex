use std::fmt::Display;

use anyhow::bail;

use crate::{
    LAMBDA_CHAR,
    ast::{Node, node_ref::NodeRef, NodeVariant, abstraction::Abstraction, variable::Variable},
    utils::group_by_delim,
};

#[derive(Debug, Clone)]
pub struct Application {
    pub(crate) left: NodeRef<Node>,
    pub(crate) right: NodeRef<Node>,
}

impl Application {
    pub fn parse_str(s: &str) -> anyhow::Result<Application> {
        if s.starts_with("(") {
            let Ok(groups) = group_by_delim(s, '(', ')') else {
                bail!("No closing ')' found!")
            };

            if groups.is_empty() {
                bail!("Invalid application input!")
            }

            if groups.len() == 1 {
                return Application::parse_str(groups[0]);
            }

            // group into right
            let left = NodeRef::new(Node::parse_str(groups[0])?);
            let right = NodeRef::new(Node::parse_str(&s[groups[0].len() + 2..])?);

            return Ok(Application { left, right });
        }

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
                bail!("Variable matches the length of the string!");
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

    pub fn reduce_self(ap: &NodeRef<Node>) {
        let mut swap = None;

        {
            let Node::Application(app) = &*ap.borrow() else {
                unreachable!()
            };

            if app.left.variant() == NodeVariant::Abstraction {
                Abstraction::reduce(&app.left, None, app.right.clone());
                swap = Some(app.left.clone());
            } else if app.left.variant() == NodeVariant::Application {
                Application::reduce_self(&app.left);
                Application::reduce_self(&app.right);
            };
        }

        if let Some(swap) = swap {
            ap.swap(&swap);
        }
    }
}

impl Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})({})", self.left, self.right)
    }
}
