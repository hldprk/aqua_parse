use super::*;

/// Parses a tab (`\t`) character.
#[strict]
#[literal("\t")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Parse, Hash)]
pub struct Tab;

#[strict]
#[literal(" ")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Parse, Hash)]
pub struct Space;

/// Parses a whitespace character.
#[strict]
#[pattern(r"\s")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parse, Hash)]
pub struct Whitespace(pub String);

/// Parses a newline(\n or \r\n).
#[strict]
#[pattern(r"(\r\n)|(\n)")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parse, Hash)]
pub struct Newline(pub String);

/// Parses a non-whitespace character.
#[strict]
#[pattern(r"\S")]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Parse, Hash)]
pub struct Nonwhitespace(pub String);
