use std::fmt::Display;

use anyhow::bail;
use derive_more::{Constructor, IsVariant};

use crate::ast::{Node, node_ref::NodeRef, variable::Variable};

#[derive(Debug, Clone, Constructor)]
pub struct Assignment {
    pub(crate) bound: Variable,
    pub(crate) body: NodeRef<Node>,
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} := {}", self.bound, self.body)
    }
}

#[derive(Debug, Clone, IsVariant)]
pub enum Statement {
    Assignment(Assignment),
    Expression(NodeRef<Node>),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Assignment(assignment) => write!(f, "{}", assignment),
            Statement::Expression(node_ref) => write!(f, "{}", node_ref),
        }
    }
}

impl From<Statement> for Assignment {
    fn from(val: Statement) -> Self {
        let Statement::Assignment(assign) = val else {
            panic!("Statement is not an assignment")
        };
        assign
    }
}

impl From<&Statement> for Assignment {
    fn from(val: &Statement) -> Self {
        let Statement::Assignment(assign) = val else {
            panic!("Statement is not an assignment")
        };
        assign.clone()
    }
}

impl From<&Statement> for NodeRef<Node> {
    fn from(val: &Statement) -> Self {
        let Statement::Expression(expr) = val else {
            panic!("Statement is not an assignment")
        };
        expr.clone()
    }
}

impl From<Statement> for NodeRef<Node> {
    fn from(val: Statement) -> Self {
        let Statement::Expression(expr) = val else {
            panic!("Statement is not an assignment")
        };
        expr
    }
}

#[allow(unused)]
pub struct Unzipped {
    pub(crate) assignments: Vec<Assignment>,
    pub(crate) expressions: Vec<NodeRef<Node>>,
}

#[allow(unused)]
pub trait Unzip: Sized + IntoIterator<Item = Statement> {
    fn unzip(self) -> Unzipped {
        let (a, e): (Vec<_>, Vec<_>) = self.into_iter().partition(|n| n.is_assignment());

        Unzipped {
            assignments: a.iter().map(Into::into).collect(),
            expressions: e.iter().map(Into::into).collect(),
        }
    }
}

impl Unzip for Vec<Statement> {}

impl Statement {
    pub fn new_assignment(bound: Variable, body: NodeRef<Node>) -> Self {
        Self::Assignment(Assignment::new(bound, body))
    }

    pub fn make_lines(s: &str) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        let mut iter = s.lines().peekable();

        while let Some(line) = iter.next() {
            let mut line = line.to_string();
            while let Some(next_line) = iter.peek()
                && next_line.starts_with(|c: char| c.is_whitespace())
            {
                line += iter.next().unwrap().trim();
            }
            v.push(line);
        }

        v
    }

    pub fn parse(s: &str) -> anyhow::Result<Vec<Statement>> {
        let lines = Self::make_lines(s);

        let mut v = Vec::new();
        for line in lines {
            if line.is_empty() {
                continue;
            }
            if line.contains(":=") {
                let (bound, body) = line.split_once(":=").unwrap();

                if bound.is_empty() || body.is_empty() {
                    bail!("Invalid input for variable assignment!")
                }

                let bound = Variable::parse_str(bound.trim())?;
                let body = Node::parse_str(body.trim())?;

                v.push(Statement::new_assignment(bound, NodeRef::new(body)));
                continue;
            }

            let body = Node::parse_str(line.trim())?;
            v.push(Statement::Expression(NodeRef::new(body)));
        }

        Ok(v)
    }

    pub fn reduce(&self) {
        if let Statement::Expression(node) = self {
            Node::reduce(node, None, None)
        }
    }
}
