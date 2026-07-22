use redex::node::Node;

fn main() {
    dbg!(Node::parse_str("Labc.abc").unwrap().to_string());
}
