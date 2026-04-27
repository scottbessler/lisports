import type { PlayerStats } from '../models/PlayerStats';
import {
	type ESPNPlayerInfo,
	type PlayerGameLog,
	fetchPlayerGameLogESPN,
	fetchPlayerInfoESPN,
	fetchPlayerStatsESPN,
} from './espn.server';
import { fetchFromCache, saveToCache } from './simpleCache.server';

export async function fetchPlayerStats(playerId: string): Promise<PlayerStats> {
	const cacheKey = `player:${playerId}`;
	const cacheResult = await fetchFromCache(cacheKey);
	if (cacheResult != null) {
		return cacheResult as unknown as PlayerStats;
	}

	const result = await fetchPlayerStatsESPN(playerId);
	if (!result) {
		throw new Error(`Player stats not found for ID ${playerId}`);
	}

	await saveToCache(cacheKey, result);
	return result;
}

export async function fetchPlayerInfo(playerId: string): Promise<ESPNPlayerInfo | undefined> {
	const cacheKey = `playerinfo:${playerId}`;
	const cacheResult = await fetchFromCache(cacheKey);
	if (cacheResult != null) {
		return cacheResult as unknown as ESPNPlayerInfo;
	}

	const info = await fetchPlayerInfoESPN(playerId);
	if (info) {
		await saveToCache(cacheKey, info);
	}
	return info;
}

export async function fetchPlayerGameLog(playerId: string): Promise<PlayerGameLog | undefined> {
	const cacheKey = `gamelog:${playerId}`;
	const cacheResult = await fetchFromCache(cacheKey);
	if (cacheResult != null) {
		return cacheResult as unknown as PlayerGameLog;
	}

	const log = await fetchPlayerGameLogESPN(playerId);
	if (log) {
		await saveToCache(cacheKey, log);
	}
	return log;
}
