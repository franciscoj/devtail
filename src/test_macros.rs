macro_rules! uuid {
        ($id: expr) => {
            format!("{0}{0}{0}{0}{0}{0}{0}{0}-{0}{0}{0}{0}-{0}{0}{0}{0}-{0}{0}{0}{0}-{0}{0}{0}{0}{0}{0}{0}{0}{0}{0}{0}{0}", $id)
        };
    }

macro_rules! log_line {
    ($id:expr, $text:expr) => {{
        format!("[{}] {}", uuid!($id), $text)
    }};
}

macro_rules! log_start {
    ($id:expr) => {
        log_line!($id, format!("GET /endpoint_{}", $id))
    };
}

macro_rules! log_end {
    ($id:expr) => {
        log_line!($id, "Completed 200")
    };

    ($id:expr, $status: expr) => {
        log_line!($id, format!("Completed {}", $status))
    };
}
