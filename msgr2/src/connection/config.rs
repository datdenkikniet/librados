#[derive(Debug, Clone)]
pub struct Config {
    support_rev21: bool,
}

impl Config {
    pub fn new(support_rev21: bool) -> Self {
        Self { support_rev21 }
    }

    pub fn support_rev21(&self) -> bool {
        self.support_rev21
    }
}
