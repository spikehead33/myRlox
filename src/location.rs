pub struct Location(pub String, pub usize, pub usize);

impl Location {
    fn filename(&self) -> &str {
        &self.0.as_str()
    }

    fn lineno(&self) -> usize {
        self.1
    }

    fn column(&self) -> usize {
        self.2
    }
}
