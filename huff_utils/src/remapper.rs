pub trait Remapper {
    fn remap(&self, path: &str) -> Option<String>;
}
