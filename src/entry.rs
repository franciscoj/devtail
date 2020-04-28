use super::{parser::parse, HttpStatus};

/// An `Entry` is a list of lines with the same UUID + he status that signals the end of the
/// request.
pub struct Entry<'a> {
    pub id: &'a str,
    pub lines: Vec<&'a str>,
    pub status: HttpStatus,
}

impl<'a> Entry<'a> {
    /// Builds a new `Entry` from a log line with an UUID.
    ///
    /// # Examples
    ///
    /// ```
    /// # use devtail::{HttpStatus, entry::Entry};
    /// let line = "[00000000-0000-0000-0000-000000000000] Some initial line";
    /// let entry = Entry::new(line);
    ///
    /// assert_eq!(entry.id, "00000000-0000-0000-0000-000000000000");
    /// assert_eq!(entry.status, HttpStatus::Unknown(0));
    /// assert_eq!(entry.lines.len(), 1);
    /// ```
    pub fn new(line: &'a str) -> Self {
        let (id, status) = parse(line).unwrap();

        Self {
            id,
            status,
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
    /// let mut entry = Entry::new("[00000000-0000-0000-0000-000000000000] Some initial line");
    /// let line = "[00000000-0000-0000-0000-000000000000] Completed 201";
    ///
    /// entry.add(line);
    ///
    /// assert_eq!(entry.status, HttpStatus::Success(201));
    /// assert_eq!(entry.lines.len(), 2);
    /// ```
    pub fn add(&mut self, line: &'a str) {
        let (_, status) = parse(line).unwrap();

        self.lines.push(line);
        self.status = status;
    }
}
