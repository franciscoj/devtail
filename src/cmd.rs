extern crate termion;

use super::log::Log;
use super::screen::Screen;
use super::tail::Tail;
use std::io::BufRead;

/// Runs a `Tail`.
///
/// Running a `Tail` means iterating on each line coming from the source (either a file or stdin)
/// and printing them.
pub fn run<T: BufRead>(tail: Tail<T>) {
    let mut log = Log::new();
    let screen = Screen::new();

    screen.clear();

    for line in tail {
        if let Some(_) = log.add(line.clone()) {
            screen.print_log(&log);
        }
    }
}
