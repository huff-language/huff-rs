use std::ops::{Add, Range};

/// A Span is a section of a source file.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Span {
    /// The start of the span.
    pub start: usize,
    /// The end of the span.
    pub end: usize,
}

impl Span {
    /// An EOF spans [0, 0].
    pub const EOF: Span = Span { start: 0, end: 0 };

    /// Public associated function to instatiate a new span.
    pub fn new(Range { start, end }: Range<usize>) -> Self {
        Self { start, end }
    }

    /// Converts a span to a range.
    pub fn range(&self) -> Option<Range<usize>> {
        (*self != Self::EOF).then(|| self.start..self.end)
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.range().unwrap()
    }
}

impl From<Range<usize>> for Span {
    fn from(Range { start, end }: Range<usize>) -> Self {
        Self { start, end }
    }
}

impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Span) -> Self::Output {
        Span::new(self.start..rhs.end)
    }
}

/// Spanned trait requires a type to have a span.
pub trait Spanned {
    /// Returns a Span.
    fn span(&self) -> Span;
}

/// WithSpan associates a value to a Span.
pub struct WithSpan<T> {
    /// The value
    pub value: T,
    /// The associated Span
    pub span: Span,
}

impl<T> WithSpan<T> {
    /// Public associated function to instatiate a new WithSpan.
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}

impl<T> Spanned for WithSpan<T> {
    fn span(&self) -> Span {
        self.span
    }
}