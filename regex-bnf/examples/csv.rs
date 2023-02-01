use regex_bnf::*;

bnf! {
    Value = <!Eof> <!NewLine> val:r"([^,\r\n]|\\,)*" <?Comma>;

    Line = <!Eof> values:<[Value]+> <LineEnd>;
    Document = lines:<[Line]^>;

    Comma = ",";
    NewLine = r"[\r\n]+";
    Eof = ^;
    enum LineEnd = [NewLine | Eof];
}

fn main() {
    let input = "foo,bar,baz\n1,2,3";
    let result = Document::parse_str(&input);

    match result {
        Ok(result) => println!("{:#?}", result.0),
        Err(err) => println!("Error: {}", err),
    }
}
