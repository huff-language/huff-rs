use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::SystemTime};
use uuid::Uuid;

#[allow(clippy::to_string_in_format_args)]

/// An aliased output location to derive from the cli arguments.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct OutputLocation(pub String);

impl Default for OutputLocation {
    fn default() -> Self {
        Self("./artifacts/".to_string())
    }
}

/// Full File Source
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FullFileSource<'a> {
    /// Flattened file source
    pub source: &'a str,
    /// The top level file source
    pub file: Option<FileSource>,
    /// Files and their associated spans in the flattend file source
    pub spans: Vec<(FileSource, Span)>,
}

impl<'a> FullFileSource<'a> {
    /// Get the relative span
    pub fn relative_span(&self, span: Span) -> Option<Span> {
        self.spans
            .iter()
            .filter(|s| s.1.start < span.start && s.1.end > span.end)
            .map(|s| Span {
                start: span.start - s.1.start,
                end: span.end - s.1.start,
                file: Some(s.0.clone()),
            })
            .collect::<Vec<Span>>()
            .pop()
    }
}

/// File Encapsulation
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FileSource {
    /// File ID
    #[serde(skip)]
    pub id: Uuid,
    /// File Path
    pub path: String,
    /// File Source
    pub source: Option<String>,
    /// Last File Access Time
    pub access: Option<SystemTime>,
    /// An Ordered List of File Dependencies
    pub dependencies: Option<Vec<FileSource>>,
}

impl FileSource {
    /// Generates a fully flattened source code for the given `FileSource` and all its dependencies
    ///
    /// ### Examples
    ///
    /// Let's say you have a file, `a.txt` with two dependencies, `b.txt` and `c.txt`,
    /// `fully_flatten()` will generate a source code string with the contents of `b.txt` and
    /// `c.txt` appended to the end of the contents of `a.txt`.
    pub fn fully_flatten(&self) -> (String, Vec<(FileSource, Span)>) {
        // First grab the parent file source
        let mut full_source = if let Some(s) = &self.source { s.clone() } else { "".to_string() };
        let span = Span::new(0..full_source.len(), None);
        let mut relative_positions = vec![(self.clone(), span)];

        // Then recursively grab source code for dependencies
        match &self.dependencies {
            Some(vfs) => {
                for fs in vfs {
                    let mut flattened = fs.fully_flatten();
                    let span =
                        Span::new(full_source.len()..(full_source.len() + flattened.0.len()), None);
                    full_source.push_str(&flattened.0);
                    relative_positions.append(&mut flattened.1);
                    relative_positions.push((fs.clone(), span))
                }
            }
            None => {}
        }

        // Return the full source
        (full_source, relative_positions)
    }

    /// Derives a File Path's directory
    pub fn derive_dir(path: &str) -> Option<String> {
        let path = PathBuf::from(path);
        match path.parent() {
            Some(p) => p.to_str().map(String::from),
            None => None,
        }
    }

    /// Localizes a file path, if path is relative
    pub fn localize_file(parent: &str, child: &str) -> Option<String> {
        let mut prefix = match FileSource::derive_dir(parent) {
            Some(p) => {
                if p.is_empty() {
                    String::from(".")
                } else {
                    p
                }
            }
            None => String::from("."),
        };
        if child.starts_with("../") {
            let mut res_str = child.to_string();
            while res_str.starts_with("../") {
                let path = PathBuf::from(prefix.clone());
                match path.parent() {
                    Some(p) => match p.to_str().map(String::from) {
                        Some(pref) => {
                            if pref.is_empty() || prefix.ends_with("..") {
                                if prefix.is_empty() || prefix == "." {
                                    prefix = "..".to_string();
                                } else {
                                    prefix = format!("../{}", prefix);
                                }
                            } else {
                                prefix = pref
                            }
                        }
                        None => {
                            tracing::warn!("Failed to convert path to string");
                            return None
                        }
                    },
                    None => {
                        tracing::warn!("Failed to find parent for path: {:?}", path);
                        return None
                    }
                }
                res_str = res_str.replacen("../", "", 1);
            }
            Some(format!("{}/{}", prefix, res_str))
        } else if child.starts_with("./") {
            Some(child.replacen("./", &format!("{}/", prefix), 1))
        } else if child.starts_with('/') {
            Some(child.to_string())
        } else {
            Some(format!("{}/{}", prefix, child))
        }
    }
}

use std::ops::{Add, Range};

/// A Span is a section of a source file.
#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Span {
    /// The start of the span.
    pub start: usize,
    /// The end of the span.
    pub end: usize,
    /// The Associated File
    pub file: Option<FileSource>,
}

impl Span {
    /// An EOF spans [0, 0].
    pub const EOF: Span = Span { start: 0, end: 0, file: None };

    /// Public associated function to instatiate a new span.
    pub fn new(Range { start, end }: Range<usize>, file: Option<FileSource>) -> Self {
        Self { start, end, file }
    }

    /// Converts a span to a range.
    pub fn range(&self) -> Option<Range<usize>> {
        (*self != Self::EOF).then(|| self.start..self.end)
    }

    /// Produces a file identifier string for errors
    pub fn identifier(&self) -> String {
        self.file
            .as_ref()
            .map(|f| format!("\n-> {}:{}-{}", f.path, self.start, self.end))
            .unwrap_or_default()
    }

    /// Produces a source segment string
    pub fn source_seg(&self) -> String {
        self.file
            .as_ref()
            .map(|f| {
                f.source
                    .as_ref()
                    .map(|s| {
                        let line_num =
                            &s[0..self.start].as_bytes().iter().filter(|&&c| c == b'\n').count();
                        let line_start = &s[0..self.start].rfind('\n').unwrap_or(0);
                        let line_end = self.end +
                            s[self.end..s.len()].find('\n').unwrap_or(s.len()).to_owned();
                        let padding =
                            (0..line_num.to_string().len()).map(|_| " ").collect::<String>();
                        format!(
                            "\n     {}|\n  > {} | {}\n     {}|",
                            padding,
                            line_num,
                            &s[line_start.to_owned()..line_end].replace('\n', ""),
                            padding
                        )
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.range().unwrap()
    }
}

impl From<Range<usize>> for Span {
    fn from(Range { start, end }: Range<usize>) -> Self {
        Self { start, end, file: None }
    }
}

impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Span) -> Self::Output {
        Span::new(self.start..rhs.end, None)
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
        self.span.clone()
    }
}
