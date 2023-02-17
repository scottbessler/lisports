import { getJSON } from "@xavdid/json-requests";
// import memoize from "lodash.memoize";
import type { BoxScore } from "../models/boxScore";
import type { TodaysScoreboard } from "../models/todaysScoreboard";
import type * as s from "zapatos/schema";
import * as db from "zapatos/db";
import { pgPool } from "./pgPool.server";
import invariant from "tiny-invariant";
import { getTodayYMD } from "../utils";

export const fetchTodaysGames = async () => {
  const result = await getJSON<TodaysScoreboard>(
    "https://cdn.nba.com/static/json/liveData/scoreboard/todaysScoreboard_00.json"
  );
  return result.scoreboard.games;
};

const fetchFromCache = async (key: string) => {
  const result = await db.sql<
    s.json_cache.SQL,
    s.json_cache.Selectable[]
  >`select * from ${"json_cache"} where ${{ key }}`.run(pgPool);
  invariant(
    result.length <= 1,
    "received multiple results for key, should be impossible"
  );
  return result[0];
};

const saveToCache = async (key: string, value: any) => {
  await db
    .upsert("json_cache", { key, value, stored_at: new Date() }, ["key"])
    .run(pgPool);
};

export const fetchDaysGames = async (day: string) => {
  // todo: validate day

  const cacheKey = `day:${day}`;
  const cacheResult = await fetchFromCache(cacheKey);
  if (cacheResult != null) {
    // todo: check stored_at?
    // todo: validate response?
    return (cacheResult.value as unknown as TodaysScoreboard).scoreboard.games;
  }
  const result = await getJSON<TodaysScoreboard>(
    `https://stats.nba.com/stats/scoreboardv3?GameDate=${day}&LeagueID=00`,
    {
      headers: {
        "User-Agent":
          "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/110.0",
        Accept: "*/*",
        "Accept-Language": "en-US,en;q=0.5",
        "Sec-Fetch-Dest": "empty",
        "Sec-Fetch-Mode": "cors",
        "Sec-Fetch-Site": "same-site",
        Pragma: "no-cache",
        "Cache-Control": "no-cache",
        Referer: "https://www.nba.com/",
        Origin: "https://www.nba.com",
      },
    }
  );

  if (day < getTodayYMD()) {
    await saveToCache(cacheKey, result);
  }
  return result.scoreboard.games;
};

export const fetchGame = async (id: string) => {
  const cacheKey = `game:${id}`;
  const cacheResult = await fetchFromCache(cacheKey);
  if (cacheResult != null) {
    return (cacheResult.value as unknown as BoxScore).game;
  }
  const result = await getJSON<BoxScore>(
    `https://cdn.nba.com/static/json/liveData/boxscore/boxscore_${id}.json`
  );
  // for now only cache completed games
  // console.log(typeof result.game.gameStatus);
  if (result.game.gameStatus === 3) {
    await saveToCache(cacheKey, result);
  }
  return result.game;
};
