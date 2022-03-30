use anyhow::{anyhow, Result};
use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::*;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "parsing/parser.pest"]
struct Grammar;

#[derive(Debug, PartialEq, Error)]
pub enum Error {
    #[error("{0}")]
    Parser(pest::error::Error<Rule>),
    #[error("{0}")]
    AtPair(String),
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

fn parse(rule: Rule, input: &str) -> Result<Pairs<Rule>> {
    match Grammar::parse(rule, input) {
        Ok(pairs) => Ok(pairs),
        Err(error) => Err(anyhow!(Error::Parser(error))),
    }
}

pub fn animation(animation: &str) -> Result<Pairs<Rule>> {
    parse(Rule::animation, animation)
}

pub fn stylesheet(stylesheet: &str) -> Result<Pairs<Rule>> {
    parse(Rule::stylesheet, stylesheet)
}

pub fn selector(selector: &str) -> Result<Pairs<Rule>> {
    parse(Rule::selector, selector)
}
