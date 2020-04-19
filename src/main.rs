use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let file = File::open(file_name.clone()).unwrap();
    let mut reader = BufReader::new(file);

    reader.seek(SeekFrom::End(0)).unwrap();

    loop {
        let mut line = String::new();
        let res = reader.read_line(&mut line);

        match res {
            Ok(len) => {
                if len > 0 {
                    println!("=> {}", line.replace("\n", ""));
                    line.clear();
                }
            }

            Err(err) => {
                println!("=> ERR: {}", err)
            }
        }

    }
}
