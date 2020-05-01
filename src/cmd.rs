use super::tail::Tail;
use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::BufRead;

// Got this from here: https://stackoverflow.com/a/6640851/233720
const UUID_REGEX: &str =
    r"\[(\b[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}\b)\].*";

pub fn run<T: BufRead>(tail: Tail<T>) {
    let regex = Regex::new(UUID_REGEX).unwrap();
    let mut map: HashMap<String, bool> = HashMap::new();

    for line in tail {
        let maybe_capures = regex.captures(line.as_str());

        if let Some(captures) = maybe_capures {
            let entry = map.entry(captures.get(1).unwrap().as_str().to_string());

            if let Entry::Vacant(o) = entry {
                o.insert(true);
                println!("=> {}", line);
            };
        };
    }
}
