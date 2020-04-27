#[derive(Debug, PartialEq)]
pub enum State {
    Unknown,
    // Success,
    // Error,
}

/// Represents a multi-line entry on a log.
///
/// It is by default empty and has `Unknown` state. Its state represents the state of the log. E.g.
/// a 200 OK on an HTTP request might turn the state into `Success` while a 500 status might turn
/// it into `Error`
///
/// # Examples
///
/// ```
/// # use devtail::log::{Log, State};
/// let log = Log::new();
///
/// assert!(log.is_empty());
/// assert_eq!(log.state, State::Unknown);
/// ```
pub struct Log<'a> {
    pub state: State,
    entries: Vec<&'a str>,
}

impl<'a> Log<'a> {
    pub fn new() -> Self {
        Log {
            state: State::Unknown,
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
