use chrono::{DateTime, Utc};

pub fn parse_date<F>(date: Option<String>, default_date: bool, now_fn: F) -> Result<String, String>
where
    F: Fn() -> String,
{
    match date {
        Some(d) => convert_date_to_utc_string(d),
        None => match default_date {
            true => Ok(now_fn()),
            false => Ok("".to_string()),
        },
    }
}

pub fn now() -> String {
    Utc::now().to_string()
}

pub fn convert_date_to_utc_string(date_arg: String) -> Result<String, String> {
    let parsed_date = chrono::NaiveDate::parse_from_str(&date_arg, "%m-%d-%Y");

    let date = match parsed_date.is_err() {
        true => return Err("Unable to parse from string".to_string()),
        false => parsed_date.unwrap(),
    };

    let utc_date = date.and_hms_opt(0, 0, 0);

    match utc_date.is_none() {
        true => return Err("Unable to convert to UTC".to_string()),
        false => {
            return Ok(
                DateTime::<Utc>::from_naive_utc_and_offset(utc_date.expect("Sho"), Utc).to_string(),
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const DATE_INPUT: &str = "09-09-2023";
    const DATE_OUTPUT: &str = "2023-09-09 00:00:00 UTC";

    fn mock_now() -> String {
        DATE_OUTPUT.to_string()
    }

    #[test]
    fn parse_date_with_empty_date() {
        assert_eq!(
            Ok(DATE_OUTPUT.to_string()),
            parse_date(None, true, mock_now)
        );
        assert_eq!(Ok("".to_string()), parse_date(None, false, mock_now));
    }

    #[test]
    fn parse_date_with_date() {
        assert_eq!(
            Ok(DATE_OUTPUT.to_string()),
            parse_date(Some(DATE_INPUT.to_string()), true, mock_now),
        )
    }

    #[test]
    fn convert_date_to_utc_string_valid_input() {
        assert_eq!(
            Ok(DATE_OUTPUT.to_string()),
            convert_date_to_utc_string(DATE_INPUT.to_string())
        );
    }

    #[test]
    fn convert_date_to_utc_string_invalid_input() {
        assert_eq!(
            Err("Unable to parse from string".to_string()),
            convert_date_to_utc_string("foobar".to_string())
        )
    }
}
