import { describe, expect, it } from 'vitest';
import type { Standings } from '~/models/PlayerStats';
import { getJSON, successOrThrow } from '~/reqs';
import { NBAStatsRequestInit } from '~/stores/scoreboard.server';

const STANDINGS_URL =
	'https://stats.nba.com/stats/leaguestandingsv3?LeagueID=00&Season=2024-25&SeasonType=Regular%20Season';

describe('fetchStandings', () => {
	it('returns valid standings data', async () => {
		const result = await getJSON(STANDINGS_URL, NBAStatsRequestInit);

		expect(result.success).toBe(true);
		const standings = successOrThrow<Standings>(result);

		expect(standings).toBeDefined();
		expect(standings.resource).toBeTypeOf('string');
		expect(Array.isArray(standings.resultSets)).toBe(true);
		expect(standings.resultSets.length).toBeGreaterThan(0);
	});

	it('has a Standings result set with expected headers', async () => {
		const result = await getJSON(STANDINGS_URL, NBAStatsRequestInit);
		const standings = successOrThrow<Standings>(result);

		const standingsSet = standings.resultSets[0];
		expect(standingsSet).toBeDefined();
		expect(standingsSet.name).toBeTypeOf('string');
		expect(Array.isArray(standingsSet.headers)).toBe(true);

		expect(standingsSet.headers).toContain('TeamID');
		expect(standingsSet.headers).toContain('TeamCity');
		expect(standingsSet.headers).toContain('TeamName');
		expect(standingsSet.headers).toContain('Conference');
		expect(standingsSet.headers).toContain('WINS');
		expect(standingsSet.headers).toContain('LOSSES');
		expect(standingsSet.headers).toContain('WinPCT');
		expect(standingsSet.headers).toContain('PlayoffRank');
	});

	it('returns 30 NBA teams', async () => {
		const result = await getJSON(STANDINGS_URL, NBAStatsRequestInit);
		const standings = successOrThrow<Standings>(result);

		const standingsSet = standings.resultSets[0];
		expect(standingsSet.rowSet.length).toBe(30);
	});

	it('has rows matching header length', async () => {
		const result = await getJSON(STANDINGS_URL, NBAStatsRequestInit);
		const standings = successOrThrow<Standings>(result);

		const standingsSet = standings.resultSets[0];
		for (const row of standingsSet.rowSet) {
			expect(row.length).toBe(standingsSet.headers.length);
		}
	});

	it('contains both Eastern and Western conference teams', async () => {
		const result = await getJSON(STANDINGS_URL, NBAStatsRequestInit);
		const standings = successOrThrow<Standings>(result);

		const standingsSet = standings.resultSets[0];
		const confIndex = standingsSet.headers.indexOf('Conference');
		expect(confIndex).toBeGreaterThanOrEqual(0);

		const conferences = new Set(
			standingsSet.rowSet.map((row) => row[confIndex]),
		);
		expect(conferences.has('East')).toBe(true);
		expect(conferences.has('West')).toBe(true);
	});
});
