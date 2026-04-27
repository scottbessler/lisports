use crate::models::{Statistics, TeamStatistics};

pub fn points_per_shot(stats: &Statistics) -> Option<f64> {
    if stats.field_goals_attempted <= 0 {
        return None;
    }
    Some(((stats.points as f64 * 100.0) / stats.field_goals_attempted as f64).round() / 100.0)
}

pub fn usage_rate(stats: &Statistics, team: &TeamStatistics) -> Option<i64> {
    let minutes = stats.minutes as f64;
    if minutes <= 0.0 {
        return None;
    }

    let numerator = (stats.field_goals_attempted as f64
        + 0.44 * stats.free_throws_attempted as f64
        + stats.turnovers as f64)
        * (team.minutes as f64 / 5.0);
    let denominator = minutes
        * (team.field_goals_attempted as f64
            + 0.44 * team.free_throws_attempted as f64
            + team.turnovers as f64);

    if denominator == 0.0 {
        None
    } else {
        Some((100.0 * numerator / denominator).round() as i64)
    }
}

pub fn pie(stats: &Statistics, team: &TeamStatistics, other: &TeamStatistics) -> i64 {
    let player =
        stats.points as f64 + stats.field_goals_made as f64 + stats.free_throws_made as f64
            - stats.field_goals_attempted as f64
            - stats.free_throws_attempted as f64
            + stats.rebounds_defensive as f64
            + stats.rebounds_offensive as f64 / 2.0
            + stats.assists as f64 / 2.0
            + stats.steals as f64
            + stats.blocks as f64 / 2.0
            - stats.blocks_received as f64 / 2.0
            - stats.fouls_personal as f64
            - stats.turnovers as f64;

    let total = team.points as f64
        + other.points as f64
        + team.field_goals_made as f64
        + other.field_goals_made as f64
        + team.free_throws_made as f64
        + other.free_throws_made as f64
        - team.field_goals_attempted as f64
        - other.field_goals_attempted as f64
        - team.free_throws_attempted as f64
        - other.free_throws_attempted as f64
        + team.rebounds_defensive as f64
        + other.rebounds_defensive as f64
        + (team.rebounds_offensive + other.rebounds_offensive) as f64 / 2.0
        + (team.assists + other.assists) as f64 / 2.0
        + team.steals as f64
        + other.steals as f64
        + (team.blocks + other.blocks) as f64 / 2.0
        - stats.blocks_received as f64 / 2.0
        - team.fouls_personal as f64
        - other.fouls_personal as f64
        - team.turnovers as f64
        - other.turnovers as f64;

    if total == 0.0 {
        0
    } else {
        (100.0 * player / total).round() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_per_shot_handles_zero_attempts() {
        assert_eq!(points_per_shot(&Statistics::default()), None);
    }

    #[test]
    fn points_per_shot_rounds_to_two_decimals() {
        let stats = Statistics {
            points: 25,
            field_goals_attempted: 12,
            ..Statistics::default()
        };
        assert_eq!(points_per_shot(&stats), Some(2.08));
    }
}
