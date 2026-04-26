import { existsSync, unlinkSync } from 'node:fs';
import path from 'node:path';
import { afterAll, describe, expect, it } from 'vitest';
import { fetchFromCache, saveToCache } from '~/stores/simpleCache.server';

const TEST_PREFIX = '__test_cache_';

function cleanupTestFile(key: string) {
	const filepath = path.join('data', `${key}.json`);
	if (existsSync(filepath)) {
		unlinkSync(filepath);
	}
}

describe('simpleCache', () => {
	const testKeys: string[] = [];

	function testKey(name: string) {
		const key = `${TEST_PREFIX}${name}`;
		testKeys.push(key);
		return key;
	}

	afterAll(() => {
		for (const key of testKeys) {
			cleanupTestFile(key);
		}
	});

	it('returns undefined for a cache miss', async () => {
		const result = await fetchFromCache(testKey('nonexistent'));
		expect(result).toBeUndefined();
	});

	it('saves and retrieves data from cache', async () => {
		const key = testKey('scoreboard');
		const testData = {
			scoreboard: {
				gameDate: '2025-01-15',
				games: [{ gameId: '001', teamName: 'Lakers' }],
			},
		};

		await saveToCache(key, testData);
		const result = await fetchFromCache(key);

		expect(result).toEqual(testData);
	});

	it('overwrites existing cache entries', async () => {
		const key = testKey('overwrite');
		const original = { value: 'original' };
		const updated = { value: 'updated' };

		await saveToCache(key, original);
		await saveToCache(key, updated);
		const result = await fetchFromCache(key);

		expect(result).toEqual(updated);
	});

	it('handles complex nested data', async () => {
		const key = testKey('complex');
		const complexData = {
			resource: 'leaguestandingsv3',
			resultSets: [
				{
					name: 'Standings',
					headers: ['TeamID', 'TeamName', 'WINS', 'LOSSES'],
					rowSet: [
						[1610612747, 'Lakers', 25, 15],
						[1610612744, 'Warriors', 22, 18],
					],
				},
			],
		};

		await saveToCache(key, complexData);
		const result = await fetchFromCache(key);

		expect(result).toEqual(complexData);
	});
});
