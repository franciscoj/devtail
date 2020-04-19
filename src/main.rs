use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};

struct Tail {
    reader: BufReader<File>
}

impl Tail {
    fn new(file_name: String) -> Tail {
        let file = File::open(file_name.clone()).unwrap();
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::End(0)).unwrap();

        Tail { reader }
    }
}

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let tail = Tail::new(file_name.clone());
    let mut reader = tail.reader;

    loop {
        let mut line = String::new();
        let res = reader.read_line(&mut line);

        match res {
            Ok(len) => {
                if len > 0 {
                    println!("=> {}", line.replace("\n", ""));
                }
            }

            Err(err) => {
                println!("=> ERR: {}", err)
            }
        }

    }
}
