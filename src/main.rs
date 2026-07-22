use redex::ast::expr::Statement;

fn main() {
    let statement = Statement::parse(
        r#"M := (Lx.x)yz
    qk
"#,
    ).unwrap();
}
