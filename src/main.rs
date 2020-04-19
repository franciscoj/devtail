use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};

struct Tail {
    reader: BufReader<File>,
}

impl Tail {
    fn new(file_name: String) -> Tail {
        let file = File::open(file_name.clone()).unwrap();
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::End(0)).unwrap();

        Tail { reader }
    }
}

impl Iterator for Tail {
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

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let tail = Tail::new(file_name.clone());

    tail.for_each(|line| println!("=> {}", line))
}
