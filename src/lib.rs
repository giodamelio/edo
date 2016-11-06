#[macro_use]
extern crate nom;

mod parse;

use std::str;

use nom::IResult;

pub struct Edo {

}

impl Edo {
    pub fn new() -> Edo {
        Edo {
        }
    }

    fn parse<'a>(self, input: &'a str) -> IResult<&[u8], Vec<parse::Expression>> {
        let (_, parsed_expressions) = try_parse!(
            input.as_bytes(),
            parse::expressions
        );
        IResult::Done(&b""[..], parsed_expressions)
    }
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    use super::{Edo};
    use super::{parse};

    #[test]
    fn parse_method() {
        let edo = Edo::new();
        assert_eq!(
            edo.parse("haha{test}"),
            IResult::Done(
                &b""[..],
                vec![
                    parse::Expression::Literal("haha"),
                    parse::Expression::Function("test"),
                ]
            )
        );
    }
}
