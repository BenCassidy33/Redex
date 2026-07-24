use derive_more::IsVariant;

use crate::ast::Node;

#[derive(Debug, Clone, IsVariant)]
pub enum NumeralKind {
    Church
}

pub fn generate_numeral(number: usize, kind: NumeralKind) -> Node {
    match kind {
        NumeralKind::Church => generate_church_numeral(number)
    }
}

#[inline]
fn generate_church_numeral(number: usize) -> Node {
    let s = format!("Lf.Lx.{}x{}", "f(".repeat(number), ")".repeat(number));
    Node::parse_str(&s).unwrap()
}
