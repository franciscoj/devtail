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

impl<'a> Log<'_> {
    fn new() -> Self {
        Log {
            state: State::Unknown,
            entries: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_log_entry() {
        let log = Log::new();

        assert_eq!(log.state, State::Unknown);
        assert!(log.entries.is_empty())
    }
}
