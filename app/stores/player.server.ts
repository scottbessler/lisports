import type { PlayerStats } from "../models/PlayerStats";
import { getJSON, successOrThrow } from "../reqs";
import { NBAStatsRequestInit } from "./scoreboard.server";
import { fetchFromCache, saveToCache } from "./simpleCache.server";

export async function fetchPlayerStats(playerId: string): Promise<PlayerStats> {
  const url = `https://stats.nba.com/stats/playerdashboardbyyearoveryearcombined?DateFrom=&DateTo=&GameSegment=&LastNGames=0&LeagueID=00&Location=&MeasureType=Base&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerID=${playerId}&PlusMinus=N&Rank=N&Season=2022-23&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&VsConference=&VsDivision=`;

  const cacheKey = `player:${playerId}`;
  const cacheResult = await fetchFromCache(cacheKey);
  if (cacheResult != null) {
    // todo: validate response?
    return cacheResult as unknown as PlayerStats;
  }

  const result = successOrThrow<PlayerStats>(
    await getJSON(url, NBAStatsRequestInit)
  );

  await saveToCache(cacheKey, result);
  return result;
}

/**
 * 
 * 
 * game logs
 * 
 * {
	"headers": [
		"SEASON_YEAR",
		"PLAYER_ID",
		"PLAYER_NAME",
		"NICKNAME",
		"TEAM_ID",
		"TEAM_ABBREVIATION",
		"TEAM_NAME",
		"GAME_ID",
		"GAME_DATE",
		"MATCHUP",
		"WL",
		"MIN",
		"E_OFF_RATING",
		"OFF_RATING",
		"sp_work_OFF_RATING",
		"E_DEF_RATING",
		"DEF_RATING",
		"sp_work_DEF_RATING",
		"E_NET_RATING",
		"NET_RATING",
		"sp_work_NET_RATING",
		"AST_PCT",
		"AST_TO",
		"AST_RATIO",
		"OREB_PCT",
		"DREB_PCT",
		"REB_PCT",
		"TM_TOV_PCT",
		"E_TOV_PCT",
		"EFG_PCT",
		"TS_PCT",
		"USG_PCT",
		"E_USG_PCT",
		"E_PACE",
		"PACE",
		"PACE_PER40",
		"sp_work_PACE",
		"PIE",
		"POSS",
		"FGM",
		"FGA",
		"FGM_PG",
		"FGA_PG",
		"FG_PCT",
		"GP_RANK",
		"W_RANK",
		"L_RANK",
		"W_PCT_RANK",
		"MIN_RANK",
		"E_OFF_RATING_RANK",
		"OFF_RATING_RANK",
		"sp_work_OFF_RATING_RANK",
		"E_DEF_RATING_RANK",
		"DEF_RATING_RANK",
		"sp_work_DEF_RATING_RANK",
		"E_NET_RATING_RANK",
		"NET_RATING_RANK",
		"sp_work_NET_RATING_RANK",
		"AST_PCT_RANK",
		"AST_TO_RANK",
		"AST_RATIO_RANK",
		"OREB_PCT_RANK",
		"DREB_PCT_RANK",
		"REB_PCT_RANK",
		"TM_TOV_PCT_RANK",
		"E_TOV_PCT_RANK",
		"EFG_PCT_RANK",
		"TS_PCT_RANK",
		"USG_PCT_RANK",
		"E_USG_PCT_RANK",
		"E_PACE_RANK",
		"PACE_RANK",
		"sp_work_PACE_RANK",
		"PIE_RANK",
		"FGM_RANK",
		"FGA_RANK",
		"FGM_PG_RANK",
		"FGA_PG_RANK",
		"FG_PCT_RANK",
		"VIDEO_AVAILABLE_FLAG"
	]


  "SEASON_YEAR",
  "PLAYER_ID",
  "PLAYER_NAME",
  "NICKNAME",
  "TEAM_ID",
  "TEAM_ABBREVIATION",
  "TEAM_NAME",
  "GAME_ID",
  "GAME_DATE",
  "MATCHUP",
  "WL",
  "MIN",
  "FGM",
  "FGA",
  "FG_PCT",
  "FG3M",
  "FG3A",
  "FG3_PCT",
  "FTM",
  "FTA",
  "FT_PCT",
  "OREB",
  "DREB",
  "REB",
  "AST",
  "TOV",
  "STL",
  "BLK",
  "BLKA",
  "PF",
  "PFD",
  "PTS",
  "PLUS_MINUS",
  "NBA_FANTASY_PTS",
  "DD2",
  "TD3",
  "WNBA_FANTASY_PTS",
  "GP_RANK",
  "W_RANK",
  "L_RANK",
  "W_PCT_RANK",
  "MIN_RANK",
  "FGM_RANK",
  "FGA_RANK",
  "FG_PCT_RANK",
  "FG3M_RANK",
  "FG3A_RANK",
  "FG3_PCT_RANK",
  "FTM_RANK",
  "FTA_RANK",
  "FT_PCT_RANK",
  "OREB_RANK",
  "DREB_RANK",
  "REB_RANK",
  "AST_RANK",
  "TOV_RANK",
  "STL_RANK",
  "BLK_RANK",
  "BLKA_RANK",
  "PF_RANK",
  "PFD_RANK",
  "PTS_RANK",
  "PLUS_MINUS_RANK",
  "NBA_FANTASY_PTS_RANK",
  "DD2_RANK",
  "TD3_RANK",
  "WNBA_FANTASY_PTS_RANK",
  "VIDEO_AVAILABLE_FLAG"
}

 */