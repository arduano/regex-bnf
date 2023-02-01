# regex-bnf

A macro-based BNF style parser for easier grammar definition.

This approach is useful when you need to parse a complex grammar without tokenizing first, e.g. where tokens may contain spaces and newlines and have complex rules surrounding them.

Here is a simple example CSV parser:

```rs
use regex_bnf::*;

bnf! {
    Value = <!Eof> <!NewLine> val:r"([^,\r\n]|\\,)*" <?Comma>;

    Line = <!Eof> values:<[Value]> <LineEnd>;
    Document = lines:<[Line]^>;

    Comma = ",";
    NewLine = r"[\r\n]+";
    Eof = ^;
    enum LineEnd = [NewLine | Eof];
}
```

The above macro creates a struct for each token (`Value`, `Line`, `Document`, etc.) and an enum for `LineEnd`. Each struct and enum contains a parse function that takes in a StringParser and returns a Result with `(parsed value, remaining string)` or an error.

## NOTE

This implementation is entirely deterministic, and succeptable to deadlocks including infite loops and stack overflows. In order to debug it, read it linearly as each struct parses in the order of declaration. So in the CSV example above, `LineEnd` would first try to parse `NewLine` and then `Eof`. If it fails to parse either, it will return an error.

## Syntax

There are 2 types of declarations:

- Tags: are a list of tokens that includes literals, regexes, or other token types, which are parsed linearly.
- Enums: are a list of tags, where each one is attempted to be parsed, and if an error is returned then the next one is attempted. If all error then an error is returned.

Within tag declarations, you can label the tokens with `label:<token>` to give them a field name within the generated struct, otherwise they are omitted.

Here are all the possible token types:

- String literal: `""`
- Regex: `r""`
- Tag: `<Tag>` (inline), `<*Tag>` (boxed, to avoid infinite size structs)
- Optional tag: `<?Tag>`
- Repeat tag: `<[Tag]*>` (zero or more times), `<[Tag]+>` (one or more times), `<[Tag]^>` (until the end of the string),
- Not tag: `<!Tag>` (fails if the tag is parsed successfully)
