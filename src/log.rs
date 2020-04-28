/// Represents a collection of all the different entries of the log.
///
/// It is empty by default. When a new line is added it gets added into the right entry that needs
/// to hold it so that all related lines are together.
///
/// # Examples
///
/// ```
/// # use devtail::log::Log;
/// let log = Log::new();
///
/// assert!(log.is_empty());
/// ```
pub struct Log<'a> {
    entries: Vec<&'a str>,
}

impl<'a> Log<'a> {
    pub fn new() -> Self {
        Log {
            entries: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn add(&mut self, line: &'a str) {
        self.entries.push(line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_line() {
        let mut log = Log::new();
        let line = "A Line";

        log.add(line);

        assert_eq!(log.entries.len(), 1);
        assert_eq!(log.entries.pop(), Some("A Line"));
    }
}
