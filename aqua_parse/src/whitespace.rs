use super::*;

/// Parses a tab (`\t`) character.
#[strict]
#[pattern("\t")]
#[derive(Clone, Parse, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tab(Span);

/// Parses a space (` `) character.
#[strict]
#[pattern("[ ]")]
#[derive(Clone, Parse, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Space(Span);

/// Parses a whitespace character.
#[strict]
#[pattern(r"\s")]
#[derive(Clone, Parse, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Whitespace(Span);

/// Parses a newline(`\n` or `\r\n`).
#[strict]
#[pattern(r"((\r\n)|(\n))")]
#[derive(Clone, Parse, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Newline(Span);

/// Parses a non-whitespace character.
#[strict]
#[pattern(r"(\S)")]
#[derive(Clone, Parse, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nonwhitespace(Span);
