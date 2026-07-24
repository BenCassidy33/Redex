use std::collections::HashMap;

use crate::ast::{
    Node,
    expr::{Assignment, Statement, Unzip},
    node_ref::NodeRef,
    variable::Variable,
};

pub type Assignments = HashMap<Variable, NodeRef<Node>>;

pub fn create_stdlib_assignment_map() -> Assignments {
    todo!()
}

pub fn insert_assignment(map: &mut Assignments, assignment: Assignment) -> Option<NodeRef<Node>> {
    map.insert(assignment.bound, assignment.body)
}

pub fn insert_many_assignments(
    map: &mut Assignments,
    assignments: Vec<Assignment>,
) -> Option<Vec<NodeRef<Node>>> {
    assignments
        .into_iter()
        .map(|a| insert_assignment(map, a))
        .collect()
}

