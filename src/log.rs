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
pub struct Log {
    entries: HashMap<String, LogEntry>,
}

impl Log {
    pub fn new() -> Self {
        Log {
            entries: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn add(&mut self, line: String) -> Option<String> {
        let (id, _) = parse(&line)?;
        let map_entry = self.entries.entry(id.clone());

        if let MapEntry::Vacant(entries) = map_entry {
            let entry = LogEntry::new(line);

            entries.insert(entry);

            Some(id)
        } else if let MapEntry::Occupied(mut entries) = map_entry {
            let entry = entries.get_mut();

            entry.add(line);

            None
        } else {
            None
        }
    }

    pub fn print(&self, id: String) {
        let entry = self.entries.get(&id).unwrap();

        entry.print();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HttpStatus;

    #[test]
    fn test_add_lines() {
        let mut log = Log::new();
        log.add(String::from(
            "[00000000-0000-0000-0000-000000000000] A line",
        ));
        log.add(String::from(
            "[00000000-0000-0000-0000-000000000000] Completed 200",
        ));
        log.add(String::from(
            "[11111111-1111-1111-1111-111111111111] Other line",
        ));
        log.add(String::from(
            "[11111111-1111-1111-1111-111111111111] Completed 302",
        ));

        let entry = log
            .entries
            .get("00000000-0000-0000-0000-000000000000")
            .unwrap();

        assert_eq!(log.entries.len(), 2);
        assert_eq!(entry.status, HttpStatus::Success(200));
    }
}
