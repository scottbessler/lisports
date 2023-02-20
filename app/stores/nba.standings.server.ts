import dayjs from "dayjs";
import type { PlayerStats, Standings } from "../models/PlayerStats";
import { getJSON, successOrThrow } from "../reqs";
import { NBAStatsRequestInit } from "./scoreboard.server";
import { fetchFromCache, saveToCache } from "./simpleCache.server";

export async function fetchStandings(): Promise<Standings> {
  const url = `https://stats.nba.com/stats/leaguestandingsv3?LeagueID=00&Season=2022-23&SeasonType=Regular%20Season`;

  const cacheKey = `standings:${dayjs().format("YYYY-MM-DD")}`;
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
