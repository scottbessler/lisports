import type { Standings } from '../models/PlayerStats';
import type {
	BoxScoreGame,
	BoxScoreTeam,
	Player,
	Statistics,
	TeamStatistics,
} from '../models/boxScore';
import type { StandingsTeam } from '../models/standings';
import type { Game, Leaders, Period, Team } from '../models/todaysScoreboard';

// ESPN abbreviation → { nbaTricode, nbaTeamId, city, name }
const ESPN_TO_NBA_TEAM: Record<
	string,
	{ tricode: string; teamId: number; city: string; name: string }
> = {
	ATL: { tricode: 'ATL', teamId: 1610612737, city: 'Atlanta', name: 'Hawks' },
	BOS: { tricode: 'BOS', teamId: 1610612738, city: 'Boston', name: 'Celtics' },
	BKN: {
		tricode: 'BKN',
		teamId: 1610612751,
		city: 'Brooklyn',
		name: 'Nets',
	},
	CHA: {
		tricode: 'CHA',
		teamId: 1610612766,
		city: 'Charlotte',
		name: 'Hornets',
	},
	CHI: {
		tricode: 'CHI',
		teamId: 1610612741,
		city: 'Chicago',
		name: 'Bulls',
	},
	CLE: {
		tricode: 'CLE',
		teamId: 1610612739,
		city: 'Cleveland',
		name: 'Cavaliers',
	},
	DAL: {
		tricode: 'DAL',
		teamId: 1610612742,
		city: 'Dallas',
		name: 'Mavericks',
	},
	DEN: {
		tricode: 'DEN',
		teamId: 1610612743,
		city: 'Denver',
		name: 'Nuggets',
	},
	DET: {
		tricode: 'DET',
		teamId: 1610612765,
		city: 'Detroit',
		name: 'Pistons',
	},
	GS: {
		tricode: 'GSW',
		teamId: 1610612744,
		city: 'Golden State',
		name: 'Warriors',
	},
	HOU: {
		tricode: 'HOU',
		teamId: 1610612745,
		city: 'Houston',
		name: 'Rockets',
	},
	IND: {
		tricode: 'IND',
		teamId: 1610612754,
		city: 'Indiana',
		name: 'Pacers',
	},
	LAC: {
		tricode: 'LAC',
		teamId: 1610612746,
		city: 'LA',
		name: 'Clippers',
	},
	LAL: {
		tricode: 'LAL',
		teamId: 1610612747,
		city: 'Los Angeles',
		name: 'Lakers',
	},
	MEM: {
		tricode: 'MEM',
		teamId: 1610612763,
		city: 'Memphis',
		name: 'Grizzlies',
	},
	MIA: { tricode: 'MIA', teamId: 1610612748, city: 'Miami', name: 'Heat' },
	MIL: {
		tricode: 'MIL',
		teamId: 1610612749,
		city: 'Milwaukee',
		name: 'Bucks',
	},
	MIN: {
		tricode: 'MIN',
		teamId: 1610612750,
		city: 'Minnesota',
		name: 'Timberwolves',
	},
	NO: {
		tricode: 'NOP',
		teamId: 1610612740,
		city: 'New Orleans',
		name: 'Pelicans',
	},
	NY: {
		tricode: 'NYK',
		teamId: 1610612752,
		city: 'New York',
		name: 'Knicks',
	},
	OKC: {
		tricode: 'OKC',
		teamId: 1610612760,
		city: 'Oklahoma City',
		name: 'Thunder',
	},
	ORL: {
		tricode: 'ORL',
		teamId: 1610612753,
		city: 'Orlando',
		name: 'Magic',
	},
	PHI: {
		tricode: 'PHI',
		teamId: 1610612755,
		city: 'Philadelphia',
		name: '76ers',
	},
	PHX: {
		tricode: 'PHX',
		teamId: 1610612756,
		city: 'Phoenix',
		name: 'Suns',
	},
	POR: {
		tricode: 'POR',
		teamId: 1610612757,
		city: 'Portland',
		name: 'Trail Blazers',
	},
	SAC: {
		tricode: 'SAC',
		teamId: 1610612758,
		city: 'Sacramento',
		name: 'Kings',
	},
	SA: {
		tricode: 'SAS',
		teamId: 1610612759,
		city: 'San Antonio',
		name: 'Spurs',
	},
	TOR: {
		tricode: 'TOR',
		teamId: 1610612761,
		city: 'Toronto',
		name: 'Raptors',
	},
	UTAH: {
		tricode: 'UTA',
		teamId: 1610612762,
		city: 'Utah',
		name: 'Jazz',
	},
	WSH: {
		tricode: 'WAS',
		teamId: 1610612764,
		city: 'Washington',
		name: 'Wizards',
	},
};

interface ESPNScoreboardResponse {
	events: ESPNEvent[];
}

interface ESPNEvent {
	id: string;
	date: string;
	competitions: ESPNCompetition[];
	status: ESPNStatus;
}

interface ESPNCompetition {
	competitors: ESPNCompetitor[];
	status: ESPNStatus;
}

interface ESPNCompetitor {
	homeAway: 'home' | 'away';
	team: {
		id: string;
		abbreviation: string;
		displayName: string;
		shortDisplayName: string;
		name: string;
		location: string;
	};
	score: string;
	records?: { type: string; summary: string }[];
	linescores?: { value: number; period: number }[];
	leaders?: {
		name: string;
		leaders: {
			athlete: {
				id: string;
				displayName: string;
				jersey: string;
				position: { abbreviation: string };
			};
			displayValue: string;
		}[];
	}[];
}

interface ESPNStatus {
	clock: number;
	displayClock: string;
	period: number;
	type: {
		id: string;
		name: string;
		completed: boolean;
		description: string;
		detail: string;
		shortDetail: string;
	};
}

interface ESPNStandingsResponse {
	children: {
		name: string;
		abbreviation: string;
		standings: {
			entries: {
				team: {
					id: string;
					abbreviation: string;
					displayName: string;
					shortDisplayName: string;
					name: string;
					location: string;
				};
				stats: { name: string; value?: number; displayValue?: string }[];
			}[];
		};
	}[];
}

function espnStatusToGameStatus(status: ESPNStatus): number {
	if (status.type.name === 'STATUS_FINAL') return 3;
	if (status.type.name === 'STATUS_IN_PROGRESS') return 2;
	return 1;
}

function parseRecord(summary: string): { wins: number; losses: number } {
	const parts = summary.split('-');
	return {
		wins: Number.parseInt(parts[0], 10) || 0,
		losses: Number.parseInt(parts[1], 10) || 0,
	};
}

function espnCompetitorToTeam(competitor: ESPNCompetitor): Team {
	const espnAbbrev = competitor.team.abbreviation;
	const mapping = ESPN_TO_NBA_TEAM[espnAbbrev];

	const overallRecord = competitor.records?.find((r) => r.type === 'total');
	const { wins, losses } = overallRecord
		? parseRecord(overallRecord.summary)
		: { wins: 0, losses: 0 };

	const periods: Period[] = (competitor.linescores ?? []).map((ls) => ({
		period: ls.period,
		periodType: ls.period <= 4 ? 'REGULAR' : 'OVERTIME',
		score: ls.value,
	}));

	return {
		teamId: mapping?.teamId ?? 0,
		teamName: mapping?.name ?? competitor.team.name,
		teamCity: mapping?.city ?? competitor.team.location,
		teamTricode: mapping?.tricode ?? espnAbbrev,
		wins,
		losses,
		score: Number.parseInt(competitor.score, 10) || 0,
		inBonus: undefined,
		timeoutsRemaining: 0,
		periods,
	};
}

function extractLeaders(competitor: ESPNCompetitor): Leaders {
	const pointsLeader = competitor.leaders?.find((l) => l.name === 'points');
	const reboundsLeader = competitor.leaders?.find((l) => l.name === 'rebounds');
	const assistsLeader = competitor.leaders?.find((l) => l.name === 'assists');

	const athlete = pointsLeader?.leaders?.[0]?.athlete;
	const mapping = ESPN_TO_NBA_TEAM[competitor.team.abbreviation];

	return {
		personId: athlete ? Number.parseInt(athlete.id, 10) : 0,
		name: athlete?.displayName ?? '',
		jerseyNum: athlete?.jersey ?? '',
		position: athlete?.position?.abbreviation ?? '',
		teamTricode: mapping?.tricode ?? competitor.team.abbreviation,
		points: pointsLeader?.leaders?.[0]
			? Number.parseFloat(pointsLeader.leaders[0].displayValue)
			: 0,
		rebounds: reboundsLeader?.leaders?.[0]
			? Number.parseFloat(reboundsLeader.leaders[0].displayValue)
			: 0,
		assists: assistsLeader?.leaders?.[0]
			? Number.parseFloat(assistsLeader.leaders[0].displayValue)
			: 0,
	};
}

export async function fetchDaysGamesESPN(day: string): Promise<Game[]> {
	const espnDate = day.replace(/-/g, '');
	const url = `https://site.api.espn.com/apis/site/v2/sports/basketball/nba/scoreboard?dates=${espnDate}`;

	console.log('ESPN: fetching scoreboard for', day);
	const response = await fetch(url);
	if (!response.ok) {
		throw new Error(`ESPN scoreboard request failed: ${response.status}`);
	}

	const data = (await response.json()) as ESPNScoreboardResponse;

	return data.events.map((event): Game => {
		const comp = event.competitions[0];
		const status = comp.status;
		const homeComp = comp.competitors.find((c) => c.homeAway === 'home');
		const awayComp = comp.competitors.find((c) => c.homeAway === 'away');

		if (!homeComp || !awayComp) {
			throw new Error(`Missing competitor data for event ${event.id}`);
		}

		const homeTeam = espnCompetitorToTeam(homeComp);
		const awayTeam = espnCompetitorToTeam(awayComp);

		const gameStatus = espnStatusToGameStatus(status);

		return {
			gameId: event.id,
			gameCode: `${day}/${awayTeam.teamTricode}${homeTeam.teamTricode}`,
			gameStatus,
			gameStatusText: status.type.shortDetail ?? status.type.description,
			period: status.period,
			gameClock: status.displayClock,
			gameTimeUTC: event.date,
			gameEt: event.date,
			regulationPeriods: 4,
			ifNecessary: false,
			seriesGameNumber: '',
			seriesText: '',
			homeTeam,
			awayTeam,
			gameLeaders: {
				homeLeaders: extractLeaders(homeComp),
				awayLeaders: extractLeaders(awayComp),
			},
			pbOdds: { team: null, odds: 0, suspended: 0 },
		};
	});
}

// ─── ESPN Box Score (Summary) ────────────────────────────────────────

interface ESPNSummaryResponse {
	boxscore: {
		teams: {
			team: { id: string; abbreviation: string; displayName: string };
			statistics: { name: string; displayValue: string }[];
		}[];
		players: {
			team: { id: string; abbreviation: string };
			statistics: {
				keys: string[];
				athletes: {
					athlete: {
						id: string;
						displayName: string;
						jersey: string;
						position?: { abbreviation: string };
					};
					stats: string[];
					starter: boolean;
					didNotPlay: boolean;
					reason?: string;
				}[];
			}[];
		}[];
	};
	gameInfo: {
		venue: {
			id: string;
			fullName: string;
			address: { city: string; state: string; country?: string };
		};
		attendance: number;
		officials: {
			fullName: string;
			order: number;
		}[];
	};
	header: {
		id: string;
		competitions: {
			date: string;
			competitors: {
				homeAway: 'home' | 'away';
				team: { id: string; abbreviation: string };
				score: string;
				linescores?: { displayValue: string }[];
				record?: { type: string; summary: string }[];
				order: number;
			}[];
			status: ESPNStatus;
		}[];
	};
}

function parseMadeAttempted(val: string): [number, number] {
	const parts = val.split('-');
	return [Number(parts[0]) || 0, Number(parts[1]) || 0];
}

function pct(made: number, attempted: number): number {
	return attempted > 0 ? Math.round((made / attempted) * 1000) / 10 : 0;
}

function espnPlayerStatsToStatistics(
	keys: string[],
	stats: string[],
): Statistics {
	const get = (key: string): string => {
		const idx = keys.indexOf(key);
		return idx >= 0 ? stats[idx] : '0';
	};

	const minutes = get('minutes');
	const minuteNum = Number.parseInt(minutes, 10) || 0;
	const [fgm, fga] = parseMadeAttempted(
		get('fieldGoalsMade-fieldGoalsAttempted'),
	);
	const [tpm, tpa] = parseMadeAttempted(
		get('threePointFieldGoalsMade-threePointFieldGoalsAttempted'),
	);
	const [ftm, fta] = parseMadeAttempted(
		get('freeThrowsMade-freeThrowsAttempted'),
	);
	const reb = Number(get('rebounds')) || 0;
	const oreb = Number(get('offensiveRebounds')) || 0;
	const dreb = Number(get('defensiveRebounds')) || 0;
	const plusMinus = Number.parseInt(get('plusMinus').replace('+', ''), 10) || 0;
	const points = Number(get('points')) || 0;

	return {
		assists: Number(get('assists')) || 0,
		blocks: Number(get('blocks')) || 0,
		blocksReceived: 0,
		fieldGoalsAttempted: fga,
		fieldGoalsMade: fgm,
		fieldGoalsPercentage: pct(fgm, fga),
		foulsOffensive: 0,
		foulsDrawn: 0,
		foulsPersonal: Number(get('fouls')) || 0,
		foulsTechnical: 0,
		freeThrowsAttempted: fta,
		freeThrowsMade: ftm,
		freeThrowsPercentage: pct(ftm, fta),
		minus: plusMinus < 0 ? Math.abs(plusMinus) : 0,
		minutes: `${minutes}:00`,
		minutesCalculated: `PT${minuteNum}M`,
		plus: plusMinus > 0 ? plusMinus : 0,
		plusMinusPoints: plusMinus,
		points,
		pointsFastBreak: 0,
		pointsInThePaint: 0,
		pointsSecondChance: 0,
		reboundsDefensive: dreb,
		reboundsOffensive: oreb,
		reboundsTotal: reb,
		steals: Number(get('steals')) || 0,
		threePointersAttempted: tpa,
		threePointersMade: tpm,
		threePointersPercentage: pct(tpm, tpa),
		turnovers: Number(get('turnovers')) || 0,
		twoPointersAttempted: fga - tpa,
		twoPointersMade: fgm - tpm,
		twoPointersPercentage: pct(fgm - tpm, fga - tpa),
	};
}

function getTeamStatNum(
	stats: { name: string; displayValue: string }[],
	name: string,
): number {
	const stat = stats.find((s) => s.name === name);
	if (!stat) return 0;
	return Number(stat.displayValue) || 0;
}

function getTeamStatSplit(
	stats: { name: string; displayValue: string }[],
	name: string,
): [number, number] {
	const stat = stats.find((s) => s.name === name);
	if (!stat) return [0, 0];
	return parseMadeAttempted(stat.displayValue);
}

function espnTeamStatsToTeamStatistics(
	teamStats: { name: string; displayValue: string }[],
	teamScore: number,
	otherScore: number,
): TeamStatistics {
	const [fgm, fga] = getTeamStatSplit(
		teamStats,
		'fieldGoalsMade-fieldGoalsAttempted',
	);
	const [tpm, tpa] = getTeamStatSplit(
		teamStats,
		'threePointFieldGoalsMade-threePointFieldGoalsAttempted',
	);
	const [ftm, fta] = getTeamStatSplit(
		teamStats,
		'freeThrowsMade-freeThrowsAttempted',
	);
	const oreb = getTeamStatNum(teamStats, 'offensiveRebounds');
	const dreb = getTeamStatNum(teamStats, 'defensiveRebounds');
	const totalReb = getTeamStatNum(teamStats, 'totalRebounds');
	const turnovers = getTeamStatNum(teamStats, 'turnovers');
	const fouls = getTeamStatNum(teamStats, 'fouls');
	const assists = getTeamStatNum(teamStats, 'assists');
	const steals = getTeamStatNum(teamStats, 'steals');
	const blocks = getTeamStatNum(teamStats, 'blocks');

	return {
		assists,
		assistsTurnoverRatio: turnovers > 0 ? assists / turnovers : 0,
		benchPoints: 0,
		biggestLead: getTeamStatNum(teamStats, 'largestLead'),
		biggestLeadScore: '',
		biggestScoringRun: 0,
		biggestScoringRunScore: '',
		blocks,
		blocksReceived: 0,
		fastBreakPointsAttempted: 0,
		fastBreakPointsMade: 0,
		fastBreakPointsPercentage: 0,
		fieldGoalsAttempted: fga,
		fieldGoalsEffectiveAdjusted: 0,
		fieldGoalsMade: fgm,
		fieldGoalsPercentage: pct(fgm, fga),
		foulsOffensive: 0,
		foulsDrawn: 0,
		foulsPersonal: fouls,
		foulsTeam: 0,
		foulsTechnical: getTeamStatNum(teamStats, 'technicalFouls'),
		foulsTeamTechnical: 0,
		freeThrowsAttempted: fta,
		freeThrowsMade: ftm,
		freeThrowsPercentage: pct(ftm, fta),
		leadChanges: getTeamStatNum(teamStats, 'leadChanges'),
		minutes: 'PT240M',
		minutesCalculated: 'PT240M',
		points: teamScore,
		pointsAgainst: otherScore,
		pointsFastBreak: getTeamStatNum(teamStats, 'fastBreakPoints'),
		pointsFromTurnovers: getTeamStatNum(teamStats, 'turnoverPoints'),
		pointsInThePaint: getTeamStatNum(teamStats, 'pointsInPaint'),
		pointsInThePaintAttempted: 0,
		pointsInThePaintMade: 0,
		pointsInThePaintPercentage: 0,
		pointsSecondChance: 0,
		reboundsDefensive: dreb,
		reboundsOffensive: oreb,
		reboundsPersonal: totalReb,
		reboundsTeam: 0,
		reboundsTeamDefensive: 0,
		reboundsTeamOffensive: 0,
		reboundsTotal: totalReb,
		secondChancePointsAttempted: 0,
		secondChancePointsMade: 0,
		secondChancePointsPercentage: 0,
		steals,
		threePointersAttempted: tpa,
		threePointersMade: tpm,
		threePointersPercentage: pct(tpm, tpa),
		timeLeading: '',
		timesTied: 0,
		trueShootingAttempts: fga + 0.44 * fta,
		trueShootingPercentage:
			fga + 0.44 * fta > 0 ? teamScore / (2 * (fga + 0.44 * fta)) : 0,
		turnovers,
		turnoversTeam: getTeamStatNum(teamStats, 'teamTurnovers'),
		turnoversTotal: getTeamStatNum(teamStats, 'totalTurnovers'),
		twoPointersAttempted: fga - tpa,
		twoPointersMade: fgm - tpm,
		twoPointersPercentage: pct(fgm - tpm, fga - tpa),
	};
}

export async function fetchGameESPN(
	espnEventId: string,
): Promise<BoxScoreGame | undefined> {
	const url = `https://site.api.espn.com/apis/site/v2/sports/basketball/nba/summary?event=${espnEventId}`;

	console.log('ESPN: fetching box score for event', espnEventId);
	const response = await fetch(url);
	if (!response.ok) {
		console.warn(`ESPN summary request failed: ${response.status}`);
		return undefined;
	}

	const data = (await response.json()) as ESPNSummaryResponse;
	const comp = data.header.competitions[0];
	const status = comp.status;

	const homeComp = comp.competitors.find((c) => c.homeAway === 'home');
	const awayComp = comp.competitors.find((c) => c.homeAway === 'away');
	if (!homeComp || !awayComp) return undefined;

	const gameStatus = espnStatusToGameStatus(status);

	const buildTeam = (
		competitor: (typeof comp.competitors)[0],
		otherCompetitor: (typeof comp.competitors)[0],
	): BoxScoreTeam => {
		const espnAbbrev = competitor.team.abbreviation;
		const mapping = ESPN_TO_NBA_TEAM[espnAbbrev];

		const score = Number.parseInt(competitor.score, 10) || 0;
		const otherScore = Number.parseInt(otherCompetitor.score, 10) || 0;

		const periods: Period[] = (competitor.linescores ?? []).map((ls, i) => ({
			period: i + 1,
			periodType: i < 4 ? 'REGULAR' : 'OVERTIME',
			score: Number(ls.displayValue) || 0,
		}));

		const boxTeamData = data.boxscore.teams.find(
			(t) => t.team.abbreviation === espnAbbrev,
		);
		const boxPlayerData = data.boxscore.players.find(
			(p) => p.team.abbreviation === espnAbbrev,
		);

		const teamStats = espnTeamStatsToTeamStatistics(
			boxTeamData?.statistics ?? [],
			score,
			otherScore,
		);

		const players: Player[] = [];
		let order = 0;
		if (boxPlayerData) {
			for (const statGroup of boxPlayerData.statistics) {
				const keys = statGroup.keys;
				for (const athlete of statGroup.athletes) {
					order++;
					const a = athlete.athlete;
					const played = !athlete.didNotPlay;
					players.push({
						status: played ? 'ACTIVE' : 'INACTIVE',
						order,
						personId: Number.parseInt(a.id, 10) || 0,
						jerseyNum: a.jersey ?? '',
						position: a.position?.abbreviation,
						starter: athlete.starter ? '1' : '0',
						oncourt: '0',
						played: played ? '1' : '0',
						statistics: played
							? espnPlayerStatsToStatistics(keys, athlete.stats)
							: espnPlayerStatsToStatistics([], []),
						name: a.displayName,
						nameI: a.displayName,
						firstName: a.displayName.split(' ')[0] ?? '',
						familyName: a.displayName.split(' ').slice(1).join(' '),
						notPlayingReason: athlete.didNotPlay
							? (athlete.reason ?? 'DNP')
							: undefined,
						notPlayingDescription: athlete.didNotPlay
							? (athlete.reason ?? 'DNP')
							: undefined,
					});
				}
			}
		}

		return {
			teamId: mapping?.teamId ?? 0,
			teamName: mapping?.name ?? espnAbbrev,
			teamCity: mapping?.city ?? '',
			teamTricode: mapping?.tricode ?? espnAbbrev,
			score,
			inBonus: '',
			timeoutsRemaining: 0,
			periods,
			players,
			statistics: teamStats,
		};
	};

	const homeTeam = buildTeam(homeComp, awayComp);
	const awayTeam = buildTeam(awayComp, homeComp);

	const officials = (data.gameInfo?.officials ?? []).map((o, i) => ({
		personId: 0,
		name: o.fullName,
		nameI: o.fullName,
		firstName: o.fullName.split(' ')[0] ?? '',
		familyName: o.fullName.split(' ').slice(1).join(' '),
		jerseyNum: '',
		assignment: i === 0 ? 'CREW_CHIEF' : i === 1 ? 'REFEREE' : 'UMPIRE',
	}));

	const venue = data.gameInfo?.venue;

	return {
		gameId: espnEventId,
		gameTimeLocal: comp.date,
		gameTimeUTC: comp.date,
		gameTimeHome: comp.date,
		gameTimeAway: comp.date,
		gameEt: comp.date,
		duration: 0,
		gameCode: '',
		gameStatusText: status.type.shortDetail ?? status.type.description,
		gameStatus,
		regulationPeriods: 4,
		period: status.period,
		gameClock: status.displayClock,
		attendance: data.gameInfo?.attendance ?? 0,
		sellout: '',
		arena: {
			arenaId: venue ? Number(venue.id) || 0 : 0,
			arenaName: venue?.fullName ?? '',
			arenaCity: venue?.address?.city ?? '',
			arenaState: venue?.address?.state ?? '',
			arenaCountry: venue?.address?.country ?? 'US',
			arenaTimezone: '',
		},
		officials,
		homeTeam,
		awayTeam,
	};
}

// Standings headers matching the NBA stats.nba.com resultSets format
const STANDINGS_HEADERS: (keyof StandingsTeam)[] = [
	'LeagueID',
	'SeasonID',
	'TeamID',
	'TeamCity',
	'TeamName',
	'TeamSlug',
	'Conference',
	'ConferenceRecord',
	'PlayoffRank',
	'ClinchIndicator',
	'Division',
	'DivisionRecord',
	'DivisionRank',
	'WINS',
	'LOSSES',
	'WinPCT',
	'LeagueRank',
	'Record',
	'HOME',
	'ROAD',
	'L10',
	'Last10Home',
	'Last10Road',
	'OT',
	'ThreePTSOrLess',
	'TenPTSOrMore',
	'LongHomeStreak',
	'strLongHomeStreak',
	'LongRoadStreak',
	'strLongRoadStreak',
	'LongWinStreak',
	'LongLossStreak',
	'CurrentHomeStreak',
	'strCurrentHomeStreak',
	'CurrentRoadStreak',
	'strCurrentRoadStreak',
	'CurrentStreak',
	'strCurrentStreak',
	'ConferenceGamesBack',
	'DivisionGamesBack',
	'ClinchedConferenceTitle',
	'ClinchedDivisionTitle',
	'ClinchedPlayoffBirth',
	'ClinchedPlayIn',
	'EliminatedConference',
	'EliminatedDivision',
	'AheadAtHalf',
	'BehindAtHalf',
	'TiedAtHalf',
	'AheadAtThird',
	'BehindAtThird',
	'TiedAtThird',
	'Score100PTS',
	'OppScore100PTS',
	'OppOver500',
	'LeadInFGPCT',
	'LeadInReb',
	'FewerTurnovers',
	'PointsPG',
	'OppPointsPG',
	'DiffPointsPG',
	'vsEast',
	'vsAtlantic',
	'vsCentral',
	'vsSoutheast',
	'vsWest',
	'vsNorthwest',
	'vsPacific',
	'vsSouthwest',
	'Jan',
	'Feb',
	'Mar',
	'Apr',
	'May',
	'Jun',
	'Jul',
	'Aug',
	'Sep',
	'Oct',
	'Nov',
	'Dec',
	'Score_80_Plus',
	'Opp_Score_80_Plus',
	'Score_Below_80',
	'Opp_Score_Below_80',
	'TotalPoints',
	'OppTotalPoints',
	'DiffTotalPoints',
];

function getStatValue(
	stats: { name: string; value?: number; displayValue?: string }[],
	name: string,
): number | string {
	const stat = stats.find((s) => s.name === name);
	if (stat?.value !== undefined) return stat.value;
	if (stat?.displayValue !== undefined) return stat.displayValue;
	return 0;
}

function getStatDisplay(
	stats: { name: string; value?: number; displayValue?: string }[],
	name: string,
): string {
	const stat = stats.find((s) => s.name === name);
	return stat?.displayValue ?? '0-0';
}

// Build a NBA tricode → ESPN abbreviation reverse map
const NBA_TRICODE_TO_ESPN: Record<string, string> = {};
for (const [espnAbbrev, mapping] of Object.entries(ESPN_TO_NBA_TEAM)) {
	NBA_TRICODE_TO_ESPN[mapping.tricode] = espnAbbrev;
}

export async function fetchStandingsESPN(): Promise<Standings> {
	const url =
		'https://site.web.api.espn.com/apis/v2/sports/basketball/nba/standings?season=2025&type=0';

	console.log('ESPN: fetching standings');
	const response = await fetch(url);
	if (!response.ok) {
		throw new Error(`ESPN standings request failed: ${response.status}`);
	}

	const data = (await response.json()) as ESPNStandingsResponse;

	const rowSet: (number | string)[][] = [];
	let rank = 0;

	for (const conf of data.children) {
		const confName = conf.name.includes('East') ? 'East' : 'West';
		let confRank = 0;

		const sortedEntries = [...conf.standings.entries].sort((a, b) => {
			const seedA = a.stats.find((s) => s.name === 'playoffSeed')?.value ?? 999;
			const seedB = b.stats.find((s) => s.name === 'playoffSeed')?.value ?? 999;
			return seedA - seedB;
		});

		for (const entry of sortedEntries) {
			rank++;
			confRank++;
			const espnAbbrev = entry.team.abbreviation;
			const mapping = ESPN_TO_NBA_TEAM[espnAbbrev];
			const stats = entry.stats;

			const wins = getStatValue(stats, 'wins') as number;
			const losses = getStatValue(stats, 'losses') as number;
			const winPctRaw = getStatValue(stats, 'winPercent') as number;
			const winPct = +winPctRaw.toFixed(3);
			const ppgRaw = getStatValue(stats, 'avgPointsFor') as number;
			const ppg = +ppgRaw.toFixed(1);
			const oppPpgRaw = getStatValue(stats, 'avgPointsAgainst') as number;
			const oppPpg = +oppPpgRaw.toFixed(1);
			const diffPpg = +(ppg - oppPpg).toFixed(1);
			const gb = getStatValue(stats, 'gamesBehind') as number;
			const streak = getStatValue(stats, 'streak') as number;
			const playoffSeed = getStatValue(stats, 'playoffSeed') as number;

			const homeRecord = getStatDisplay(stats, 'Home');
			const roadRecord = getStatDisplay(stats, 'Road');
			const l10 = getStatDisplay(stats, 'Last Ten Games');
			const vsConf = getStatDisplay(stats, 'vs. Conf.');
			const vsDiv = getStatDisplay(stats, 'vs. Div.');
			const overall = getStatDisplay(stats, 'overall');

			const teamId = mapping?.teamId ?? 0;
			const teamCity = mapping?.city ?? entry.team.location;
			const teamName = mapping?.name ?? entry.team.name;
			const teamSlug = teamName.toLowerCase().replace(/\s+/g, '-');

			const row: (number | string)[] = STANDINGS_HEADERS.map((header) => {
				switch (header) {
					case 'LeagueID':
						return '00';
					case 'SeasonID':
						return '22024';
					case 'TeamID':
						return teamId;
					case 'TeamCity':
						return teamCity;
					case 'TeamName':
						return teamName;
					case 'TeamSlug':
						return teamSlug;
					case 'Conference':
						return confName;
					case 'ConferenceRecord':
						return vsConf;
					case 'PlayoffRank':
						return playoffSeed || confRank;
					case 'ClinchIndicator':
						return '';
					case 'Division':
						return '';
					case 'DivisionRecord':
						return vsDiv;
					case 'DivisionRank':
						return 0;
					case 'WINS':
						return wins;
					case 'LOSSES':
						return losses;
					case 'WinPCT':
						return winPct;
					case 'LeagueRank':
						return rank;
					case 'Record':
						return overall;
					case 'HOME':
						return homeRecord;
					case 'ROAD':
						return roadRecord;
					case 'L10':
						return l10;
					case 'CurrentStreak':
						return streak;
					case 'strCurrentStreak':
						return streak > 0 ? `W ${streak}` : `L ${Math.abs(streak)}`;
					case 'ConferenceGamesBack':
						return gb;
					case 'DivisionGamesBack':
						return 0;
					case 'PointsPG':
						return ppg;
					case 'OppPointsPG':
						return oppPpg;
					case 'DiffPointsPG':
						return diffPpg;
					case 'TotalPoints':
						return Math.round(ppg * (wins + losses));
					case 'OppTotalPoints':
						return Math.round(oppPpg * (wins + losses));
					case 'DiffTotalPoints':
						return Math.round(diffPpg * (wins + losses));
					default:
						return 0;
				}
			});

			rowSet.push(row);
		}
	}

	return {
		resource: 'leaguestandingsv3',
		parameters: {},
		resultSets: [
			{
				name: 'Standings',
				headers: STANDINGS_HEADERS as unknown as string[],
				rowSet,
			},
		],
	};
}
