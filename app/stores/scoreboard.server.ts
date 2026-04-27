import type { BoxScore } from '../models/boxScore';
import type { TodaysScoreboard } from '../models/todaysScoreboard';

import { getJSON, getJSONWithTimeout, successOrThrow, successOrUndefined } from '../reqs';
import { fetchDaysGamesESPN, fetchGameESPN } from './espn.server';
import { fetchFromCache, saveToCache } from './simpleCache.server';

let todayData: { data: TodaysScoreboard; fetchedAt: number } | undefined = undefined;

export const fetchTodaysScoreboard = async () => {
	if (todayData && Date.now() - todayData.fetchedAt < 30000) {
		return todayData.data.scoreboard;
	}
	const result = successOrThrow<TodaysScoreboard>(
		await getJSON(
			'https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json',
		),
	);
	todayData = { data: result, fetchedAt: Date.now() };
	return todayData.data.scoreboard;
};

export const NBAStatsRequestInit = {
	headers: {
		'User-Agent':
			'Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/110.0',
		'Accept-Language': 'en-US,en;q=0.5',
		Referer: 'https://www.nba.com/',
		Origin: 'https://www.nba.com',
	},
};

// TODO: https://cdn.nba.com/static/json/liveData/odds/odds_todaysGames.json
// TODO?: https://cdn.nba.com/static/json/liveData/channels/v2/channels_00.json
// https://cdn.nba.com/logos/nba/1610612748/primary/L/logo.svg

export const fetchDaysGames = async (day: string) => {
	const cacheKey = `day:${day}`;
	const cacheResult = await fetchFromCache(cacheKey);
	if (cacheResult != null) {
		return (cacheResult as unknown as TodaysScoreboard).scoreboard.games;
	}

	// Try ESPN first, fall back to stats.nba.com
	try {
		const games = await fetchDaysGamesESPN(day);

		if (games.every((g) => g.gameStatus === 3)) {
			const wrapped: TodaysScoreboard = {
				meta: {
					version: 1,
					request: 'espn',
					time: new Date().toISOString(),
					code: 200,
				},
				scoreboard: {
					gameDate: day,
					leagueId: '00',
					leagueName: 'National Basketball Association',
					games,
				},
			};
			await saveToCache(cacheKey, wrapped);
		}
		return games;
	} catch (err) {
		console.warn('ESPN scoreboard failed, falling back to stats.nba.com:', err);
	}

	const result = successOrThrow<TodaysScoreboard>(
		await getJSONWithTimeout(
			`https://stats.nba.com/stats/scoreboardv3?GameDate=${day}&LeagueID=00`,
			5000,
			NBAStatsRequestInit,
		),
	);

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

	// Try ESPN first (works from datacenter IPs, consistent with ESPN scoreboard IDs)
	try {
		const espnResult = await fetchGameESPN(id);
		if (espnResult) {
			if (espnResult.gameStatus === 3) {
				await saveToCache(cacheKey, { game: espnResult });
			}
			return espnResult;
		}
	} catch (err) {
		console.warn('ESPN box score failed, trying NBA CDN:', err);
	}

	// Fall back to NBA CDN with a timeout to avoid hanging forever
	const result = successOrUndefined<BoxScore>(
		await getJSONWithTimeout(
			`https://cdn.nba.com/static/json/liveData/boxscore/boxscore_${id}.json?x=${Math.random()}`,
			8000,
		),
	);
	if (!result) {
		return undefined;
	}

	if (result.game.gameStatus === 3) {
		await saveToCache(cacheKey, result);
	}
	return result.game;
};
