use super::{entry::Entry as LogEntry, parser::parse};
use std::collections::{hash_map::Entry as MapEntry, HashMap};

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
    entries: HashMap<&'a str, LogEntry<'a>>,
}

impl<'a> Log<'a> {
    pub fn new() -> Self {
        Log {
            entries: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn add(&mut self, line: &'a str) {
        let (id, _) = parse(line).unwrap();
        let map_entry = self.entries.entry(id);

        if let MapEntry::Vacant(entries) = map_entry {
            let entry = LogEntry::new(line);
            entries.insert(entry);
        } else if let MapEntry::Occupied(mut entries) = map_entry {
            let entry = entries.get_mut();

            entry.add(line);
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HttpStatus;

    #[test]
    fn test_add_lines() {
        let mut log = Log::new();
        log.add("[00000000-0000-0000-0000-000000000000] A line");
        log.add("[00000000-0000-0000-0000-000000000000] Completed 200");
        log.add("[11111111-1111-1111-1111-111111111111] Other line");
        log.add("[11111111-1111-1111-1111-111111111111] Completed 302");

        let entry = log
            .entries
            .get("00000000-0000-0000-0000-000000000000")
            .unwrap();

        assert_eq!(log.entries.len(), 2);
        assert_eq!(entry.status, HttpStatus::Success(200));
    }
}
