use std::fmt::Display;

use anyhow::bail;

use crate::{LAMBDA_CHAR, group_by_delim, node::{Node, NodeRef, abstraction::Abstraction, variable::Variable}};

#[derive(Debug)]
pub struct Application {
    left: NodeRef<Node>,
    right: NodeRef<Node>
}

impl Application {
    pub fn parse_str(s: &str) -> anyhow::Result<Application> {
        if s.starts_with("(") {
            let Ok(groups) = group_by_delim(s, '(', ')') else {
                bail!("No closing ')' found!")
            };

            if groups.len() < 2 {
                bail!("Invalid applicaiton input!");
            }

            // group into right
            let left = NodeRef::new(Node::parse_str(groups[0])?);
            let right = NodeRef::new(Node::parse_str(&s[groups[0].len() + 2..])?);

            return Ok(Application {
                left,
                right
            });
        }

        if s.starts_with(LAMBDA_CHAR) {
            let left = Abstraction::parse_str(s, true)?;
            let right = Node::parse_str(&s[left.len()..])?;

            return Ok(Application {
                left: NodeRef::new(Node::Abstraction(left)),
                right: NodeRef::new(right)
            })
        }

        if let Ok(v) = Variable::parse_str(s) {
            if v.len() == s.len() {
                bail!("Variable matches the length of the string!");
            }

            let right = Node::parse_str(&s[v.len()..])?;
            return Ok(Application {
                left: NodeRef::new(Node::Variable(v)),
                right: NodeRef::new(right)
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
}

impl Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})({})", self.left, self.right)
    }
}
