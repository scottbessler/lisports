#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeagueId {
    Nba,
    Wnba,
    Mlb,
    Nfl,
    Nhl,
    WorldCup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleBucket {
    Date,
    Week,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerFeature {
    Supported,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct League {
    pub id: LeagueId,
    pub slug: &'static str,
    pub nav_label: &'static str,
    pub route_base: &'static str,
    pub sport_path: &'static str,
    pub league_path: &'static str,
    pub bucket: ScheduleBucket,
    pub logo_path: &'static str,
    pub scoreboard: bool,
    pub game: bool,
    pub standings: bool,
    pub player: PlayerFeature,
}

pub const LEAGUES: &[League] = &[
    League {
        id: LeagueId::Nba,
        slug: "nba",
        nav_label: "NBA",
        route_base: "/nba",
        sport_path: "basketball",
        league_path: "nba",
        bucket: ScheduleBucket::Date,
        logo_path: "https://cdn.nba.com/logos/nba/{team_id}/primary/L/logo.svg",
        scoreboard: true,
        game: true,
        standings: true,
        player: PlayerFeature::Supported,
    },
    League {
        id: LeagueId::Wnba,
        slug: "wnba",
        nav_label: "WNBA",
        route_base: "/wnba",
        sport_path: "basketball",
        league_path: "wnba",
        bucket: ScheduleBucket::Date,
        logo_path: "https://a.espncdn.com/i/teamlogos/wnba/500/{tricode}.png",
        scoreboard: true,
        game: true,
        standings: true,
        player: PlayerFeature::Supported,
    },
    League {
        id: LeagueId::Mlb,
        slug: "mlb",
        nav_label: "MLB",
        route_base: "/mlb",
        sport_path: "baseball",
        league_path: "mlb",
        bucket: ScheduleBucket::Date,
        logo_path: "https://a.espncdn.com/i/teamlogos/mlb/500/{tricode}.png",
        scoreboard: true,
        game: true,
        standings: true,
        player: PlayerFeature::Supported,
    },
    League {
        id: LeagueId::Nfl,
        slug: "nfl",
        nav_label: "NFL",
        route_base: "/nfl",
        sport_path: "football",
        league_path: "nfl",
        bucket: ScheduleBucket::Week,
        logo_path: "https://a.espncdn.com/i/teamlogos/nfl/500/{tricode}.png",
        scoreboard: true,
        game: true,
        standings: true,
        player: PlayerFeature::Supported,
    },
    League {
        id: LeagueId::Nhl,
        slug: "nhl",
        nav_label: "NHL",
        route_base: "/nhl",
        sport_path: "hockey",
        league_path: "nhl",
        bucket: ScheduleBucket::Date,
        logo_path: "https://a.espncdn.com/i/teamlogos/nhl/500/{tricode}.png",
        scoreboard: true,
        game: true,
        standings: true,
        player: PlayerFeature::Supported,
    },
    League {
        id: LeagueId::WorldCup,
        slug: "worldcup",
        nav_label: "World Cup",
        route_base: "/worldcup",
        sport_path: "soccer",
        league_path: "fifa.world",
        bucket: ScheduleBucket::Date,
        logo_path: "https://a.espncdn.com/i/teamlogos/countries/500/{tricode}.png",
        scoreboard: true,
        game: true,
        standings: true,
        player: PlayerFeature::Unsupported,
    },
];

pub const DEFAULT_LEAGUE_SLUG: &str = "worldcup";

pub fn all() -> &'static [League] {
    LEAGUES
}

pub fn by_slug(slug: &str) -> Option<&'static League> {
    LEAGUES.iter().find(|league| league.slug == slug)
}

pub fn default_league() -> &'static League {
    by_slug(DEFAULT_LEAGUE_SLUG).expect("default league registry entry")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_declares_all_leagues() {
        assert_eq!(LEAGUES.len(), 6);
        assert!(by_slug("nba").is_some());
        assert!(by_slug("wnba").is_some());
        assert!(by_slug("mlb").is_some());
        assert!(by_slug("nfl").is_some());
        assert!(by_slug("nhl").is_some());
        assert!(by_slug("worldcup").is_some());
    }

    #[test]
    fn registry_declares_supported_surfaces() {
        for league in LEAGUES {
            assert!(league.scoreboard);
            assert!(league.game);
            assert!(league.standings);
            assert!(!league.route_base.is_empty());
            assert!(!league.sport_path.is_empty());
            assert!(!league.league_path.is_empty());
            assert!(!league.logo_path.is_empty());
        }
        assert_eq!(by_slug("nba").unwrap().player, PlayerFeature::Supported);
        assert_eq!(by_slug("wnba").unwrap().player, PlayerFeature::Supported);
        assert_eq!(by_slug("mlb").unwrap().player, PlayerFeature::Supported);
        assert_eq!(by_slug("nfl").unwrap().player, PlayerFeature::Supported);
        assert_eq!(by_slug("nhl").unwrap().player, PlayerFeature::Supported);
        assert_eq!(
            by_slug("worldcup").unwrap().player,
            PlayerFeature::Unsupported
        );
    }

    #[test]
    fn default_league_is_world_cup_first() {
        assert_eq!(default_league().slug, "worldcup");
        assert_eq!(default_league().route_base, "/worldcup");
    }
}
