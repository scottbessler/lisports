export interface PlayerStats {
  resource: string;
  parameters: PlayerStatsParameters;
  resultSets: ResultSet[];
}

export interface Standings {
  resource: string;
  parameters: unknown;
  resultSets: ResultSet[];
}

export interface ResultSet {
  name: string;
  headers: string[];
  rowSet: Row[];
}

export type Row = (number | string)[];

export interface PlayerStatsParameters {
  PerMode: string;
  PlusMinus: string;
  PaceAdjust: string;
  Rank: string;
  LeagueID: string;
  Season: string;
  SeasonType: string;
  PORound: number;
  PlayerID: number;
  Outcome?: any;
  Location?: any;
  Month: number;
  SeasonSegment?: any;
  DateFrom?: any;
  DateTo?: any;
  OpponentTeamID: number;
  VsConference?: any;
  VsDivision?: any;
  GameSegment?: any;
  Period: number;
  ShotClockRange?: any;
  LastNGames: number;
}

export const PLAYER_FIELD_DESCRIPTIONS: {
  [key: string]: { title: string; abbrev: string };
} = {
  AST_PCT: { title: "Assist Percentage", abbrev: "AST%" },
  GROUP_VALUE: { title: "Year", abbrev: "YR" },
  AST_RATIO: { title: "Assist Ratio", abbrev: "AST Ratio" },
  AST_TO: { title: "Assist to Turnover Ratio", abbrev: "AST/TO" },
  AST: { title: "Assists", abbrev: "AST" },
  BLK: { title: "Blocks", abbrev: "BLK" },
  BLKA: { title: "Blocks Against", abbrev: "BLKA" },
  DD2: { title: "Double Doubles", abbrev: "DD2" },
  DEF_RATING: { title: "Defensive Rating", abbrev: "DefRtg" },
  DREB_PCT: { title: "Defensive Rebounding Percentage", abbrev: "DREB%" },
  DREB: { title: "Defensive Rebounds", abbrev: "DREB" },
  EFG_PCT: { title: "Effective Field Goal Percentage", abbrev: "eFG%" },
  FG_PCT: { title: "Field Goal Percentage", abbrev: "FG%" },
  FG3_PCT: { title: "3 Point Field Goal Percentage", abbrev: "3P%" },
  FG3A: { title: "3 Point Field Goals Attempted", abbrev: "3PA" },
  FG3M: { title: "3 Point Field Goals Made", abbrev: "3PM" },
  FGA: { title: "Field Goals Attempted", abbrev: "FGA" },
  FGM: { title: "Field Goals Made", abbrev: "FGM" },
  FT_PCT: { title: "Free Throw Percentage", abbrev: "FT%" },
  FTA: { title: "Free Throws Attempted", abbrev: "FTA" },
  FTM: { title: "Free Throws Made", abbrev: "FTM" },
  GP: { title: "Games Played", abbrev: "GP" },
  MIN: { title: "Minutes Played", abbrev: "MIN" },
  NBA_FANTASY_PTS: { title: "Fantasy Points", abbrev: "FP" },
  NET_RATING: { title: "Net Rating", abbrev: "NetRtg" },
  OFF_RATING: { title: "Offensive Rating", abbrev: "OffRtg" },
  OPP_PTS_2ND_CHANCE: {
    title: "Opponent 2nd Chance Points",
    abbrev: "Opp<br>2nd PTS",
  },
  OPP_PTS_FB: { title: "Opponent Fast Break Points", abbrev: "Opp<br>FBPs" },
  OPP_PTS_OFF_TOV: {
    title: "Opponent Points Off Turnovers",
    abbrev: "Opp<br>PTS OFF TO",
  },
  OPP_PTS_PAINT: {
    title: "Opponent Points in the Paint",
    abbrev: "Opp<br>PITP",
  },
  OREB_PCT: { title: "Offensive Rebounding Percentage", abbrev: "OREB%" },
  OREB: { title: "Offensive Rebounds", abbrev: "OREB" },
  PACE: { title: "Pace", abbrev: "PACE" },
  PCT_AST_2PM: {
    title: "Percent of 2 Point Field Goals Made Assisted",
    abbrev: "2FGM<br>%AST",
  },
  PCT_AST_3PM: {
    title: "Percent of 3 Point Field Goals Made Assisted",
    abbrev: "3FGM<br>%AST",
  },
  PCT_AST_FGM: {
    title: "Percent of Point Field Goals Made Assisted",
    abbrev: "FGM<br>%AST",
  },
  PCT_AST: { title: "Percent of Team's Assists", abbrev: "%AST" },
  PCT_BLK: { title: "Percent of Team's Blocks", abbrev: "%BLK" },
  PCT_BLKA: {
    title: "Percent of Team's Blocked Field Goal Attempts",
    abbrev: "%BLKA",
  },
  PCT_DREB: { title: "Percent of Team's Defensive Rebounds", abbrev: "%DREB" },
  PCT_FG3A: {
    title: "Percent of Team's 3PT Field Goals Attempted",
    abbrev: "%3PA",
  },
  PCT_FG3M: { title: "Percent of Team's 3PT Field Goals Made", abbrev: "%3PM" },
  PCT_FGA_2PT: {
    title: "Percent of Field Goals Attempted (2 Pointers)",
    abbrev: "%FGA<br>2PT",
  },
  PCT_FGA_3PT: {
    title: "Percent of Field Goals Attempted (3 Pointers)",
    abbrev: "%FGA<br>3PT",
  },
  PCT_FGA: { title: "Percent of Team's Field Goals Attempted", abbrev: "%FGA" },
  PCT_FGM: { title: "Percent of Team's Field Goals Made", abbrev: "%FGM" },
  PCT_FTA: { title: "Percent of Team's Free Throws Attempted", abbrev: "%FTA" },
  PCT_FTM: { title: "Percent of Team's Free Throws Made", abbrev: "%FTM" },
  PCT_OREB: { title: "Percent of Team's Offensive Rebounds", abbrev: "%OREB" },
  PCT_PF: { title: "Percent of Team's Personal Fouls", abbrev: "%PF" },
  PCT_PFD: { title: "Percent of Team's Personal Fouls Drawn", abbrev: "%PFD" },
  PCT_PTS_2PT_MR: {
    title: "Percent of Points (Mid-Range)",
    abbrev: "%PTS<br>2PT MR",
  },
  PCT_PTS_2PT: {
    title: "Percent of Points (2 Pointers)",
    abbrev: "%PTS<br>2PT",
  },
  PCT_PTS_3PT: {
    title: "Percent of Points (3 Pointers)",
    abbrev: "%PTS<br>3PT",
  },
  PCT_PTS_FB: {
    title: "Percent of Points (Fast Break Points)",
    abbrev: "%PTS<br>FBPs",
  },
  PCT_PTS_FT: {
    title: "Percent of Points (Free Throws)",
    abbrev: "%PTS<br>FT",
  },
  PCT_PTS_OFF_TOV: {
    title: "Percent of Points (Off Turnovers)",
    abbrev: "%PTS<br>OffTO",
  },
  PCT_PTS_PAINT: {
    title: "Percent of Points (Points in the Paint)",
    abbrev: "%PTS<br>PITP",
  },
  PCT_PTS: { title: "Percent of Team's Points", abbrev: "%PTS" },
  PCT_REB: { title: "Percent of Team's Total Rebounds", abbrev: "%REB" },
  PCT_STL: { title: "Percent of Team's Steals", abbrev: "%STL" },
  PCT_TOV: { title: "Percent of Team's Turnovers", abbrev: "%TOV" },
  PCT_UAST_2PM: {
    title: "Percent of 2 Point Field Goals Made Unassisted",
    abbrev: "2FGM<br>%UAST",
  },
  PCT_UAST_3PM: {
    title: "Percent of 3 Point Field Goals Made Unassisted",
    abbrev: "3FGM<br>%UAST",
  },
  PCT_UAST_FGM: {
    title: "Percent of Point Field Goals Made Unassisted",
    abbrev: "FGM<br>%UAST",
  },
  PF: { title: "Personal Fouls", abbrev: "PF" },
  PFD: { title: "Personal Fouls Drawn", abbrev: "PFD" },
  PIE: { title: "Player Impact Estimate", abbrev: "PIE" },
  PLUS_MINUS: { title: "Plus-Minus", abbrev: "+/-" },
  PTS_2ND_CHANCE: { title: "Second Chance Points", abbrev: "2nd PTS" },
  PTS_FB: { title: "Fast Break Points", abbrev: "FBPs" },
  PTS_OFF_TOV: {
    title: "Points Off Turnovers",
    abbrev: "PTS OFF TO",
  },
  PTS_PAINT: { title: "Points in the Paint", abbrev: "PITP" },
  PTS: { title: "Points", abbrev: "PTS" },
  REB_PCT: { title: "Rebounding Percentage", abbrev: "REB%" },
  REB: { title: "Rebounds", abbrev: "REB" },
  STL: { title: "Steals", abbrev: "STL" },
  TD3: { title: "Triple Doubles", abbrev: "TD3" },
  TEAM_ABBREVIATION: { title: "Team", abbrev: "TEAM " },
  TM_TOV_PCT: { title: "Turnover Ratio", abbrev: "TO Ratio" },
  TOV: { title: "Turnovers", abbrev: "TOV" },
  TS_PCT: { title: "True Shooting Percentage", abbrev: "TS%" },
  USG_PCT: { title: "Usage Percentage", abbrev: "USG%" },
};

/*
 */

/*
<th class="Crom_text__NpR1_ Crom_primary__EajZu Crom_sticky__uYvkp" By: {Year">By Year</th><th class="Crom_text__NpR1_" TEAM_ABBREVIATION: {>TEAM </th><th sort="true" dir="D" GP: { title:"Games Played">GP</th><th sort="true" dir="D" MIN: { title:"Minutes Played">MIN</th><th sort="true" dir="D" PTS: { title:"Points">PTS</th><th sort="true" dir="D" FGM: { title:"Field Goals Made">FGM</th><th sort="true" dir="D" FGA: { title:"Field Goals Attempted">FGA</th><th sort="true" dir="D" FG_PCT: { title:"Field Goal Percentage">FG%</th><th sort="true" dir="D" FG3M: { title:"3 Point Field Goals Made">3PM</th><th sort="true" dir="D" FG3A: { title:"3 Point Field Goals Attempted">3PA</th><th sort="true" dir="D" FG3_PCT: { title:"3 Point Field Goal Percentage">3P%</th><th sort="true" dir="D" FTM: { title:"Free Throws Made">FTM</th><th sort="true" dir="D" FTA: { title:"Free Throws Attempted">FTA</th><th sort="true" dir="D" FT_PCT: { title:"Free Throw Percentage">FT%</th><th sort="true" dir="D" OREB: { title:"Offensive Rebounds">OREB</th><th sort="true" dir="D" DREB: { title:"Defensive Rebounds">DREB</th><th sort="true" dir="D" REB: { title:"Rebounds">REB</th><th sort="true" dir="D" AST: { title:"Assists">AST</th><th sort="true" dir="D" TOV: { title:"Turnovers">TOV</th><th sort="true" dir="D" STL: { title:"Steals">STL</th><th sort="true" dir="D" BLK: { title:"Blocks">BLK</th><th sort="true" dir="D" PF: { title:"Personal Fouls">PF</th><th sort="true" dir="D" NBA_FANTASY_PTS: { title:"Fantasy Points">FP</th><th sort="true" dir="D" DD2: { title:"Double Doubles">DD2</th><th sort="true" dir="D" TD3: { title:"Triple Doubles">TD3</th><th sort="true" dir="D" PLUS_MINUS: { title:"Plus-Minus">+/-</th> },
<th class="Crom_sticky__uYvkp Crom_text__NpR1_ Crom_primary__EajZu" By: {Year"><br>By Year</th><th class="Crom_text__NpR1_" TEAM_ABBREVIATION: {>TEAM </th><th sort="true" dir="D" GP: { title:"Games Played">GP</th><th sort="true" dir="D" MIN: { title:"Minutes Played">MIN</th><th sort="true" dir="D" PTS_OFF_TOV: { title:"Points Off Turnovers">PTS OFF TO</th><th sort="true" dir="D" PTS_2ND_CHANCE: { title:"Second Chance Points">2nd PTS</th><th sort="true" dir="D" PTS_FB: { title:"Fast Break Points">FBPs</th><th sort="true" dir="D" PTS_PAINT: { title:"Points in the Paint" sorted="D">PITP</th><th sort="true" dir="D" OPP_PTS_OFF_TOV: { title:"Opponent Points Off Turnovers">Opp<br>PTS OFF TO</th><th sort="true" dir="D" OPP_PTS_2ND_CHANCE: { title:"Opponent 2nd Chance Points">Opp<br>2nd PTS</th><th sort="true" dir="D" OPP_PTS_FB: { title:"Opponent Fast Break Points">Opp<br>FBPs</th><th sort="true" dir="D" OPP_PTS_PAINT: { title:"Opponent Points in the Paint">Opp<br>PITP</th><th sort="true" dir="D" BLK: { title:"Blocks">BLK</th><th sort="true" dir="D" BLKA: { title:"Blocks Against">BLKA</th><th sort="true" dir="D" PF: { title:"Personal Fouls">PF</th><th sort="true" dir="D" PFD: { title:"Personal Fouls Drawn">PFD</th> },
<th class="Crom_text__NpR1_ Crom_primary__EajZu Crom_sticky__uYvkp" By: {Year"><br>By Year</th><th class="Crom_text__NpR1_" TEAM_ABBREVIATION: {>TEAM </th><th sort="true" dir="D" GP: { title:"Games Played">GP</th><th sort="true" dir="D" MIN: { title:"Minutes Played">MIN</th><th sort="true" dir="D" PCT_FGA_2PT: { title:"Percent of Field Goals Attempted (2 Pointers)">%FGA<br>2PT</th><th sort="true" dir="D" PCT_FGA_3PT: { title:"Percent of Field Goals Attempted (3 Pointers)">%FGA<br>3PT</th><th sort="true" dir="D" PCT_PTS_2PT: { title:"Percent of Points (2 Pointers)">%PTS<br>2PT</th><th sort="true" dir="D" PCT_PTS_2PT_MR: { title:"Percent of Points (Mid-Range)">%PTS<br>2PT MR</th><th sort="true" dir="D" PCT_PTS_3PT: { title:"Percent of Points (3 Pointers)">%PTS<br>3PT</th><th sort="true" dir="D" PCT_PTS_FB: { title:"Percent of Points (Fast Break Points)">%PTS<br>FBPs</th><th sort="true" dir="D" PCT_PTS_FT: { title:"Percent of Points (Free Throws)">%PTS<br>FT</th><th sort="true" dir="D" PCT_PTS_OFF_TOV: { title:"Percent of Points (Off Turnovers)">%PTS<br>OffTO</th><th sort="true" dir="D" PCT_PTS_PAINT: { title:"Percent of Points (Points in the Paint)">%PTS<br>PITP</th><th sort="true" dir="D" PCT_AST_2PM: { title:"Percent of 2 Point Field Goals Made Assisted">2FGM<br>%AST</th><th sort="true" dir="D" PCT_UAST_2PM: { title:"Percent of 2 Point Field Goals Made Unassisted">2FGM<br>%UAST</th><th sort="true" dir="D" PCT_AST_3PM: { title:"Percent of 3 Point Field Goals Made Assisted">3FGM<br>%AST</th><th sort="true" dir="D" PCT_UAST_3PM: { title:"Percent of 3 Point Field Goals Made Unassisted">3FGM<br>%UAST</th><th sort="true" dir="D" PCT_AST_FGM: { title:"Percent of Point Field Goals Made Assisted">FGM<br>%AST</th><th sort="true" dir="D" PCT_UAST_FGM: { title:"Percent of Point Field Goals Made Unassisted">FGM<br>%UAST</th> },
<th class="Crom_text__NpR1_ Crom_primary__EajZu Crom_sticky__uYvkp" By: {Year">By Year</th><th class="Crom_text__NpR1_" TEAM_ABBREVIATION: {>TEAM </th><th sort="true" dir="D" GP: { title:"Games Played">GP</th><th sort="true" dir="D" MIN: { title:"Minutes Played">MIN</th><th sort="true" dir="D" USG_PCT: { title:"Usage Percentage">USG%</th><th sort="true" dir="D" PCT_FGM: { title:"Percent of Team's Field Goals Made">%FGM</th><th sort="true" dir="D" PCT_FGA: { title:"Percent of Team's Field Goals Attempted">%FGA</th><th sort="true" dir="D" PCT_FG3M: { title:"Percent of Team's 3PT Field Goals Made">%3PM</th><th sort="true" dir="D" PCT_FG3A: { title:"Percent of Team's 3PT Field Goals Attempted">%3PA</th><th sort="true" dir="D" PCT_FTM: { title:"Percent of Team's Free Throws Made">%FTM</th><th sort="true" dir="D" PCT_FTA: { title:"Percent of Team's Free Throws Attempted">%FTA</th><th sort="true" dir="D" PCT_OREB: { title:"Percent of Team's Offensive Rebounds">%OREB</th><th sort="true" dir="D" PCT_DREB: { title:"Percent of Team's Defensive Rebounds">%DREB</th><th sort="true" dir="D" PCT_REB: { title:"Percent of Team's Total Rebounds">%REB</th><th sort="true" dir="D" PCT_AST: { title:"Percent of Team's Assists">%AST</th><th sort="true" dir="D" PCT_TOV: { title:"Percent of Team's Turnovers">%TOV</th><th sort="true" dir="D" PCT_STL: { title:"Percent of Team's Steals">%STL</th><th sort="true" dir="D" PCT_BLK: { title:"Percent of Team's Blocks">%BLK</th><th sort="true" dir="D" PCT_BLKA: { title:"Percent of Team's Blocked Field Goal Attempts">%BLKA</th><th sort="true" dir="D" PCT_PF: { title:"Percent of Team's Personal Fouls">%PF</th><th sort="true" dir="D" PCT_PFD: { title:"Percent of Team's Personal Fouls Drawn">%PFD</th><th sort="true" dir="D" PCT_PTS: { title:"Percent of Team's Points">%PTS</th> },
*/

/*
 */
