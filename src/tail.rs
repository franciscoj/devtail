use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Result as IOResult, SeekFrom};

pub struct Tail<T> {
    reader: T,
}

impl Tail<BufReader<File>> {
    pub fn new(file_name: String) -> Tail<BufReader<File>> {
        let file = File::open(file_name.clone()).unwrap();
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::End(0)).unwrap();

        Tail { reader }
    }

    fn read_line(&mut self, buf: &mut String) -> IOResult<usize> {
        self.reader.read_line(buf)
    }
}

impl Iterator for Tail<BufReader<File>> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        let mut maybe_line = None;

        while let None = maybe_line {
            if let Ok(len) = self.read_line(&mut line) {
                maybe_line = if len > 0 {
                    Some(line.clone().replace("\n", ""))
                } else {
                    None
                }
            } else {
                // TODO: This can be handled, might mean that the file has been truncated or
                // something like that.
                panic!("Error reading the file!")
            }
        }

        maybe_line
    }
}
