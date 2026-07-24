use std::{collections::HashSet, fmt::Display, rc::Rc};

use anyhow::bail;
use derive_more::IsVariant;

use crate::{
    LAMBDA_CHAR,
    ast::{
        abstraction::Abstraction, application::Application, assignments::Assignments,
        deep_clone::NodeDeepClone, node_ref::NodeRef, variable::Variable,
    },
    stdlib::{
        numerals::{self, NumeralKind},
    },
    utils::group_by_delim,
};

pub mod abstraction;
pub mod application;
pub mod assignments;
pub mod deep_clone;
pub mod node_ref;
pub mod variable;

#[derive(Debug, IsVariant, Clone, PartialEq)]
pub enum NodeVariant {
    Application,
    Abstraction,
    Variable,
}

impl From<&NodeRef<Node>> for NodeVariant {
    fn from(value: &NodeRef<Node>) -> Self {
        Into::<NodeVariant>::into(&*value.borrow())
    }
}

impl From<&Node> for NodeVariant {
    fn from(value: &Node) -> Self {
        match value {
            Node::Application(_) => Self::Application,
            Node::Abstraction(_) => Self::Abstraction,
            Node::Variable(_) => Self::Variable,
        }
    }
}

#[derive(Debug, IsVariant, Clone, PartialEq)]
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

        if let Ok(v) = Variable::parse_str(s)
            && v.len() == s.len()
        {
            return Ok(Node::Variable(v));
        }

        if s.starts_with("(") {
            let Ok(groups) = group_by_delim(s, '(', ')') else {
                bail!("No closing ')' found!")
            };

            if groups.is_empty() {
                bail!("Invalid application input!")
            }

            if groups.len() == 1 {
                return Node::parse_str(groups[0]);
            }

            // group into right
            let left = NodeRef::new(Node::parse_str(groups[0])?);
            let right = NodeRef::new(Node::parse_str(&s[groups[0].len() + 2..])?);

            return Ok(Node::Application(Application { left, right }));
        }

        Ok(Node::Application(Application::parse_str(s)?))
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Variable(v) => v.len(),
            Self::Abstraction(ab) => ab.len(),
            Self::Application(ap) => ap.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn substitute(node: &NodeRef<Self>, var: &NodeRef<Variable>, replacement: NodeRef<Node>) {
        let Some((replace, with)) = (match &*node.borrow() {
            Node::Variable(v) => {
                if *v == *var.borrow() {
                    Some((node, replacement))
                } else {
                    None
                }
            }

            Node::Application(ap) => {
                ap.left.substitute(var, replacement.clone());
                ap.right.substitute(var, replacement.clone());

                None
            }

            Node::Abstraction(ab) => {
                if *ab.bound.borrow() != *var.borrow() {
                    ab.body.substitute(var, replacement.clone());
                }

                if let Node::Variable(bv) = &*ab.body.borrow()
                    && *bv == *var.borrow()
                {
                    Some((node, replacement))
                } else {
                    None
                }
            }
        }) else {
            return;
        };

        replace.replace(with.borrow().clone());
    }

    pub fn substitute_assignments(
        node: &NodeRef<Node>,
        assignments: &Assignments,
        no_replace: Option<&NodeRef<Variable>>,
    ) {
        let Some((replace, with)) = (match &*node.borrow() {
            Node::Variable(v) => {
                if let Ok(n) = v.parse_to_number() {
                    Some((
                        node,
                        &NodeRef::new(numerals::generate_numeral(n, NumeralKind::Church)),
                    ))
                } else if let Some(rep) = no_replace
                    && *v == *rep.borrow()
                {
                    None
                } else {
                    assignments.get(v).map(|body| (node, body))
                }
            }

            Node::Application(ap) => {
                Node::substitute_assignments(&ap.left, assignments, no_replace);
                Node::substitute_assignments(&ap.right, assignments, no_replace);

                None
            }

            Node::Abstraction(ab) => {
                Node::substitute_assignments(&ab.body, assignments, Some(&ab.bound));

                None
            }
        }) else {
            return;
        };

        replace.replace(with.borrow().clone());
    }

    pub fn reduce(
        node: &NodeRef<Self>,
        var: Option<&NodeRef<Variable>>,
        replacement: Option<NodeRef<Self>>,
    ) {
        match node.variant() {
            NodeVariant::Abstraction => {
                if let Some(with) = replacement {
                    Abstraction::reduce(node, var, with);
                } else {
                    Abstraction::reduce_self(node);
                }
            }

            NodeVariant::Application => {
                if let Some(var) = var
                    && let Some(replacement) = replacement
                {
                    Node::substitute(node, var, replacement);
                } else {
                    Application::reduce_self(node);
                }
            }

            NodeVariant::Variable => {
                if let Some(var) = var
                    && let Some(replacement) = replacement
                {
                    Node::substitute(node, var, replacement);
                }
            }
        }
    }

    #[inline]
    pub fn variant(node: &NodeRef<Self>) -> NodeVariant {
        node.into()
    }

    #[inline]
    pub fn deep_clone(node: &NodeRef<Node>) -> NodeDeepClone {
        NodeDeepClone::deep_clone_from(node.clone())
    }

    pub fn has_cycle(node: &NodeRef<Node>, seen: &mut HashSet<*const ()>) -> bool {
        let ptr = Rc::as_ptr(&node.inner) as *const ();
        if !seen.insert(ptr) {
            return true;
        }

        match &*node.borrow() {
            Node::Application(ap) => Node::has_cycle(&ap.left, seen) || Node::has_cycle(&ap.right, seen),
            Node::Abstraction(ab) => Node::has_cycle(&ab.body, seen),
            Node::Variable(_) => false,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self {
                Self::Abstraction(ab) => write!(f, "{:#}", ab),
                Self::Application(ap) => write!(f, "{:#}", ap),
                Self::Variable(v) => write!(f, "{:#}", v),
            }
        } else {
            match self {
                Self::Abstraction(ab) => write!(f, "{}", ab),
                Self::Application(ap) => write!(f, "{}", ap),
                Self::Variable(v) => write!(f, "{}", v),
            }
        }
    }
}
