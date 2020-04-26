#[derive(Debug, PartialEq)]
enum State {
    Unknown,
    // Success,
    // Error,
}

struct Log<'a> {
    state: State,
    entries: Vec<&'a str>,
}

impl<'a> Log<'a> {
    fn new() -> Self {
        Log {
            state: State::Unknown,
            entries: Vec::new(),
        }
    }

    fn add(&mut self, line: &'a str) {
        self.entries.push(line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let log = Log::new();

        assert_eq!(log.state, State::Unknown);
        assert!(log.entries.is_empty())
    }

    #[test]
    fn test_add_line() {
        let mut log = Log::new();
        let line = "A Line";

        log.add(line);

        assert_eq!(log.entries.len(), 1);
        assert_eq!(log.entries.pop(), Some("A Line"));
    }
}
