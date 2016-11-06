#[macro_use]
extern crate nom;

use std::str;

use nom::IResult;

pub struct Edo {

}

#[derive(Debug, PartialEq)]
enum Expression<'a> {
    Function(&'a str),
    Literal(&'a str),
}

// Parse a function
named!(function<&[u8], &str>, map_res!(
    delimited!(
        char!('{'),
        take_until!("}"),
        char!('}')
    ),
    str::from_utf8
));

// Parse multiple functions
named!(functions<&[u8], Vec<&str> >, many0!(function));

impl Edo {
    pub fn new() -> Edo {
        Edo {
        }
    }

    fn parse<'a>(self, input: &'a str) -> IResult<&[u8], Vec<Expression>> {
        let (_, parsed_functions) = try_parse!(input.as_bytes(), functions);
        let functions: Vec<Expression> = parsed_functions
            .iter()
            .map(|name| Expression::Function(name))
            .collect();
        IResult::Done(&b""[..], functions)
    }
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    use super::{Edo, Expression, function, functions};

    #[test]
    fn parse_function() {
        assert_eq!(
            function(b"{test}"),
            IResult::Done(
                &b""[..],
                "test"
            )
        );
    }

    #[test]
    fn parse_multiple_functions() {
        assert_eq!(
            functions(b"{test}{haha}"),
            IResult::Done(
                &b""[..],
                vec!["test", "haha"]
            )
        );
    }
}
