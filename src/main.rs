mod tail;

use std::env;
use crate::tail::Tail;

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let tail = Tail::new(file_name.clone());

    tail.for_each(|line| println!("=> {}", line))
}
