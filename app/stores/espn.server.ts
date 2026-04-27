import type { Standings } from '../models/PlayerStats';
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

		for (const entry of conf.standings.entries) {
			rank++;
			const espnAbbrev = entry.team.abbreviation;
			const mapping = ESPN_TO_NBA_TEAM[espnAbbrev];
			const stats = entry.stats;

			const wins = getStatValue(stats, 'wins') as number;
			const losses = getStatValue(stats, 'losses') as number;
			const winPct = getStatValue(stats, 'winPercent') as number;
			const ppg = getStatValue(stats, 'avgPointsFor') as number;
			const oppPpg = getStatValue(stats, 'avgPointsAgainst') as number;
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
						return playoffSeed || rank;
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
