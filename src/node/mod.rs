use crate::node::variable::Variable;
pub mod variable;

pub struct ParsingError {

}

pub enum Node {
    Abstraction,
    Application,
    Variable(Variable)
}
