import { getJSON } from "@xavdid/json-requests";
import type { BoxScore } from "../models/boxScore";
import type { TodaysScoreboard } from "../models/todaysScoreboard";

import { existsSync } from "node:fs";
import { readFile, writeFile } from "node:fs/promises";

import path from "node:path";

let todayData: { data: TodaysScoreboard; fetchedAt: number } | undefined =
  undefined;

export const fetchTodaysScoreboard = async () => {
  if (todayData && Date.now() - todayData.fetchedAt < 30000) {
    return todayData.data.scoreboard;
  }
  const result = await getJSON<TodaysScoreboard>(
    "https://cdn.nba.com/static/json/liveData/scoreboard/todaysScoreboard_00.json"
  );
  todayData = { data: result, fetchedAt: Date.now() };
  return result.scoreboard;
};

const dataPath = process.env.DATA_PATH || "data";
function toFileName(key: string) {
  return path.join(dataPath, `${key}.json`);
}

const fetchFromCache = async (key: string) => {
  try {
    const filename = toFileName(key);
    if (!existsSync(filename)) {
      return undefined;
    }
    const result = await readFile(filename, { encoding: "utf-8" });
    return JSON.parse(result);
  } catch (err) {
    console.error(err);
    return undefined;
  }
};

const saveToCache = async (key: string, value: any) => {
  console.log(`storing cache to ${toFileName(key)}`);
  await writeFile(toFileName(key), JSON.stringify(value), {
    encoding: "utf-8",
  });
};

export const fetchDaysGames = async (day: string) => {
  // todo: validate day

  const cacheKey = `day:${day}`;
  const cacheResult = await fetchFromCache(cacheKey);
  if (cacheResult != null) {
    // todo: validate response?
    return (cacheResult as unknown as TodaysScoreboard).scoreboard.games;
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

  // cache if all games are completed
  if (result.scoreboard.games.every((g) => g.gameStatus === 3)) {
    await saveToCache(cacheKey, result);
  }
  return result.scoreboard.games;
};

export const fetchGame = async (id: string) => {
  const cacheKey = `game:${id}`;
  const cacheResult = await fetchFromCache(cacheKey);
  if (cacheResult != null) {
    return (cacheResult as unknown as BoxScore).game;
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
