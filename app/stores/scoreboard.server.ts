import type { BoxScore } from '../models/boxScore';
import type { TodaysScoreboard } from '../models/todaysScoreboard';

import { getJSON, successOrThrow, successOrUndefined } from '../reqs';
import { fetchFromCache, saveToCache } from './simpleCache.server';

let todayData: { data: TodaysScoreboard; fetchedAt: number } | undefined =
	undefined;

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
	// todo: validate day

	const cacheKey = `day:${day}`;
	const cacheResult = await fetchFromCache(cacheKey);
	if (cacheResult != null) {
		// todo: validate response?
		return (cacheResult as unknown as TodaysScoreboard).scoreboard.games;
	}

	const result = successOrThrow<TodaysScoreboard>(
		await getJSON(
			`https://stats.nba.com/stats/scoreboardv3?GameDate=${day}&LeagueID=00`,
			NBAStatsRequestInit,
		),
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

	const result = successOrUndefined<BoxScore>(
		await getJSON(
			`https://cdn.nba.com/static/json/liveData/boxscore/boxscore_${id}.json`,
		),
	);
	if (!result) {
		return undefined;
	}

	// for now only cache completed games
	if (result.game.gameStatus === 3) {
		await saveToCache(cacheKey, result);
	}
	return result.game;
};
