import { describe, expect, it } from 'vitest';
import type { BoxScoreGame } from '~/models/boxScore';
import type { Game, TodaysScoreboard } from '~/models/todaysScoreboard';
import { getJSON, successOrThrow } from '~/reqs';
import { NBAStatsRequestInit } from '~/stores/scoreboard.server';

function expectValidTeam(team: {
	teamId: number;
	teamName: string;
	teamCity: string;
	teamTricode: string;
	score: number;
}) {
	expect(team.teamId).toBeTypeOf('number');
	expect(team.teamName).toBeTypeOf('string');
	expect(team.teamName.length).toBeGreaterThan(0);
	expect(team.teamCity).toBeTypeOf('string');
	expect(team.teamTricode).toBeTypeOf('string');
	expect(team.teamTricode.length).toBe(3);
	expect(team.score).toBeTypeOf('number');
}

function expectValidGame(game: Game) {
	expect(game.gameId).toBeTypeOf('string');
	expect(game.gameId.length).toBeGreaterThan(0);
	expect(game.gameCode).toBeTypeOf('string');
	expect(game.gameStatus).toBeTypeOf('number');
	expect([1, 2, 3]).toContain(game.gameStatus);
	expect(game.gameStatusText).toBeTypeOf('string');
	expect(game.period).toBeTypeOf('number');
	expect(game.gameTimeUTC).toBeTypeOf('string');
	expectValidTeam(game.homeTeam);
	expectValidTeam(game.awayTeam);
	expect(game.gameLeaders).toBeDefined();
	expect(game.gameLeaders.homeLeaders).toBeDefined();
	expect(game.gameLeaders.awayLeaders).toBeDefined();
}

describe('fetchTodaysScoreboard', () => {
	it('returns a valid scoreboard from the S3 endpoint', async () => {
		const result = await getJSON(
			'https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json',
		);

		expect(result.success).toBe(true);
		const data = successOrThrow<TodaysScoreboard>(result);
		const scoreboard = data.scoreboard;

		expect(scoreboard).toBeDefined();
		expect(scoreboard.gameDate).toBeTypeOf('string');
		expect(scoreboard.gameDate).toMatch(/^\d{4}-\d{2}-\d{2}$/);
		expect(scoreboard.leagueId).toBeTypeOf('string');
		expect(scoreboard.leagueName).toBeTypeOf('string');
		expect(Array.isArray(scoreboard.games)).toBe(true);
	});

	it('returns games with valid structure when games exist', async () => {
		const result = await getJSON(
			'https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json',
		);

		const data = successOrThrow<TodaysScoreboard>(result);
		for (const game of data.scoreboard.games) {
			expectValidGame(game);
		}
	});
});

// stats.nba.com blocks datacenter IPs used by CI runners
describe.skipIf(!!process.env.CI)('fetchDaysGames (scoreboardv3 API)', () => {
	it('returns games for a known date with NBA games', async () => {
		const result = await getJSON(
			'https://stats.nba.com/stats/scoreboardv3?GameDate=2025-01-15&LeagueID=00',
			NBAStatsRequestInit,
		);

		expect(result.success).toBe(true);
		const data = successOrThrow<TodaysScoreboard>(result);
		expect(Array.isArray(data.scoreboard.games)).toBe(true);
		expect(data.scoreboard.games.length).toBeGreaterThan(0);

		for (const game of data.scoreboard.games) {
			expectValidGame(game);
		}
	});

	it('returns empty games array for a date with no scheduled games', async () => {
		const result = await getJSON(
			'https://stats.nba.com/stats/scoreboardv3?GameDate=2025-07-04&LeagueID=00',
			NBAStatsRequestInit,
		);

		// The API should return success with an empty games array for off-season dates
		if (result.success) {
			const data = successOrThrow<TodaysScoreboard>(result);
			expect(Array.isArray(data.scoreboard.games)).toBe(true);
			expect(data.scoreboard.games.length).toBe(0);
		} else {
			// Some dates may return an error from the API
			expect(result.response.status).toBeGreaterThanOrEqual(400);
		}
	});
});

describe('fetchGame (box score API)', () => {
	it('returns a valid box score for a known completed game', async () => {
		// Known completed game from Jan 15, 2025 (LAL vs MIA: 0022400573)
		const boxResult = await getJSON(
			'https://cdn.nba.com/static/json/liveData/boxscore/boxscore_0022400573.json',
		);

		expect(boxResult.success).toBe(true);
		const boxData = successOrThrow<{ game: BoxScoreGame }>(boxResult);
		const game = boxData.game;

		expectValidBoxScore(game);
	});

	it('returns failure for a non-existent game ID', async () => {
		const result = await getJSON(
			'https://cdn.nba.com/static/json/liveData/boxscore/boxscore_0000000000.json',
		);

		expect(result.success).toBe(false);
		expect(result.response.status).toBeGreaterThanOrEqual(400);
	});
});

function expectValidBoxScore(game: BoxScoreGame) {
	expect(game.gameId).toBeTypeOf('string');
	expect(game.gameStatus).toBeTypeOf('number');
	expect([1, 2, 3]).toContain(game.gameStatus);
	expect(game.gameStatusText).toBeTypeOf('string');
	expect(game.period).toBeTypeOf('number');

	expect(game.homeTeam).toBeDefined();
	expect(game.homeTeam.teamId).toBeTypeOf('number');
	expect(game.homeTeam.teamName).toBeTypeOf('string');
	expect(game.homeTeam.teamTricode).toBeTypeOf('string');
	expect(Array.isArray(game.homeTeam.players)).toBe(true);
	expect(game.homeTeam.statistics).toBeDefined();

	expect(game.awayTeam).toBeDefined();
	expect(game.awayTeam.teamId).toBeTypeOf('number');
	expect(game.awayTeam.teamName).toBeTypeOf('string');
	expect(game.awayTeam.teamTricode).toBeTypeOf('string');
	expect(Array.isArray(game.awayTeam.players)).toBe(true);
	expect(game.awayTeam.statistics).toBeDefined();

	for (const player of game.homeTeam.players) {
		expect(player.personId).toBeTypeOf('number');
		expect(player.name).toBeTypeOf('string');
		expect(player.statistics).toBeDefined();
		expect(player.statistics.points).toBeTypeOf('number');
		expect(player.statistics.assists).toBeTypeOf('number');
	}

	const stats = game.homeTeam.statistics;
	expect(stats.points).toBeTypeOf('number');
	expect(stats.assists).toBeTypeOf('number');
	expect(stats.fieldGoalsAttempted).toBeTypeOf('number');
	expect(stats.fieldGoalsMade).toBeTypeOf('number');
	expect(stats.fieldGoalsPercentage).toBeTypeOf('number');
	expect(stats.threePointersAttempted).toBeTypeOf('number');
	expect(stats.threePointersMade).toBeTypeOf('number');
	expect(stats.freeThrowsAttempted).toBeTypeOf('number');
	expect(stats.freeThrowsMade).toBeTypeOf('number');

	expect(game.arena).toBeDefined();
	expect(game.arena.arenaName).toBeTypeOf('string');
}
