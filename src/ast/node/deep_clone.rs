use std::fmt::Display;

use crate::{
    LAMBDA_CHAR,
    ast::{
        Node, NodeVariant, abstraction::Abstraction, application::Application, node_ref::NodeRef,
        variable::Variable,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeDeepClone {
    Application(Box<ApplicationDeepClone>),
    Abstraction(Box<AbstractionDeepClone>),
    Variable(Box<VariableDeepClone>),
}

impl NodeDeepClone {
    pub fn deep_clone_from(node: NodeRef<Node>) -> NodeDeepClone {
        match node.variant() {
            NodeVariant::Application => {
                NodeDeepClone::Application(ApplicationDeepClone::deep_clone_from(node))
            }

            NodeVariant::Abstraction => {
                NodeDeepClone::Abstraction(AbstractionDeepClone::deep_clone_from(node))
            }

            NodeVariant::Variable => {
                NodeDeepClone::Variable(VariableDeepClone::deep_clone_from(node))
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Variable(v) => v.len(),
            Self::Abstraction(ab) => ab.len(),
            Self::Application(ap) => ap.len(),
        }
    }
}

impl Display for NodeDeepClone {
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

impl PartialEq<Node> for NodeDeepClone {
    fn eq(&self, other: &Node) -> bool {
        self == other
    }
}

impl PartialEq<NodeRef<Node>> for NodeDeepClone {
    fn eq(&self, other: &NodeRef<Node>) -> bool {
        *self == *other.borrow()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationDeepClone {
    pub(crate) left: Box<NodeDeepClone>,
    pub(crate) right: Box<NodeDeepClone>,
}

impl ApplicationDeepClone {
    pub fn deep_clone_from(node: NodeRef<Node>) -> Box<Self> {
        match &*node.borrow() {
            Node::Application(ap) => Box::new(Self {
                left: Box::new(NodeDeepClone::deep_clone_from(ap.left.clone())),
                right: Box::new(NodeDeepClone::deep_clone_from(ap.right.clone())),
            }),
            _ => {
                unreachable!()
            }
        }
    }

    pub fn len(&self) -> usize {
        self.left.len() + self.right.len()
    }
}

impl Display for ApplicationDeepClone {
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

impl PartialEq<Application> for ApplicationDeepClone {
    fn eq(&self, other: &Application) -> bool {
        *self.left == other.left && *self.right == other.right
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractionDeepClone {
    pub(crate) bound: Box<VariableDeepClone>,
    pub(crate) body: Box<NodeDeepClone>,

    pub(crate) was_curried: bool,
}

impl AbstractionDeepClone {
    pub fn deep_clone_from(node: NodeRef<Node>) -> Box<Self> {
        match &*node.borrow() {
            Node::Abstraction(ab) => Box::new(Self {
                bound: VariableDeepClone::deep_clone_var_from(ab.bound.clone()),
                body: Box::new(NodeDeepClone::deep_clone_from(ab.body.clone())),
                was_curried: ab.was_curried.clone(),
            }),
            _ => unreachable!(),
        }
    }

    pub fn len(&self) -> usize {
        self.bound.len() + self.body.len() + 2
    }
}

impl Display for AbstractionDeepClone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            if self.was_curried {
                let NodeDeepClone::Abstraction(ref ab) = *self.body else {
                    unreachable!()
                };
                write!(f, "{}{}{}.{:#}", LAMBDA_CHAR, self.bound, ab.bound, ab.body)
            } else {
                write!(f, "{}{}.{:#}", LAMBDA_CHAR, self.bound, self.body)
            }
        } else {
            if self.was_curried {
                let NodeDeepClone::Abstraction(ref ab) = *self.body else {
                    unreachable!()
                };
                write!(f, "{}{}{}.{}", LAMBDA_CHAR, self.bound, ab.bound, ab.body)
            } else {
                write!(f, "{}{}.{}", LAMBDA_CHAR, self.bound, self.body)
            }
        }
    }
}

impl PartialEq<Abstraction> for AbstractionDeepClone {
    fn eq(&self, other: &Abstraction) -> bool {
        *self.bound == other.bound
            && *self.body == other.body
            && self.was_curried == other.was_curried
    }
}
impl PartialEq<NodeRef<Abstraction>> for AbstractionDeepClone {
    fn eq(&self, other: &NodeRef<Abstraction>) -> bool {
        let other = other.borrow();
        *self.bound == other.bound
            && *self.body == other.body
            && self.was_curried == other.was_curried
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct VariableDeepClone {
    pub(crate) ident: String,
    pub(crate) subtext: Option<String>,
}

impl VariableDeepClone {
    pub fn deep_clone_from(node: NodeRef<Node>) -> Box<Self> {
        match &*node.borrow() {
            Node::Variable(var) => Box::new(Self {
                ident: var.ident.clone(),
                subtext: var.subtext.clone(),
            }),

            _ => unreachable!(),
        }
    }

    pub fn deep_clone_var_from(node: NodeRef<Variable>) -> Box<Self> {
        let b = &*node.borrow();
        Box::new(Self {
            ident: b.ident.clone(),
            subtext: b.subtext.clone(),
        })
    }

    pub fn len(&self) -> usize {
        self.ident.len()
            + if let Some(ref st) = self.subtext {
                if st.len() != 1 {
                    st.len() + 3
                } else {
                    st.len() + 1
                }
            } else {
                0
            }
    }
}

impl Display for VariableDeepClone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.subtext {
            Some(sub) => {
                if sub.len() > 1 {
                    write!(f, "{}_{{{}}}", self.ident, sub)
                } else {
                    write!(f, "{}_{}", self.ident, sub)
                }
            }

            None => write!(f, "{}", self.ident),
        }
    }
}

impl PartialEq<Variable> for VariableDeepClone {
    fn eq(&self, other: &Variable) -> bool {
        self.ident == other.ident && self.subtext == other.subtext
    }
}

impl PartialEq<NodeRef<Variable>> for VariableDeepClone {
    fn eq(&self, other: &NodeRef<Variable>) -> bool {
        let other = other.borrow();
        self.ident == other.ident && self.subtext == other.subtext
    }
}
