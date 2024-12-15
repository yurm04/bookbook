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
    let parsed_date =
        chrono::NaiveDate::parse_from_str(&date_arg, "%m-%d-%Y").expect("Failed to parse date");

    let utc_date = parsed_date.and_hms_opt(0, 0, 0);

    if utc_date.is_none() {
        return Err("Failed to parse date".to_string());
    }

    Ok(DateTime::<Utc>::from_naive_utc_and_offset(utc_date.expect("Sho"), Utc).to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn mock_now() -> String {
        "10-12-2021".to_string()
    }

    #[test]
    fn parse_date_with_empty_date() {
        assert_eq!(
            Ok("10-12-2021".to_string()),
            parse_date(None, true, mock_now)
        );
        assert_eq!(Ok("".to_string()), parse_date(None, false, mock_now));
    }
}
