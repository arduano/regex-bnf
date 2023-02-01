use regex_bnf::*;

bnf! {
    Operation = left:<SingleExpr> op:<Operator> right:<*Expr>;
    Operator = op:r"\+|\*";

    Number = val:r"[0-9]+";
    Group = "(" expr:<*Expr> ")";

    enum SingleExpr = [Number | Group];
    enum Expr = [Operation | Group | Number];
}

fn main() {
    let input = "1+(2*3)";
    let parser = StringParser::new(input);
    let result = Expr::parse(parser);

    match result {
        Ok(result) => println!("{:#?}", result.0),
        Err(err) => println!("Error: {}", err),
    }
}
