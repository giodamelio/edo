use std::str;

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Function(&'a str),
    Literal(&'a str),
}

// Parse a function
named!(function<&[u8], Expression>, map!(
    map_res!(
        delimited!(
            char!('{'),
            take_until!("}"),
            char!('}')
        ),
        str::from_utf8
    ),
    Expression::Function
));

// Parse a literal
named!(literal<&[u8], Expression>, map!(
    map_res!(
        is_not!("{"),
        str::from_utf8
    ),
    Expression::Literal
));

// Parse multiple functions and text literals
named!(pub expressions<&[u8], Vec<Expression> >, many0!(alt!(
    function |
    literal
)));

#[cfg(test)]
mod tests {
    use nom::IResult;

    use super::{Expression, function, literal, expressions};

    #[test]
    fn parse_function() {
        assert_eq!(
            function(b"{test}"),
            IResult::Done(
                &b""[..],
                Expression::Function("test")
            )
        );
    }

    #[test]
    fn parse_literal() {
        assert_eq!(
            literal(b"testing"),
            IResult::Done(
                &b""[..],
                Expression::Literal("testing")
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
                    Expression::Function("test"),
                    Expression::Literal("literal"),
                    Expression::Function("test2"),
                    Expression::Literal("haha"),
                ]
            )
        );
        
        assert_eq!(
            expressions(b"haha{test}"),
            IResult::Done(
                &b""[..],
                vec![
                    Expression::Literal("haha"),
                    Expression::Function("test"),
                ]
            )
        );
    }
}
