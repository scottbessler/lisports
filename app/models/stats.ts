import type { Statistics, TeamStatistics } from './boxScore';

export const trueShooting = ({
	points,
	fieldGoalsAttempted,
	freeThrowsAttempted,
}: {
	points: number;
	fieldGoalsAttempted: number;
	freeThrowsAttempted: number;
}) => {
	if (fieldGoalsAttempted + freeThrowsAttempted <= 0) {
		return undefined;
	}
	return (0.5 * points) / (fieldGoalsAttempted + 0.475 * freeThrowsAttempted);
};

export const usageRate = (
	{
		fieldGoalsAttempted,
		freeThrowsAttempted,
		turnovers,
		minutes,
	}: {
		fieldGoalsAttempted: number;
		freeThrowsAttempted: number;
		turnovers: number;
		minutes: number;
	},
	teamStats: {
		fieldGoalsAttempted: number;
		freeThrowsAttempted: number;
		turnovers: number;
		minutes: number;
	},
) => {
	return Math.round(
		(100 *
			((fieldGoalsAttempted + 0.44 * freeThrowsAttempted + turnovers) *
				(teamStats.minutes / 5))) /
			(minutes *
				(teamStats.fieldGoalsAttempted +
					0.44 * teamStats.freeThrowsAttempted +
					teamStats.turnovers)),
	);
};

export const pointsPerShot = ({
	points,
	fieldGoalsAttempted,
}: {
	points: number;
	fieldGoalsAttempted: number;
}) => {
	if (fieldGoalsAttempted <= 0) {
		return undefined;
	}
	return Math.round((points * 100) / fieldGoalsAttempted) / 100;
};

export const pie = (
	{
		points,
		fieldGoalsMade,
		freeThrowsMade,
		fieldGoalsAttempted,
		freeThrowsAttempted,
		reboundsDefensive,
		reboundsOffensive,
		assists,
		steals,
		blocks,
		blocksReceived,
		foulsPersonal,
		turnovers,
	}: Statistics,
	teamStats: TeamStatistics,
	otherTeamStats: TeamStatistics,
) => {
	return Math.round(
		100 *
			((points +
				fieldGoalsMade +
				freeThrowsMade -
				fieldGoalsAttempted -
				freeThrowsAttempted +
				reboundsDefensive +
				reboundsOffensive / 2 +
				assists / 2 /* avoid double counting */ +
				steals +
				blocks / 2 -
				blocksReceived / 2 -
				foulsPersonal -
				turnovers) /
				(teamStats.points +
					otherTeamStats.points +
					(teamStats.fieldGoalsMade + otherTeamStats.fieldGoalsMade) +
					(teamStats.freeThrowsMade + otherTeamStats.freeThrowsMade) -
					(teamStats.fieldGoalsAttempted + otherTeamStats.fieldGoalsAttempted) -
					(teamStats.freeThrowsAttempted + otherTeamStats.freeThrowsAttempted) +
					(teamStats.reboundsDefensive + otherTeamStats.reboundsDefensive) +
					(teamStats.reboundsOffensive + otherTeamStats.reboundsOffensive) / 2 +
					(teamStats.assists + otherTeamStats.assists) /
						2 /* avoid double counting */ +
					(teamStats.steals + otherTeamStats.steals) +
					(teamStats.blocks + otherTeamStats.blocks) / 2 -
					(teamStats.blocksReceived + otherTeamStats.blocksReceived) / 2 -
					(teamStats.foulsPersonal + otherTeamStats.foulsPersonal) -
					(teamStats.turnovers + otherTeamStats.turnovers))),
	);
};
