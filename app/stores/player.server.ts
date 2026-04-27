import type { PlayerStats } from '../models/PlayerStats';
import { getJSONWithTimeout, successOrThrow } from '../reqs';
import { NBAStatsRequestInit } from './scoreboard.server';
import { fetchFromCache, saveToCache } from './simpleCache.server';

export async function fetchPlayerStats(playerId: string): Promise<PlayerStats> {
	const url = `https://stats.nba.com/stats/playerdashboardbyyearoveryearcombined?DateFrom=&DateTo=&GameSegment=&LastNGames=0&LeagueID=00&Location=&MeasureType=Base&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerID=${playerId}&PlusMinus=N&Rank=N&Season=2023-24&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&VsConference=&VsDivision=`;

	const cacheKey = `player:${playerId}`;
	const cacheResult = await fetchFromCache(cacheKey);
	if (cacheResult != null) {
		return cacheResult as unknown as PlayerStats;
	}

	const result = successOrThrow<PlayerStats>(
		await getJSONWithTimeout(url, 8000, NBAStatsRequestInit),
	);

	await saveToCache(cacheKey, result);
	return result;
}
