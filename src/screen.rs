use super::entry::Entry;
use super::log::Log;
use super::HttpStatus;
use std::convert::TryFrom;
use termion::{clear, color, cursor, terminal_size};

type Size = (u16, u16);

#[derive(Default)]
pub struct Screen {
    size: Size,
}

impl Screen {
    pub fn new() -> Self {
        let (cols, rows) = terminal_size().unwrap();
        Self {
            size: (cols, rows - 2),
        }
    }

    pub fn new_with_size(size: Size) -> Self {
        let (cols, rows) = size;
        Self {
            size: (cols, rows - 2),
        }
    }

    pub fn clear(&self) {
        println!("{}{}", clear::All, cursor::Goto(1, 1));
    }

    /// Prints a log by printing the last lines that fit in the screen.
    pub fn print_log(&self, log: &Log) {
        let (_c, r) = self.size;
        let rows = usize::try_from(r).unwrap();
        let len = log.len();
        let range = if len <= rows {
            0..len
        } else {
            let start = len - rows;

            start..len
        };

        for i in range {
            let id = log.entry_ids.get(i).unwrap();

            self.print(&log, id.clone());
        }
    }

    fn print(&self, log: &Log, id: String) {
        if let Some(line) = self.line_nr_for(&log, id.clone()) {
            let entry = log.get(id).unwrap();
            let color = self.color_for(entry);
            let (cols, _rows) = self.size;

            print!("{}", cursor::Goto(1, line));
            print!("{}", color::Fg(color));
            // print!("[{}/{}:s{}]", line, rows, log.len());
            entry.print(usize::try_from(cols).unwrap());
        }
    }

    fn color_for(&self, entry: &Entry) -> color::Rgb {
        match entry.status {
            HttpStatus::Info(_) => color::Rgb(0, 255, 0),
            HttpStatus::Success(_) => color::Rgb(0, 255, 0),
            HttpStatus::Redirect(_) => color::Rgb(0, 0, 255),
            HttpStatus::ClientError(_) => color::Rgb(255, 0, 255),
            HttpStatus::ServerError(_) => color::Rgb(255, 0, 0),
            HttpStatus::Unknown(_) => color::Rgb(255, 255, 0),
        }
    }

    fn line_nr_for(&self, log: &Log, id: String) -> Option<u16> {
        let entry = log.get(id)?;

        let log_size = u16::try_from(log.len()).unwrap();
        let (_, rows) = self.size;
        let order = u16::try_from(entry.order).unwrap();

        if log_size <= rows {
            Some(order)
        } else if order > log_size - rows {
            Some(order + rows - log_size)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Log;
    use super::Screen;

    #[test]
    fn test_line_nr_for() {
        let mut log = Log::new();
        let size = (30, 4);
        let screen = Screen::new_with_size(size);

        let uuid0 = log.add(log_start!("0")).unwrap();
        assert_eq!(screen.line_nr_for(&log, uuid0.clone()), Some(1));

        let uuid1 = log.add(log_start!("1")).unwrap();
        assert_eq!(screen.line_nr_for(&log, uuid1.clone()), Some(2));

        let uuid2 = log.add(log_start!("2")).unwrap();
        assert_eq!(screen.line_nr_for(&log, uuid0.clone()), None);
        assert_eq!(screen.line_nr_for(&log, uuid1.clone()), Some(1));
        assert_eq!(screen.line_nr_for(&log, uuid2.clone()), Some(2));

        let uuid3 = log.add(log_start!("3")).unwrap();
        assert_eq!(screen.line_nr_for(&log, uuid0), None);
        assert_eq!(screen.line_nr_for(&log, uuid1), None);
        assert_eq!(screen.line_nr_for(&log, uuid2), Some(1));
        assert_eq!(screen.line_nr_for(&log, uuid3), Some(2));
    }

    #[test]
    fn test_line_nr_for_on_updated_entries() {
        let mut log = Log::new();
        let size = (30, 4);
        let screen = Screen::new_with_size(size);

        let uuid0 = log.add(log_start!("0")).unwrap();
        let uuid1 = log.add(log_start!("1")).unwrap();
        log.add(log_start!("2")).unwrap();
        let uuid3 = log.add(log_start!("3")).unwrap();

        log.add(log_start!("4")).unwrap();
        let uuid4 = log.add(log_end!("4")).unwrap();
        let uuid2 = log.add(log_end!("2")).unwrap();

        assert_eq!(screen.line_nr_for(&log, uuid0), None);
        assert_eq!(screen.line_nr_for(&log, uuid1), None);
        assert_eq!(screen.line_nr_for(&log, uuid2), None);
        assert_eq!(screen.line_nr_for(&log, uuid3), Some(1));
        assert_eq!(screen.line_nr_for(&log, uuid4), Some(2));
    }
}
