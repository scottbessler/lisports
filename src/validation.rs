use chrono::NaiveDate;

use crate::error::AppError;

pub fn parse_day(input: &str) -> Result<NaiveDate, AppError> {
    NaiveDate::parse_from_str(input, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("day must use YYYY-MM-DD format".to_string()))
}

pub fn numeric_id(input: &str, name: &str) -> Result<String, AppError> {
    if !input.is_empty() && input.chars().all(|c| c.is_ascii_digit()) {
        Ok(input.to_string())
    } else {
        Err(AppError::BadRequest(format!("{name} must be a numeric id")))
    }
}

/// A team reference for ESPN team endpoints, which accept either a numeric team
/// id or the team abbreviation (e.g. `bos`). Normalized to lowercase.
pub fn team_ref(input: &str) -> Result<String, AppError> {
    if !input.is_empty() && input.chars().all(|c| c.is_ascii_alphanumeric()) {
        Ok(input.to_ascii_lowercase())
    } else {
        Err(AppError::BadRequest(
            "team_id must be alphanumeric".to_string(),
        ))
    }
}

pub fn nfl_week(input: &str) -> Result<i64, AppError> {
    let week = input
        .parse::<i64>()
        .map_err(|_| AppError::BadRequest("week must be a number from 1 to 23".to_string()))?;
    if (1..=23).contains(&week) {
        Ok(week)
    } else {
        Err(AppError::BadRequest(
            "week must be a number from 1 to 23".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_accepts_ymd() {
        assert!(parse_day("2026-04-26").is_ok());
    }

    #[test]
    fn day_rejects_bad_format() {
        assert!(parse_day("04/26/2026").is_err());
    }

    #[test]
    fn ids_must_be_numeric() {
        assert!(numeric_id("401869385", "game_id").is_ok());
        assert!(numeric_id("abc", "game_id").is_err());
    }

    #[test]
    fn team_refs_accept_ids_and_tricodes() {
        assert_eq!(team_ref("BOS").unwrap(), "bos");
        assert_eq!(team_ref("2").unwrap(), "2");
        assert!(team_ref("").is_err());
        assert!(team_ref("a b").is_err());
    }

    #[test]
    fn nfl_weeks_must_be_regular_season_weeks() {
        assert!(nfl_week("1").is_ok());
        assert!(nfl_week("23").is_ok());
        assert!(nfl_week("0").is_err());
        assert!(nfl_week("24").is_err());
        assert!(nfl_week("abc").is_err());
    }
}
