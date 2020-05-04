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
#[derive(Default)]
pub struct Log {
    /// A list of all the UUIDs on this log.
    pub entry_ids: Vec<String>,
    entries: HashMap<String, LogEntry>,
}

impl Log {
    pub fn new() -> Self {
        Log {
            entries: HashMap::new(),
            entry_ids: vec![],
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
        let len = self.entries.len() + 1;
        let map_entry = self.entries.entry(id.clone());

        match map_entry {
            MapEntry::Vacant(entries) => {
                entries.insert(LogEntry::new(line, len));
                self.entry_ids.push(id.clone());

                Some(id)
            }

            MapEntry::Occupied(mut entries) => {
                entries.get_mut().add(line);

                Some(id)
            }
        }
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

        assert_eq!(id, uuid!("0"));
        assert_eq!(log.len(), 2);
        assert_eq!(entry.status, HttpStatus::Success(200));
    }
}
