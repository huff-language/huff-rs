use std::{
    fmt,
    io::{self, Write},
};

use crate::files::{Span, Spanned};

/// A Report Trait
pub trait Report<W>: Spanned {
    /// Report to the reporter
    fn report(&self, reporter: &mut Reporter<'_, W>) -> io::Result<()>;
}

/// A Reporter
pub struct Reporter<'a, W> {
    /// The output writer
    pub out: W,
    /// The raw source code
    pub source: &'a str,
}

impl<'a, W> Reporter<'a, W>
where
    W: Write,
{
    /// Public associated function to instatiate a new Reporter.
    pub fn new(out: W, source: &'a str) -> Self {
        Self { out, source }
    }

    fn report(&mut self, err: impl Report<W>) -> Result<(), io::Error> {
        write!(self.out, "[error]: ")?;
        err.report(self)?;
        writeln!(self.out, "\n{}", self.source)?;

        let pad = if err.span() == Span::EOF { self.source.len() } else { err.span().start };

        writeln!(self.out, "{:pad$}^ ", "")
    }

    /// Reports and exits the process.
    pub fn exit(&mut self, err: impl Report<W>) -> ! {
        self.report(err).expect("failed to write to stdout");
        std::process::exit(1)
    }
}

impl<E, W> From<E> for Box<dyn Report<W>>
where
    E: Report<W> + fmt::Debug + 'static,
{
    fn from(err: E) -> Self {
        Box::new(err)
    }
}

impl<W> Spanned for Box<dyn Report<W>> {
    fn span(&self) -> Span {
        (**self).span()
    }
}

impl<W> Report<W> for Box<dyn Report<W>> {
    fn report(&self, reporter: &mut Reporter<'_, W>) -> io::Result<()> {
        (**self).report(reporter)
    }
}
