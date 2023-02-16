import { getJSON } from "@xavdid/json-requests";
// import memoize from "lodash.memoize";
import type { BoxScore } from "../models/boxScore";
import type { TodaysScoreboard } from "../models/todaysScoreboard";

export const fetchTodaysGames = async () => {
  const result = await getJSON<TodaysScoreboard>(
    "https://cdn.nba.com/static/json/liveData/scoreboard/todaysScoreboard_00.json"
  );
  return result.scoreboard.games;
};

const dayCache: Record<string, TodaysScoreboard> = Object.create(null);

export const fetchDaysGames = async (day: string) => {
  if (day in dayCache) {
    return dayCache[day].scoreboard.games;
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
  dayCache[day] = result;
  return result.scoreboard.games;
};

const gameCache: Record<string, BoxScore> = Object.create(null);

export const fetchGame = async (id: string) => {
  if (id in gameCache) {
    return gameCache[id].game;
  }
  const result = await getJSON<BoxScore>(
    `https://cdn.nba.com/static/json/liveData/boxscore/boxscore_${id}.json`
  );
  gameCache[id] = result;
  return result.game;
};
