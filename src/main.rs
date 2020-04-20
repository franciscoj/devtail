#[macro_use]
extern crate clap;

use devtail::tail::Tail;

use std::collections::hash_map::Entry;
use std::collections::HashMap;

use regex::Regex;

use clap::{Arg};

// Got this from here: https://stackoverflow.com/a/6640851/233720
const UUID_REGEX: &str =
    r"\[(\b[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}\b)\].*";

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("file")
                .short("f")
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();

    if let Some(file_name) = matches.value_of("file") {
        let tail = Tail::new(file_name.to_string());
        let mut map: HashMap<String, bool> = HashMap::new();

        tail.for_each(|line| {
            let regex = Regex::new(UUID_REGEX).unwrap();
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

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn regex_matches_uuid() {
        let line = r#"[df7f9091-18d5-4002-91c9-e084516526ab] Started POST "/visits" for 127.0.0.1 at 2020-04-18 17:50:07 +0200"#;
        let regex = Regex::new(crate::UUID_REGEX).unwrap();
        let captures = regex.captures(&line).unwrap();

        assert_eq!(
            captures.get(1).unwrap().as_str(),
            "df7f9091-18d5-4002-91c9-e084516526ab"
        );
    }

    #[test]
    fn regex_matches_uuid_on_line_with_brackets() {
        let line = r#"[be155bd9-587d-468a-994f-441815edc79d]   CACHE MyModel Load (0.0ms)  SELECT  `my_models`.* FROM `my_models` WHERE `my_models`.`id` = 1 LIMIT 1  [["id", 1], ["LIMIT", 1]]"#;
        let regex = Regex::new(crate::UUID_REGEX).unwrap();
        let captures = regex.captures(&line).unwrap();

        assert_eq!(
            captures.get(1).unwrap().as_str(),
            "be155bd9-587d-468a-994f-441815edc79d"
        );
    }
}
