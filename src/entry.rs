extern crate termion;

use super::{parser::parse, HttpStatus};

/// An `Entry` is a list of lines with the same UUID + he status that signals the end of the
/// request.
pub struct Entry {
    pub id: String,
    pub lines: Vec<String>,
    pub status: HttpStatus,
    pub order: usize,
}

impl Entry {
    /// Builds a new `Entry` from a log line with an UUID.
    ///
    /// # Examples
    ///
    /// ```
    /// # use devtail::{HttpStatus, entry::Entry};
    /// let line = String::from("[00000000-0000-0000-0000-000000000000] GET /users");
    /// let entry = Entry::new(line, 0);
    ///
    /// assert_eq!(entry.id, String::from("00000000-0000-0000-0000-000000000000"));
    /// assert_eq!(entry.status, HttpStatus::Unknown(0));
    /// assert_eq!(entry.lines.len(), 1);
    /// assert_eq!(entry.order, 0)
    /// ```
    pub fn new(line: String, order: usize) -> Self {
        let (id, status) = parse(&line).unwrap();

        Self {
            id,
            status,
            order,
            lines: vec![line],
        }
    }

    // TODO: Find a way to somehow have a bit of parsing in case there's a 500 with a backtrace of
    // an error on it.
    /// Adds a new line to an existing `Entry`. In ase there's a `Completed XXX` text on it, it
    /// modifies the state of the line accordingly.
    ///
    /// # Examples
    ///
    /// ```
    /// # use devtail::{HttpStatus, entry::Entry};
    /// let mut entry = Entry::new(
    ///    String::from("[00000000-0000-0000-0000-000000000000] GET /users"),
    ///    0
    /// );
    /// let line = String::from("[00000000-0000-0000-0000-000000000000] Completed 201");
    ///
    /// entry.add(line);
    ///
    /// assert_eq!(entry.status, HttpStatus::Success(201));
    /// assert_eq!(entry.lines.len(), 2);
    /// ```
    pub fn add(&mut self, line: String) {
        let (_, status) = parse(&line).unwrap();

        self.lines.push(line);
        self.status = status;
    }

    // TODO: I'm still not sure that this belongs here. Right now screen takes care of the styling
    // while this prints. Seems like some kind of "presenter" or "view" could be needed but I'm
    // still not sure that's something that can be needed.
    /// Prints the first line on this `Entry`.
    pub fn print(&self, cols: &usize) {
        let mut line = self.lines[0].clone();
        line.truncate(*cols - 15);

        // print!("[{}]", self.order);
        println!("{}", line);
    }
}
