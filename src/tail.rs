use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, SeekFrom};
use std::io::{Stdin, StdinLock};

pub struct Tail<T: BufRead> {
    reader: T,
}

pub type TailF = Tail<BufReader<File>>;
pub type TailS<'a> = Tail<StdinLock<'a>>;

impl TailF {
    pub fn new(file_name: String) -> Self {
        let file = File::open(file_name.clone()).unwrap();
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::End(0)).unwrap();

        Tail { reader }
    }
}

impl<'a> TailS<'a> {
    pub fn new(stdin: &'a Stdin) -> Self {
        Tail {
            reader: stdin.lock(),
        }
    }
}

impl<T: BufRead> Iterator for Tail<T> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        let mut maybe_line = None;

        while let None = maybe_line {
            if let Ok(len) = self.reader.read_line(&mut line) {
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
