//! These are test macros. They can be used in case you need an UUID or a log line instead of
//! writing it by hand.

/// Returns a literal for a UUID like "00000000-0000-0000-0000-000000000000" just with the
/// character given. It does no check or validation so it is up to the developer to provide UUID
/// valid characters
///
/// # Examples
///
/// ```
/// assert_eq!(uuid!("0"), "00000000-0000-0000-0000-000000000000");
/// ```
#[allow(unused_macros)]
macro_rules! uuid {
        ($id: expr) => {
            format!("{0}{0}{0}{0}{0}{0}{0}{0}-{0}{0}{0}{0}-{0}{0}{0}{0}-{0}{0}{0}{0}-{0}{0}{0}{0}{0}{0}{0}{0}{0}{0}{0}{0}", $id)
        };
    }

/// Returns a `String` for with an UUID.
///
/// # Examples
///
/// ```
/// assert_eq!(log_line!("0", "Some line"), String::from("[00000000-0000-0000-0000-000000000000] Some line"));
/// ```
#[allow(unused_macros)]
macro_rules! log_line {
    ($id:expr, $text:expr) => {{
        String::from(format!("[{}] {}", uuid!($id), $text))
    }};
}

/// Returns a String with a line for a `GET /endpoint_id`
///
/// # Examples
///
/// ```
/// assert_eq!(log_start!("0"), String::from("[00000000-0000-0000-0000-000000000000] GET /endpoint_0"));
/// ```
#[allow(unused_macros)]
macro_rules! log_start {
    ($id:expr) => {
        log_line!($id, format!("GET /endpoint_{}", $id))
    };
}

/// Returns a String with a line for a `Completed XXX`, the HTTP status can be chosen.
///
/// # Examples
///
/// ```
/// assert_eq!(log_end!("0"), String::from("[00000000-0000-0000-0000-000000000000] Completed 200"));
/// assert_eq!(log_end!("0", 301), String::from("[00000000-0000-0000-0000-000000000000] Completed 301"));
/// ```
#[allow(unused_macros)]
macro_rules! log_end {
    ($id:expr) => {
        log_line!($id, "Completed 200")
    };

    ($id:expr, $status: expr) => {
        log_line!($id, format!("Completed {}", $status))
    };
}
