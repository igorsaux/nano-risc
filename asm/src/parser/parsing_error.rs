use super::{ParsingErrorKind, Span};
use nano_risc_arch::Location;
use nom::error::{ErrorKind, ParseError};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsingError {
    message: String,
    location: Location,
    kind: ParsingErrorKind,
    inner: Option<Box<ParsingError>>,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Parsing error: {}", self.message))
    }
}

impl Error for ParsingError {}

impl<'s> ParseError<Span<'s>> for ParsingError {
    fn from_error_kind(input: Span, kind: ErrorKind) -> Self {
        Self {
            message: format!("Parsing error: {kind:#?}"),
            location: input
                .extra
                .find_location(input.location_offset())
                .unwrap_or_default(),
            kind: ParsingErrorKind::Unknown,
            inner: None,
        }
    }

    fn append(input: Span, kind: ErrorKind, other: Self) -> Self {
        Self {
            message: format!("Parsing error: {kind:#?}"),
            location: input
                .extra
                .find_location(input.location_offset())
                .unwrap_or_default(),
            kind: ParsingErrorKind::Unknown,
            inner: Some(Box::new(other)),
        }
    }
}

impl ParsingError {
    pub fn new(message: String, location: Location, kind: ParsingErrorKind) -> Self {
        Self {
            message,
            location,
            kind,
            inner: None,
        }
    }

    pub fn wrap(message: String, location: Location, kind: ParsingErrorKind, inner: Self) -> Self {
        Self {
            message,
            location,
            kind,
            inner: Some(Box::new(inner)),
        }
    }

    pub fn set_inner(&mut self, inner: Option<Box<Self>>) {
        self.inner = inner;
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn kind(&self) -> ParsingErrorKind {
        self.kind
    }

    pub fn inner(&self) -> Option<&ParsingError> {
        self.inner.as_ref().map(|v| v.as_ref())
    }

    pub(crate) fn from_nom_error(
        message: String,
        error: nom::Err<Self>,
        kind: ParsingErrorKind,
    ) -> nom::Err<Self> {
        match error {
            nom::Err::Incomplete(_) => error,
            nom::Err::Error(inner) => nom::Err::Error(Self::new(message, inner.location, kind)),
            nom::Err::Failure(inner) => nom::Err::Failure(Self::new(message, inner.location, kind)),
        }
    }
}
