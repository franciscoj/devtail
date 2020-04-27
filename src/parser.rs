use regex::Regex;

// Got this from here: https://stackoverflow.com/a/6640851/233720
const UUID_REGEX: &str =
    r"\[(\b[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}\b)\].*";

const HTTP_STATUS_REGEX: &str = r"Completed ([1-5]+[0-9]{2})";

#[derive(Debug, PartialEq)]
enum ParseResult<'a> {
    Id(&'a str),
    Success(&'a str, u16),
    Redirect(&'a str, u16),
    ClientError(&'a str, u16),
    ServerError(&'a str, u16),
}

fn parse(line: &str) -> Option<ParseResult<'_>> {
    let uuid_regex = Regex::new(UUID_REGEX).unwrap();
    let captures = uuid_regex.captures(&line)?;
    let id_match = captures.get(1)?;
    let id = id_match.as_str();

    if let Some(http_status) = parse_http_status(line) {
        match http_status {
            200..=299 => Some(ParseResult::Success(id, http_status)),
            300..=399 => Some(ParseResult::Redirect(id, http_status)),
            400..=499 => Some(ParseResult::ClientError(id, http_status)),
            500..=599 => Some(ParseResult::ServerError(id, http_status)),
            _ => None,
        }
    } else {
        Some(ParseResult::Id(id))
    }
}

fn parse_http_status(line: &str) -> Option<u16> {
    let http_status_regex = Regex::new(HTTP_STATUS_REGEX).unwrap();
    let captures = http_status_regex.captures(&line)?;
    let status_match = captures.get(1)?;
    let status = status_match.as_str().to_string();

    Some(status.parse::<u16>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_uuid() {
        let line = r#"[df7f9091-18d5-4002-91c9-e084516526ab] Started POST "/visits" for 127.0.0.1 at 2020-04-18 17:50:07 +0200"#;

        assert_eq!(
            ParseResult::Id("df7f9091-18d5-4002-91c9-e084516526ab"),
            parse(line).unwrap()
        );
    }

    #[test]
    fn parse_uuid_on_line_with_brackets() {
        let line = r#"[be155bd9-587d-468a-994f-441815edc79d]  CACHE MyModel Load (0.0ms)  SELECT  `my_models`.* FROM `my_models` WHERE `my_models`.`id` = 1 LIMIT 1  [["id", 1], ["LIMIT", 1]]"#;

        assert_eq!(
            ParseResult::Id("be155bd9-587d-468a-994f-441815edc79d"),
            parse(line).unwrap()
        );
    }

    #[test]
    fn parse_uuid_and_status() {
        let line = r"[df7f9091-18d5-4002-91c9-e084516526ab] Completed 200 OK in 21ms (Views: 0.1ms | ActiveRecord: 8.0ms)";

        assert_eq!(
            ParseResult::Success("df7f9091-18d5-4002-91c9-e084516526ab", 200),
            parse(line).unwrap()
        )
    }
}
