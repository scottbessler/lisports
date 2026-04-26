import { describe, expect, it } from 'vitest';
import { getJSON, successOrThrow, successOrUndefined } from '~/reqs';

describe('getJSON', () => {
	it('returns success with parsed JSON for a valid URL', async () => {
		const result = await getJSON(
			'https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json',
		);

		expect(result.success).toBe(true);
		expect(result.response.status).toBe(200);
		expect(result.data).toBeDefined();
		if (result.success) {
			expect(result.data).toHaveProperty('scoreboard');
		}
	});

	it('returns failure for a non-existent URL', async () => {
		const result = await getJSON(
			'https://cdn.nba.com/static/json/liveData/boxscore/boxscore_NONEXISTENT.json',
		);

		expect(result.success).toBe(false);
		expect(result.response.status).toBeGreaterThanOrEqual(400);
	});

	it.skipIf(!!process.env.CI)('forwards custom headers', async () => {
		const result = await getJSON(
			'https://stats.nba.com/stats/leaguestandingsv3?LeagueID=00&Season=2024-25&SeasonType=Regular%20Season',
			{
				headers: {
					'User-Agent':
						'Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/110.0',
					Referer: 'https://www.nba.com/',
					Origin: 'https://www.nba.com',
				},
			},
		);

		expect(result.success).toBe(true);
		expect(result.response.status).toBe(200);
	});
});

describe('successOrThrow', () => {
	it('returns data when result is successful', async () => {
		const result = await getJSON(
			'https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json',
		);

		const data = successOrThrow<{ scoreboard: unknown }>(result);
		expect(data).toHaveProperty('scoreboard');
	});

	it('throws when result is a failure', async () => {
		const result = await getJSON(
			'https://cdn.nba.com/static/json/liveData/boxscore/boxscore_NONEXISTENT.json',
		);

		expect(() => successOrThrow(result)).toThrow('request failed');
	});
});

describe('successOrUndefined', () => {
	it('returns data when result is successful', async () => {
		const result = await getJSON(
			'https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json',
		);

		const data = successOrUndefined<{ scoreboard: unknown }>(result);
		expect(data).toBeDefined();
		expect(data).toHaveProperty('scoreboard');
	});

	it('returns undefined when result is a failure', async () => {
		const result = await getJSON(
			'https://cdn.nba.com/static/json/liveData/boxscore/boxscore_NONEXISTENT.json',
		);

		const data = successOrUndefined(result);
		expect(data).toBeUndefined();
	});
});
