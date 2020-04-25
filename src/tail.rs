use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, SeekFrom};
use std::io::{Stdin, StdinLock};

// Got this from here: https://stackoverflow.com/a/6640851/233720
const UUID_REGEX: &str =
    r"\[(\b[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}\b)\].*";

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

impl<T: BufRead> Tail<T> {
    pub fn run(self) {
        let regex = Regex::new(UUID_REGEX).unwrap();
        let mut map: HashMap<String, bool> = HashMap::new();

        self.for_each(|line| {
            let maybe_capures = regex.captures(&line);

            if let Some(captures) = maybe_capures {
                let entry = map.entry(captures.get(1).unwrap().as_str().to_string());

                if let Entry::Vacant(o) = entry {
                    o.insert(true);
                    println!("=> {}", line);
                };
            };
        });
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

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn regex_matches_uuid() {
        let line = r#"[df7f9091-18d5-4002-91c9-e084516526ab] Started POST "/visits" for 127.0.0.1 at 2020-04-18 17:50:07 +0200"#;
        let regex = Regex::new(super::UUID_REGEX).unwrap();
        let captures = regex.captures(&line).unwrap();

        assert_eq!(
            captures.get(1).unwrap().as_str(),
            "df7f9091-18d5-4002-91c9-e084516526ab"
        );
    }

    #[test]
    fn regex_matches_uuid_on_line_with_brackets() {
        let line = r#"[be155bd9-587d-468a-994f-441815edc79d]   CACHE MyModel Load (0.0ms)  SELECT  `my_models`.* FROM `my_models` WHERE `my_models`.`id` = 1 LIMIT 1  [["id", 1], ["LIMIT", 1]]"#;
        let regex = Regex::new(super::UUID_REGEX).unwrap();
        let captures = regex.captures(&line).unwrap();

        assert_eq!(
            captures.get(1).unwrap().as_str(),
            "be155bd9-587d-468a-994f-441815edc79d"
        );
    }
}
