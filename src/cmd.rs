extern crate termion;

use super::log::Log;
use super::tail::Tail;
use std::io::BufRead;
use termion::{clear, cursor};

/// Runs a `Tail`.
///
/// Running a `Tail` means iterating on each line coming from the source (either a file or stdin)
/// and printing them.
pub fn run<T: BufRead>(tail: Tail<T>) {
    println!("{}{}", clear::All, cursor::Goto(1, 1));

    let mut log = Log::new();

    for line in tail {
        if let Some(id) = log.add(line.clone()) {
            log.print(id);
        }
    }
}
