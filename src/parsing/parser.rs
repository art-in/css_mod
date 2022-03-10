use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::*;
use std::fmt;

#[derive(Parser)]
#[grammar = "parsing/parser.pest"]
struct Grammar;

pub type ParserResult<'p, T> = Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    Parser(pest::error::Error<Rule>),
    AtPair(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Parser(error) => write!(formatter, "{}", error),
            Error::AtPair(error) => write!(formatter, "{}", error),
        }
    }
}

impl<'p> From<Pair<'p, Rule>> for Error {
    fn from(pair: Pair<'p, Rule>) -> Self {
        let message = match pair.as_rule() {
            Rule::error_block_not_terminated => "Unterminated ruleset",
            _ => "Unexpected error",
        };
        let start = pair.as_span().start_pos();
        let line = start.line_of();
        let (line_no, col_no) = start.line_col();
        let line_no_len = format!("{}", line_no).len();
        let mut spacing = String::new();

        for _ in 0..line_no_len {
            spacing.push(' ');
        }

        // TODO: add file path to error message
        Error::AtPair(format!(
            " {line_no:indent$} ┊ {line}\n    {spacing:col_no$}│\n    {spacing:col_no$}╰ {message} at {line_no}:{col_no}",
            spacing = spacing,
            indent = spacing.len(),
            col_no = col_no,
            line_no = line_no,
            line = line,
            message = message,
        ))
    }
}

fn parse<'p>(rule: Rule, input: &'p str) -> ParserResult<Pairs<'p, Rule>> {
    match Grammar::parse(rule, input) {
        Ok(pairs) => Ok(pairs),
        Err(error) => Err(Error::Parser(error)),
    }
}

pub fn animation<'p>(animation: &'p str) -> ParserResult<Pairs<'p, Rule>> {
    parse(Rule::animation, animation)
}

pub fn stylesheet<'p>(stylesheet: &'p str) -> ParserResult<Pairs<'p, Rule>> {
    parse(Rule::stylesheet, stylesheet)
}

pub fn selector<'p>(selector: &'p str) -> ParserResult<Pairs<'p, Rule>> {
    parse(Rule::selector, selector)
}
