#[macro_use]
extern crate nom;

mod error;
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

    // Parse a template into a vector of expressions
    fn parse<'a>(self, input: &'a str) -> Result<Vec<parse::Expression>, error::EdoError> {
        match parse::expressions(input.as_bytes()) {
            IResult::Done(_, expressions) => Ok(expressions),
            IResult::Error(err) =>
                Err(error::EdoError::ParsingError),
            IResult::Incomplete(needed) =>
                Err(error::EdoError::ParsingError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Edo};
    use super::{parse};

    #[test]
    fn parse_method() {
        let edo = Edo::new();
        assert_eq!(
            edo.parse("haha{test(a, b, c)}"),
            Ok(vec![
                parse::Expression::Literal("haha"),
                parse::Expression::Function {
                    name: "test",
                    arguments: vec!["a", "b", "c"],
                },
            ])
        );
    }
}
