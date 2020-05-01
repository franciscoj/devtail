use super::log::Log;
use super::tail::Tail;
use std::io::BufRead;

pub fn run<T: BufRead>(tail: Tail<T>) {
    let mut log = Log::new();

    for line in tail {
        log.add(line);
    }
}
