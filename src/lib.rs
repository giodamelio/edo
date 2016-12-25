//! Edo is a VERY simple templating language. It allows you to register handlers that are executed when their matching names are found in the template.
//!
//! For example, with the template `"Hello {name}"`, the `name` handler would be executed and the string it returns would be substituted in place of the original `{name}`. Handler functions can also accept arguments, which are passed in as a `Vec<str>`.
//!
//! # Examples
//!
//! ### Static input
//! ```
//! use edo::Edo;
//!
//! let mut template = Edo::new("Hello {name}").unwrap();
//! template.register_static("name", "World!");
//! let output = template.render("");
//! assert_eq!(output, "Hello World!");
//! ```
//!
//! ### Simple Handler
//! ```
//! use edo::Edo;
//!
//! let mut template = Edo::new("Hello {name}").unwrap();
//! template.register_handler("name", |_, _| Ok("World!".to_string()));
//! let output = template.render("");
//! assert_eq!(output, "Hello World!");
//! ```
//!
//! ### Handler With Arguments 
//! ```
//! use edo::Edo;
//!
//! let mut template = Edo::new("{say_hello(World)}").unwrap();
//! template.register_handler("say_hello", |args, _| Ok(format!("Hello {}", args[0])));
//! let output = template.render("");
//! assert_eq!(output, "Hello World");
//! ```
#![deny(missing_docs)]

#[macro_use]
extern crate nom;

pub mod error;
mod parse;

use std::str;
use std::collections::HashMap;

use error::EdoError;
use parse::Expression;

enum ValueProducer<'a, C> {
    Handler(Box<Fn(Vec<&'a str>, C) -> Result<String, String>>),
    Static(String),
}

/// A single template. Allows registering of handlers and rendering
pub struct Edo<'a, C> {
    #[doc(hidden)]
    value_producers: HashMap<&'a str, ValueProducer<'a, C>>,
    template: Vec<Expression<'a>>,
}

impl<'a, C: Clone> Edo<'a, C> {
    /// Creates a new template instance.
    ///
    /// # Examples
    /// ```no_run
    /// # #![allow(unused_variables)]
    /// # use edo::Edo;
    /// let template: Result<Edo<&str>, _> = Edo::new("Hello {name}");
    /// ```
    pub fn new(template_string: &'a str) -> Result<Edo<'a, C>, EdoError> {
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
    /// let mut template: Edo<&str> = Edo::new("Hello {name}").unwrap();
    /// template.register_handler("name", |_, _| Ok("World!".to_string()));
    /// ```
    pub fn register_handler<F>(&mut self, name: &'a str, handler: F) where
        F: 'static + Fn(Vec<&'a str>, C) -> Result<String, String> {
        self.value_producers.insert(name, ValueProducer::Handler(Box::new(handler)));
    }

    /// Register a static replacement
    ///
    /// # Examples
    /// ```no_run
    /// # #![allow(unused_variables)]
    /// # use edo::Edo;
    /// let mut template: Edo<&str> = Edo::new("Hello {name}").unwrap();
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
    /// template.register_handler("name", |_, _| Ok("World!".to_string()));
    /// let output = template.render("");
    /// assert_eq!(output, "Hello World!");
    /// ```
    // TODO: add a strict mode that errors when there is no handler
    pub fn render(&mut self, context: C) -> String {
        self.render_with_errors(context).0
    }

    /// Render a template into a string and recieve a vector of errors
    ///
    /// # Examples
    /// ```
    /// # use edo::Edo;
    /// let mut template = Edo::new("Hello {name}").unwrap();
    /// template.register_handler("name", |_, _| Err("Something Broke".to_string()));
    /// let (output, errors) = template.render_with_errors("");
    /// assert_eq!(output, "Hello ");
    /// assert_eq!(errors, vec!["Something Broke".to_string()]);
    /// ```
    pub fn render_with_errors(&mut self, context: C) -> (String, Vec<String>) {
        // Keep track of errors
        let mut errors: Vec<String> = vec![];

        // Iterate over the template and
        // 1. Leave literals untouched
        // 2. Call the handlers for each function call and replace within the output
        (self.template.iter()
            .map(|expression| match *expression {
                Expression::Literal(text) => text.to_string(),
                Expression::Function { name, ref arguments } => {
                    match self.value_producers.get(name) {
                        None => "".to_string(),
                        Some(value_producer) => match value_producer {
                            &ValueProducer::Handler(ref handler) => match handler(arguments.clone(), context.clone()) {
                                Ok(string) => string,
                                Err(error_string) => {
                                    errors.push(error_string);
                                    "".to_string()
                                },
                            },
                            &ValueProducer::Static(ref value) => value.clone(),
                        },
                    }
                }
            })
            .collect::<Vec<String>>()
            .concat(), errors)
    }
}

#[cfg(test)]
mod tests {
    use super::Edo;

    #[test]
    fn create_new_edo() {
        let edo: Result<Edo<&str>, _> = Edo::new("Hello {name}");
        assert!(edo.is_ok());
    }

    #[test]
    fn register_handler() {
        let mut edo: Edo<&str> = match Edo::new("Hello {name}") {
            Ok(edo) => edo,
            Err(err) => panic!(err),
        };
        edo.register_handler("name", |_, context| Ok("World!".to_string()));
        assert!(edo.value_producers.get("name").is_some());
    }

    #[test]
    fn register_static() {
        let mut edo: Edo<&str> = match Edo::new("Hello {name}") {
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
        edo.register_handler("name", |_, context| Ok("World!".to_string()));
        assert_eq!(
            edo.render(""),
            "Hello World!"
        );
    }

    #[test]
    fn render_template_with_missing_handler() {
        let mut edo = match Edo::new("Hello {name}") {
            Ok(edo) => edo,
            Err(err) => panic!(err),
        };
        assert_eq!(
            edo.render(""),
            "Hello "
        );
    }

    #[test]
    fn render_template_with_arguments() {
        let mut edo = match Edo::new("Hello {name(Gio, yes)}") {
            Ok(edo) => edo,
            Err(err) => panic!(err),
        };
        edo.register_handler("name", |args, _|
            Ok(format!("{}{}", args[0], if args[1] == "yes" { "!" } else { "" }))
        );
        assert_eq!(
            edo.render(""),
            "Hello Gio!"
        );
    }

    #[test]
    fn render_template_with_context() {
        let mut edo = match Edo::new("Hello {name}") {
            Ok(edo) => edo,
            Err(err) => panic!(err),
        };
        edo.register_handler("name", |_, context: &str| Ok(context.to_string()));
        assert_eq!(
            edo.render("Context!!!"),
            "Hello Context!!!"
        );
    }

    #[test]
    fn render_with_errors() {
        let mut edo = match Edo::new("Hello {name}") {
            Ok(edo) => edo,
            Err(err) => panic!(err),
        };
        edo.register_handler("name", |_, _| Err("BORK".to_string()));
        let (output, errors) = edo.render_with_errors("");
        assert_eq!(output, "Hello ");
        assert_eq!(errors, vec!["BORK"]);
    }
}
