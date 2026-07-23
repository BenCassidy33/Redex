use std::collections::HashMap;

use crate::ast::{Node, expr::Assignment, node_ref::NodeRef, variable::Variable};

pub type Assignments = HashMap<Variable, NodeRef<Node>>;

pub fn stdlib_assignment_map() -> Assignments {
    todo!()
}

pub fn insert_assignment(
    assignments: &mut Assignments,
    assignment: Assignment,
) -> Option<NodeRef<Node>> {
    assignments.insert(assignment.bound, assignment.body)
}
