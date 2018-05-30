use std::str;
use std::borrow::Cow;

use nom::{alphanumeric, IResult};

use error::EdoError;

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Function {
        name: Cow<'a, str>,
        arguments: Vec<Cow<'a, str>>,
    },
    Literal(Cow<'a, str>),
}

// Parse a list of arguments
// TODO: allow non alphanumeric values inside arguments
// TODO: allow trailing commas, allow leading and trailing whitespace
named!(arguments<&[u8], Vec<&str> >, delimited!(
    char!('('),
    separated_list!(
        terminated!(
            char!(','),
            many0!(char!(' '))
        ),
        map_res!(
            alphanumeric,
            str::from_utf8
        )
    ),
    char!(')')
));

// Parse a function
named!(function<&[u8], Expression>, chain!(
    tag!("{") ~
    // Parse until the function ends or the arguments start
    name: map_res!(
        alt!(
            take_until!("(") |
            take_until!("}")
        ),
        str::from_utf8
    ) ~
    // Optionally parse a list of arguments
    args: arguments? ~
    tag!("}") ,
    || { Expression::Function {
        name: name.into(),
        arguments: args.unwrap_or(vec![]).into_iter().map(|v| v.into()).collect(),
    }}
));

fn tocow<'a>(s: &'a [u8]) -> Result<Cow<'a, str>, str::Utf8Error> {
    str::from_utf8(s)
        .and_then(|v| Ok(v.into()))
}

// Parse a literal
named!(literal<&[u8], Expression>, map!(
    map_res!(
        is_not!("{"),
        tocow
    ),
    Expression::Literal
));

// Parse multiple functions and text literals
named!(pub expressions<&[u8], Vec<Expression> >, many0!(alt!(
    function |
    literal
)));

/// Parse a template into a vector of expressions
pub fn parse<'a>(input: &'a str) -> Result<Vec<Expression<'a>>, EdoError> {
    match expressions(input.as_bytes()) {
        IResult::Done(_, expressions) => Ok(expressions),
        IResult::Error(_) =>
            Err(EdoError::ParsingError),
        IResult::Incomplete(_) =>
            Err(EdoError::ParsingError),
    }
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    use super::{
        Expression,
        arguments,
        function,
        literal,
        expressions,
        parse
    };

    #[test]
    fn parse_arguments() {
        assert_eq!(
            arguments(b"()"),
            IResult::Done(
                &b""[..],
                vec![]
            )
        );

        assert_eq!(
            arguments(b"(test)"),
            IResult::Done(
                &b""[..],
                vec!["test"]
            )
        );

        assert_eq!(
            arguments(b"(test,test2)"),
            IResult::Done(
                &b""[..],
                vec!["test", "test2"]
            )
        );

        assert_eq!(
            arguments(b"(test, test2)"),
            IResult::Done(
                &b""[..],
                vec!["test", "test2"]
            )
        );
    }

    #[test]
    fn parse_function() {
        assert_eq!(
            function(b"{test}"),
            IResult::Done(
                &b""[..],
                Expression::Function {
                    name: "test".into(),
                    arguments: vec![],
                }
            )
        );

        assert_eq!(
            function(b"{test()}"),
            IResult::Done(
                &b""[..],
                Expression::Function {
                    name: "test".into(),
                    arguments: vec![],
                }
            )
        );

        assert_eq!(
            function(b"{test(1, 2, 3)}"),
            IResult::Done(
                &b""[..],
                Expression::Function {
                    name: "test".into(),
                    arguments: vec!["1".into(), "2".into(), "3".into()],
                }
            )
        );
    }

    #[test]
    fn parse_literal() {
        assert_eq!(
            literal(b"testing"),
            IResult::Done(
                &b""[..],
                Expression::Literal("testing".into())
            )
        );
    }

    #[test]
    fn parse_multiple_expressions() {
        assert_eq!(
            expressions(b"{test}literal{test2}haha"),
            IResult::Done(
                &b""[..],
                vec![
                    Expression::Function {
                        name: "test".into(),
                        arguments: vec![],
                    },
                    Expression::Literal("literal".into()),
                    Expression::Function {
                        name: "test2".into(),
                        arguments: vec![],
                    },
                    Expression::Literal("haha".into()),
                ]
            )
        );
        
        assert_eq!(
            expressions(b"haha{test}"),
            IResult::Done(
                &b""[..],
                vec![
                    Expression::Literal("haha".into()),
                    Expression::Function {
                        name: "test".into(),
                        arguments: vec![],
                    },
                ]
            )
        );
    }

    #[test]
    fn parse_method() {
        assert_eq!(
            parse("haha{test(a, b, c)}".into()),
            Ok(vec![
                Expression::Literal("haha".into()),
                Expression::Function {
                    name: "test".into(),
                    arguments: vec!["a".into(), "b".into(), "c".into()],
                },
            ])
        );
    }
}
