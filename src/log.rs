use super::{entry::Entry as LogEntry, parser::parse};
use std::collections::{hash_map::Entry as MapEntry, HashMap};

/// Represents a collection of all the different entries of the log.
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

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn get(&self, id: String) -> Option<&LogEntry> {
        self.entries.get(&id)
    }

    /// Adds a new line to a log.
    ///
    /// When the log already has an entry for that UUID it gets appended. If not, it creates a new
    /// entry.
    ///
    /// In case it has added an entry it returns `Option<String>`
    pub fn add(&mut self, line: String) -> Option<String> {
        let (id, _) = parse(&line)?;
        let len = self.entries.len();
        let map_entry = self.entries.entry(id.clone());

        match map_entry {
            MapEntry::Vacant(entries) => {
                entries.insert(LogEntry::new(line, len));

                Some(id)
            }

            MapEntry::Occupied(mut entries) => {
                entries.get_mut().add(line);

                None
            }
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
        let id = log.add(log_start!("0")).unwrap();
        log.add(log_end!("0"));
        log.add(log_start!("1"));
        log.add(log_end!("1"));

        let entry = log.entries.get(&uuid!("0")).unwrap();

        assert_eq!(id, String::from(uuid!("0")));
        assert_eq!(log.len(), 2);
        assert_eq!(entry.status, HttpStatus::Success(200));
    }
}
