//! Edo is a VERY simple templating language. It allows you to register handlers that are executed when their matching names are found in the template.
//!
//! For example, with the template `"Hello {name}"`, the `name` handler would be executed and the string it returns would be substituted in place of the original `{name}`. Handler functions can also accept arguments, which are passed in as a `Vec<str>`.
//!
//! # Examples
//!
//! ### Static input
//! ```
//! use edo::Edo;
//! let mut template = Edo::new("Hello {name}").unwrap();
//! template.register_static("name", "World!");
//! let output = template.render();
//! assert_eq!(output, "Hello World!");
//! ```
//!
//! ### Simple Handler
//! ```
//! use edo::Edo;
//! let mut template = Edo::new("Hello {name}").unwrap();
//! template.register_handler("name", |_| String::from("World!"));
//! let output = template.render();
//! assert_eq!(output, "Hello World!");
//! ```
//!
//! ### Handler With Arguments 
//! ```
//! use edo::Edo;
//! let mut template = Edo::new("{say_hello(World)}").unwrap();
//! template.register_handler("say_hello", |args| format!("Hello {}", args[0]));
//! let output = template.render();
//! assert_eq!(output, "Hello World");
//! ```
#![deny(missing_docs)]

#[macro_use]
extern crate nom;

mod error;
mod parse;

use std::str;
use std::collections::HashMap;

use error::EdoError;
use parse::Expression;

enum ValueProducer<'a> {
    Handler(Box<Fn(Vec<&'a str>) -> String>),
    Static(String),
}

/// A single template. Allows registering of handlers and rendering
pub struct Edo<'a> {
    #[doc(hidden)]
    value_producers: HashMap<&'a str, ValueProducer<'a>>,
    template: Vec<Expression<'a>>,
}

impl<'a> Edo<'a> {
    /// Creates a new template instance.
    ///
    /// # Examples
    /// ```no_run
    /// # #![allow(unused_variables)]
    /// # use edo::Edo;
    /// let template = Edo::new("Hello {name}");
    /// ```
    pub fn new(template_string: &'a str) -> Result<Edo<'a>, EdoError> {
        Ok(Edo {
            value_producers: HashMap::new(),
            template: try!(parse::parse(template_string)),
        })
    }

    /// Register a new function handler
    ///
    /// # Examples
    /// ```no_run
    /// # #![allow(unused_variables)]
    /// # use edo::Edo;
    /// let mut template = Edo::new("Hello {name}").unwrap();
    /// template.register_handler("name", |_| String::from("World!"));
    /// ```
    pub fn register_handler<F>(&mut self, name: &'a str, handler: F) where
        F: 'static + Fn(Vec<&'a str>) -> String {
        self.value_producers.insert(name, ValueProducer::Handler(Box::new(handler)));
    }

    /// Register a static replacement
    ///
    /// # Examples
    /// ```no_run
    /// # #![allow(unused_variables)]
    /// # use edo::Edo;
    /// let mut template = Edo::new("Hello {name}").unwrap();
    /// template.register_static("name", "World!");
    /// ```
    pub fn register_static<S: Into<String>>(&mut self, name: &'a str, input: S) {
        self.value_producers.insert(name, ValueProducer::Static(input.into()));
    }

    /// Render template into a string
    ///
    /// # Examples
    /// ```
    /// # use edo::Edo;
    /// let mut template = Edo::new("Hello {name}").unwrap();
    /// template.register_handler("name", |_| String::from("World!"));
    /// let output = template.render();
    /// assert_eq!(output, "Hello World!");
    /// ```
    // TODO: add a strict mode that errors when there is no handler
    pub fn render(self) -> String {
        // Iterate over the template and
        // 1. Leave literals untouched
        // 2. Call the handlers for each function call and replace within the output
        self.template.iter()
            .map(|expression| match *expression {
                Expression::Literal(text) => String::from(text),
                Expression::Function { name, ref arguments } => {
                    match self.value_producers.get(name) {
                        None => String::from(""),
                        Some(value_producer) => match value_producer {
                            &ValueProducer::Handler(ref handler) => handler(arguments.clone()),
                            &ValueProducer::Static(ref value) => value.clone(),
                        },
                    }
                }
            })
            .collect::<Vec<String>>()
            .concat()
    }
}

#[cfg(test)]
mod tests {
    use super::Edo;

    #[test]
    fn create_new_edo() {
        let edo = Edo::new("Hello {name}");
        assert!(edo.is_ok());
    }

    #[test]
    fn register_handler() {
        let mut edo = match Edo::new("Hello {name}") {
            Ok(edo) => edo,
            Err(err) => panic!(err),
        };
        edo.register_handler("name", |_| String::from("World!"));
        assert!(edo.value_producers.get("name").is_some());
    }

    #[test]
    fn register_static() {
        let mut edo = match Edo::new("Hello {name}") {
            Ok(edo) => edo,
            Err(err) => panic!(err),
        };
        edo.register_static("name", "World!");
        assert!(edo.value_producers.get("name").is_some());
    }

    #[test]
    fn render_template() {
        let mut edo = match Edo::new("Hello {name}") {
            Ok(edo) => edo,
            Err(err) => panic!(err),
        };
        edo.register_handler("name", |_| String::from("World!"));
        assert_eq!(
            edo.render(),
            "Hello World!"
        );
    }

    #[test]
    fn render_template_with_missing_handler() {
        let edo = match Edo::new("Hello {name}") {
            Ok(edo) => edo,
            Err(err) => panic!(err),
        };
        assert_eq!(
            edo.render(),
            "Hello "
        );
    }

    #[test]
    fn render_template_with_arguments() {
        let mut edo = match Edo::new("Hello {name(Gio, yes)}") {
            Ok(edo) => edo,
            Err(err) => panic!(err),
        };
        edo.register_handler("name", |args|
            format!("{}{}", args[0], if args[1] == "yes" { "!" } else { "" })
        );
        assert_eq!(
            edo.render(),
            "Hello Gio!"
        );
    }
}
