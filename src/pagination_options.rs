#[derive(Debug, Default)]
pub struct PaginationOptions {
    pub count: Option<u64>,
    pub sort: Option<String>,
    pub cursor: Option<String>,
}

impl PaginationOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(mut self, count: u64) -> Self {
        self.count = Some(count);
        self
    }

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}
