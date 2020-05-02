use super::entry::Entry;
use super::log::Log;
use std::convert::TryFrom;

type Size = (u16, u16);

struct Screen {
    size: Size,
}

impl Screen {
    pub fn new(size: Size) -> Self {
        Self { size }
    }

    fn line_for(&self, log: &Log, id: String) -> Option<u16> {
        let entry = log.get(id)?;

        // let size = log.len();
        // let (_, rows) = self.size;
        let order = u16::try_from(entry.order).unwrap();

        Some(order)
    }
}

#[cfg(test)]
mod tests {
    use super::Log;
    use super::Screen;

    #[test]
    fn test_line_for_when_all_fit() {
        let mut log = Log::new();
        let uuid0 = log.add(log_start!("0")).unwrap();
        let uuid1 = log.add(log_start!("1")).unwrap();
        let size = (1, 2);

        let screen = Screen::new(size);

        assert_eq!(screen.line_for(&log, uuid0), Some(1));
        assert_eq!(screen.line_for(&log, uuid1), Some(2));
    }
}
