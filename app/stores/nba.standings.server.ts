import dayjs from 'dayjs';
import type { PlayerStats, Standings } from '../models/PlayerStats';
import { getJSONWithTimeout, successOrThrow } from '../reqs';
import { fetchStandingsESPN } from './espn.server';
import { NBAStatsRequestInit } from './scoreboard.server';
import { fetchFromCache, saveToCache } from './simpleCache.server';

export async function fetchStandings(): Promise<Standings> {
	const cacheKey = `standings2:${dayjs().format('YYYY-MM-DD')}`;
	const cacheResult = await fetchFromCache(cacheKey);
	if (cacheResult != null) {
		return cacheResult as unknown as PlayerStats;
	}

	// Try ESPN first, fall back to stats.nba.com
	try {
		const result = await fetchStandingsESPN();
		await saveToCache(cacheKey, result);
		return result;
	} catch (err) {
		console.warn('ESPN standings failed, falling back to stats.nba.com:', err);
	}

	const url =
		'https://stats.nba.com/stats/leaguestandingsv3?LeagueID=00&Season=2024-25&SeasonType=Regular%20Season';

	const result = successOrThrow<PlayerStats>(
		await getJSONWithTimeout(url, 5000, NBAStatsRequestInit),
	);

	await saveToCache(cacheKey, result);
	return result;
}
