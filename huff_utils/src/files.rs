use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    cell::Ref,
    collections::HashMap,
    fs,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    sync::Arc,
};
use uuid::Uuid;

#[allow(clippy::to_string_in_format_args)]

/// An aliased output location to derive from the cli arguments.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct OutputLocation(pub String);
/// Full File Source
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FullFileSource<'a> {
    /// Flattened file source
    pub source: &'a str,
    /// The top level file source
    pub file: Option<Arc<FileSource>>,
    /// Files and their associated spans in the flattend file source
    pub spans: Vec<(Arc<FileSource>, Span)>,
}

impl<'a> FullFileSource<'a> {
    /// Get the relative span
    pub fn relative_span(&self, span: Ref<'a, Span>) -> Option<Span> {
        self.spans
            .iter()
            .filter(|s| s.1.start < span.start && s.1.end > span.end)
            .map(|s| Span {
                start: span.start - s.1.start,
                end: span.end - s.1.start,
                file: Some(s.0.clone()),
            })
            .collect::<Vec<Span>>()
            .into_iter()
            .next()
    }
}

/// A wrapper for dealing with Remappings
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Remapper {
    /// The remappings
    pub remappings: HashMap<String, String>,
    /// The base directory
    pub base_dir: String,
}

impl Remapper {
    /// Extracts remappings from configuration files.
    ///
    /// Currently only supports `foundry.toml` remapping definitions.
    pub fn new(root: impl AsRef<str>) -> Self {
        let mut inner = HashMap::<String, String>::new();

        // Gracefully parse remappings from foundry.toml
        Remapper::from_foundry(root.as_ref(), &mut inner);

        // And from remappings.txt
        Remapper::from_file(root.as_ref(), &mut inner);

        // Return the constructed remappings
        Self { remappings: inner, base_dir: root.as_ref().to_string() }
    }

    /// Helper to break apart a remapping gracefully
    pub fn split(remapping: &str) -> Option<(String, String)> {
        let mut split = remapping.splitn(2, '=');
        match split.next() {
            Some(from) => split.next().map(|to| (from.to_string(), to.to_string())),
            None => None,
        }
    }

    /// Parse foundry toml remappings
    pub fn from_foundry(root: &str, inner: &mut HashMap<String, String>) {
        // Look for a `foundry.toml` file in the current directory.
        let path = Path::new(root).join("foundry.toml");

        match fs::File::open(&path) {
            Ok(f) => {
                // Open the buffered reader and read foundry.toml
                let mut data = String::new();
                let mut br = BufReader::new(f);

                // Gracefully read foundry.toml
                if let Err(e) = br.read_to_string(&mut data) {
                    tracing::warn!(target: "parser", "Failed to read \"foundry.toml\" file contents!\nError: {:?}", e);
                    return;
                }

                // Parse the foundry.toml file as toml
                let toml = if let Ok(t) = data.parse::<toml::Value>() {
                    t
                } else {
                    tracing::warn!(target: "parser", "\"foundry.toml\" incorrectly formatted!");
                    return;
                };

                // Parse the toml as a map
                let toml_map = toml.as_table().cloned().unwrap_or_else(toml::value::Map::new);

                // Transform the mappings into profiles
                let profiles = toml_map
                    .iter()
                    .filter_map(|p| p.1.as_table())
                    .collect::<Vec<&toml::value::Map<String, toml::Value>>>();
                let unwrapped_profiles = profiles
                    .iter()
                    .flat_map(|t| t.values().collect_vec())
                    .collect::<Vec<&toml::Value>>();

                // Extract the inner tables from each profile
                let inner_tables = unwrapped_profiles
                    .iter()
                    .filter_map(|t| t.as_table())
                    .collect::<Vec<&toml::value::Map<String, toml::Value>>>();
                let unwrapped_inner_tables = inner_tables
                    .iter()
                    .flat_map(|t| {
                        t.into_iter().filter(|m| m.0.eq("remappings")).map(|m| m.1).collect_vec()
                    })
                    .collect::<Vec<&toml::Value>>();

                // Extract mappings that are arrays
                let arr_mappings = unwrapped_inner_tables
                    .iter()
                    .filter_map(|t| t.as_array())
                    .collect::<Vec<&Vec<toml::Value>>>();
                let unwrapped_mappings =
                    arr_mappings.iter().cloned().flatten().collect::<Vec<&toml::Value>>();

                // Filter the remappings as strings
                let remapping_strings =
                    unwrapped_mappings.iter().filter_map(|t| t.as_str()).collect::<Vec<&str>>();

                // For each remapping string, try to split it and insert it into the remappings
                remapping_strings.iter().for_each(|remapping| {
                    match Remapper::split(remapping) {
                        Some((from, to)) => {
                            inner.insert(from, to);
                        }
                        None => tracing::warn!(target: "parser", "Failed to split remapping using \"=\" at \"{}\" in \"{}\"!", remapping, path.to_string_lossy()),
                    }
                });
            }
            Err(e) => {
                tracing::warn!(target: "parser", "Foundry.toml not found in specified \"{}\"", root);
                tracing::warn!(target: "parser", "{:?}", e);
            }
        }
    }

    /// Get remappings from a remappings.txt file
    pub fn from_file(root: &str, inner: &mut HashMap<String, String>) {
        let mut remappings: HashMap<String, String> = HashMap::new();
        let remappings_file = PathBuf::new().join(root).join("remappings.txt");
        if remappings_file.is_file() {
            let content =
                fs::read_to_string(remappings_file).map_err(|err| err.to_string()).unwrap();

            let rem_lines = content.split('\n').collect::<Vec<&str>>();
            let rem = rem_lines
                .iter()
                .filter(|l| l != &&"")
                .map(|l| l.split_once('='))
                .collect::<Vec<Option<(&str, &str)>>>();
            rem.iter().for_each(|pair| {
                if let Some((lib, path)) = pair {
                    remappings.insert(lib.to_string(), path.to_string());
                }
            });

            inner.extend(remappings);
        }
    }
}

impl Remapper {
    /// Tries to replace path segments in a string with our remappings
    pub fn remap(&self, path: &str) -> Option<String> {
        let mut path = path.to_string();
        for (k, v) in self.remappings.iter() {
            if path.starts_with(k) {
                tracing::debug!(target: "parser", "found key {} and value {}", k, v);
                path = path.replace(k, v);
                return Some(format!("{}{path}", self.base_dir));
            }
        }
        None
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
    pub access: Option<time::Time>,
    /// An Ordered List of File Dependencies
    pub dependencies: Option<Vec<Arc<FileSource>>>,
}

impl FileSource {
    /// Generates a fully flattened source code for the given `FileSource` and all its dependencies
    ///
    /// ### Examples
    ///
    /// Let's say you have a file, `a.txt` with two dependencies, `b.txt` and `c.txt`,
    /// `fully_flatten()` will generate a source code string with the contents of `b.txt` and
    /// `c.txt` appended to the end of the contents of `a.txt`.
    pub fn fully_flatten(self_ref: Arc<FileSource>) -> (String, Vec<(Arc<FileSource>, Span)>) {
        // First grab the parent file source
        let mut full_source =
            if let Some(s) = &self_ref.source { s.clone() } else { String::default() };
        let span = Span::new(0..full_source.len(), None);
        let mut relative_positions = vec![(Arc::clone(&self_ref), span)];

        // Then recursively grab source code for dependencies
        match &self_ref.dependencies {
            Some(vfs) => {
                for fs in vfs {
                    let mut flattened = FileSource::fully_flatten(Arc::clone(fs));
                    let span =
                        Span::new(full_source.len()..(full_source.len() + flattened.0.len()), None);
                    full_source.push_str(&flattened.0);
                    relative_positions.append(&mut flattened.1);
                    relative_positions.push((Arc::clone(fs), span))
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
        let mut prefixed_parent;
        if !parent.starts_with('.') {
            prefixed_parent = "./".to_owned();
            prefixed_parent.push_str(parent);
        } else {
            prefixed_parent = parent.to_owned();
        }
        let mut prefix = match FileSource::derive_dir(prefixed_parent.as_str()) {
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
                                    prefix = format!("../{prefix}");
                                }
                            } else {
                                prefix = pref
                            }
                        }
                        None => {
                            tracing::warn!("Failed to convert path to string");
                            return None;
                        }
                    },
                    None => {
                        tracing::warn!("Failed to find parent for path: {:?}", path);
                        return None;
                    }
                }
                res_str = res_str.replacen("../", "", 1);
            }
            Some(format!("{prefix}/{res_str}"))
        } else if child.starts_with("./") {
            Some(child.replacen("./", &format!("{prefix}/"), 1))
        } else if child.starts_with('/') {
            Some(child.to_string())
        } else {
            Some(format!("{prefix}/{child}"))
        }
    }
}

use crate::time;
use std::ops::{Add, Range};

/// A Span is a section of a source file.
#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Span {
    /// The start of the span.
    pub start: usize,
    /// The end of the span.
    pub end: usize,
    /// The Associated File
    pub file: Option<Arc<FileSource>>,
}

impl Span {
    /// An EOF spans [0, 0].
    pub const EOF: Span = Span { start: 0, end: 0, file: None };

    /// Public associated function to instatiate a new span.
    pub fn new(Range { start, end }: Range<usize>, file: Option<Arc<FileSource>>) -> Self {
        Self { start, end, file }
    }

    /// Converts a span to a range.
    pub fn range(&self) -> Option<Range<usize>> {
        (*self != Self::EOF).then_some(self.start..self.end)
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
                            &s[0..self.start].as_bytes().iter().filter(|&&c| c == b'\n').count()
                                + 1;
                        let line_start = &s[0..self.start].rfind('\n').unwrap_or(0);
                        let line_end = self.end
                            + s[self.end..s.len()]
                                .find('\n')
                                .unwrap_or(s.len() - self.end)
                                .to_owned();
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
