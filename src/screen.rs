use super::entry::Entry;
use super::log::Log;
use super::HttpStatus;
use std::convert::TryFrom;
use termion::{clear, color, cursor, terminal_size};

type Size = (u16, u16);

pub struct Screen {
    size: Size,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            size: terminal_size().unwrap(),
        }
    }

    pub fn new_with_size(size: Size) -> Self {
        Self { size }
    }

    pub fn clear(&self) {
        println!("{}{}", clear::All, cursor::Goto(1, 1));
    }

    pub fn print(&self, log: &Log, id: String) {
        if let Some(line) = self.line_nr_for(&log, id.clone()) {
            let entry = log.get(id.clone()).unwrap();
            let color = self.color_for(entry);

            print!("{}", cursor::Goto(1, line));
            print!("{}", color::Fg(color));
            entry.print();
        }
    }

    fn color_for(&self, entry: &Entry) -> color::Rgb {
        match entry.status {
            HttpStatus::Success(_) => color::Rgb(0, 255, 0),
            HttpStatus::Redirect(_) => color::Rgb(0, 0, 255),
            HttpStatus::ClientError(_) => color::Rgb(255, 0, 0),
            HttpStatus::ServerError(_) => color::Rgb(255, 0, 0),
            HttpStatus::Unknown(_) => color::Rgb(255, 255, 0),
        }
    }

    fn line_nr_for(&self, log: &Log, id: String) -> Option<u16> {
        let entry = log.get(id)?;

        let log_size = u16::try_from(log.len()).unwrap();
        let (_, rows) = self.size;
        let order = u16::try_from(entry.order).unwrap();

        // TODO: This is wrong and there's a bug here somehow. When the size of the log overflows
        // the screen's size, then the cursor get's kind of crazy and things aren't printed where
        // they should.
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
    fn test_line_nr_for_when_all_fit() {
        let mut log = Log::new();
        let uuid0 = log.add(log_start!("0")).unwrap();
        let uuid1 = log.add(log_start!("1")).unwrap();
        let size = (1, 2);

        let screen = Screen::new_with_size(size);

        assert_eq!(screen.line_nr_for(&log, uuid0), Some(1));
        assert_eq!(screen.line_nr_for(&log, uuid1), Some(2));
    }

    #[test]
    fn test_line_nr_for_when_lines_out_of_screen() {
        let mut log = Log::new();
        let uuids = vec![
            log.add(log_start!("0")).unwrap(),
            log.add(log_start!("1")).unwrap(),
            log.add(log_start!("2")).unwrap(),
        ];

        let size = (1, 2);

        let screen = Screen::new_with_size(size);

        assert_eq!(screen.line_nr_for(&log, uuids[0].clone()), None);
        assert_eq!(screen.line_nr_for(&log, uuids[1].clone()), Some(1));
        assert_eq!(screen.line_nr_for(&log, uuids[2].clone()), Some(2));
    }

    #[test]
    fn test_line_nr_for_on_many_lines() {
        let mut log = Log::new();
        let uuids = vec![
            log.add(log_start!("0")).unwrap(),
            log.add(log_start!("1")).unwrap(),
            log.add(log_start!("2")).unwrap(),
            log.add(log_start!("3")).unwrap(),
            log.add(log_start!("4")).unwrap(),
            log.add(log_start!("5")).unwrap(),
            log.add(log_start!("6")).unwrap(),
            log.add(log_start!("7")).unwrap(),
        ];
        let size = (1, 2);

        let screen = Screen::new_with_size(size);

        assert_eq!(screen.line_nr_for(&log, uuids[3].clone()), None);
        assert_eq!(screen.line_nr_for(&log, uuids[7].clone()), Some(2));
    }
}
