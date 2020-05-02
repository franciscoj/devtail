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
    fn test_line_nr_for_when_all_fit() {
        let mut log = Log::new();
        let uuid0 = log.add(log_start!("0")).unwrap();
        let uuid1 = log.add(log_start!("1")).unwrap();
        let size = (1, 2);

        let screen = Screen::new(size);

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

        let screen = Screen::new(size);

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

        let screen = Screen::new(size);

        assert_eq!(screen.line_nr_for(&log, uuids[3].clone()), None);
        assert_eq!(screen.line_nr_for(&log, uuids[7].clone()), Some(2));
    }
}
