import { describe, expect, it } from 'vitest';
import type { PlayerStats } from '~/models/PlayerStats';
import { getJSON, successOrThrow } from '~/reqs';
import { NBAStatsRequestInit } from '~/stores/scoreboard.server';

const PLAYER_STATS_BASE_URL = 'https://stats.nba.com/stats/playerdashboardbyyearoveryearcombined';

function buildPlayerStatsUrl(playerId: string): string {
	return `${PLAYER_STATS_BASE_URL}?DateFrom=&DateTo=&GameSegment=&LastNGames=0&LeagueID=00&Location=&MeasureType=Base&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerID=${playerId}&PlusMinus=N&Rank=N&Season=2023-24&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&VsConference=&VsDivision=`;
}

// stats.nba.com blocks datacenter IPs used by CI runners
describe.skipIf(!!process.env.CI)('fetchPlayerStats', () => {
	// LeBron James player ID
	const LEBRON_ID = '2544';
	// Stephen Curry player ID
	const CURRY_ID = '201939';

	it('returns valid player stats for LeBron James', async () => {
		const result = await getJSON(buildPlayerStatsUrl(LEBRON_ID), NBAStatsRequestInit);

		expect(result.success).toBe(true);
		const stats = successOrThrow<PlayerStats>(result);

		expect(stats).toBeDefined();
		expect(stats.resource).toBeTypeOf('string');
		expect(stats.parameters).toBeDefined();
		expect(stats.parameters.PlayerID).toBe(Number(LEBRON_ID));
		expect(Array.isArray(stats.resultSets)).toBe(true);
		expect(stats.resultSets.length).toBeGreaterThan(0);
	});

	it('returns result sets with headers and rowSet', async () => {
		const result = await getJSON(buildPlayerStatsUrl(LEBRON_ID), NBAStatsRequestInit);

		const stats = successOrThrow<PlayerStats>(result);

		for (const resultSet of stats.resultSets) {
			expect(resultSet.name).toBeTypeOf('string');
			expect(resultSet.name.length).toBeGreaterThan(0);
			expect(Array.isArray(resultSet.headers)).toBe(true);
			expect(resultSet.headers.length).toBeGreaterThan(0);
			expect(Array.isArray(resultSet.rowSet)).toBe(true);

			for (const row of resultSet.rowSet) {
				expect(row.length).toBe(resultSet.headers.length);
			}
		}
	});

	it('returns stats for a different player (Curry)', async () => {
		const result = await getJSON(buildPlayerStatsUrl(CURRY_ID), NBAStatsRequestInit);

		const stats = successOrThrow<PlayerStats>(result);

		expect(stats).toBeDefined();
		expect(stats.parameters.PlayerID).toBe(Number(CURRY_ID));
		expect(stats.resultSets.length).toBeGreaterThan(0);
	});

	it('includes ByYearBasePlayerDashboard result set', async () => {
		const result = await getJSON(buildPlayerStatsUrl(LEBRON_ID), NBAStatsRequestInit);

		const stats = successOrThrow<PlayerStats>(result);

		const byYear = stats.resultSets.find((rs) => rs.name === 'ByYearBasePlayerDashboard');
		expect(byYear).toBeDefined();
		if (byYear) {
			expect(byYear.headers).toContain('GROUP_VALUE');
			expect(byYear.headers).toContain('GP');
			expect(byYear.headers).toContain('PTS');
			expect(byYear.headers).toContain('AST');
			expect(byYear.headers).toContain('REB');
			expect(byYear.rowSet.length).toBeGreaterThan(0);
		}
	});
});
