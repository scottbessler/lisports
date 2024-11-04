export interface StandingsTeam {
	LeagueID: string;
	SeasonID: string;
	TeamID: number;
	TeamCity: string;
	TeamName: string;
	TeamSlug: string;
	Conference: string;
	ConferenceRecord: string;
	PlayoffRank: number;
	ClinchIndicator: string;
	Division: string;
	DivisionRecord: string;
	DivisionRank: number;
	WINS: number;
	LOSSES: number;
	WinPCT: number;
	LeagueRank: number;
	Record: string;
	HOME: string;
	ROAD: string;
	L10: string;
	Last10Home: string;
	Last10Road: string;
	OT: string;
	ThreePTSOrLess: string;
	TenPTSOrMore: string;
	LongHomeStreak: number;
	strLongHomeStreak: string;
	LongRoadStreak: number;
	strLongRoadStreak: string;
	LongWinStreak: number;
	LongLossStreak: number;
	CurrentHomeStreak: number;
	strCurrentHomeStreak: string;
	CurrentRoadStreak: number;
	strCurrentRoadStreak: string;
	CurrentStreak: number;
	strCurrentStreak: string;
	ConferenceGamesBack: number;
	DivisionGamesBack: number;
	ClinchedConferenceTitle: number;
	ClinchedDivisionTitle: number;
	ClinchedPlayoffBirth: number;
	ClinchedPlayIn: number;
	EliminatedConference: number;
	EliminatedDivision: number;
	AheadAtHalf: string;
	BehindAtHalf: string;
	TiedAtHalf: string;
	AheadAtThird: string;
	BehindAtThird: string;
	TiedAtThird: string;
	Score100PTS: string;
	OppScore100PTS: string;
	OppOver500: string;
	LeadInFGPCT: string;
	LeadInReb: string;
	FewerTurnovers: string;
	PointsPG: number;
	OppPointsPG: number;
	DiffPointsPG: number;
	vsEast: string;
	vsAtlantic: string;
	vsCentral: string;
	vsSoutheast: string;
	vsWest: string;
	vsNorthwest: string;
	vsPacific: string;
	vsSouthwest: string;
	Jan: string;
	Feb: string;
	Mar?: string;
	Apr?: string;
	May?: string;
	Jun?: string;
	Jul?: string;
	Aug?: string;
	Sep?: string;
	Oct: string;
	Nov: string;
	Dec: string;
	Score_80_Plus: string;
	Opp_Score_80_Plus: string;
	Score_Below_80: string;
	Opp_Score_Below_80: string;
	TotalPoints: number;
	OppTotalPoints: number;
	DiffTotalPoints: number;
}

export const StandingsHeaders = {
	LeagueID: { title: 'LeagueID', abbrev: 'LeagueID' },
	SeasonID: { title: 'SeasonID', abbrev: 'SeasonID' },
	TeamID: { title: 'TeamID', abbrev: 'TeamID' },
	TeamCity: { title: 'TeamCity', abbrev: 'TeamCity' },
	TeamName: { title: 'TeamName', abbrev: 'TeamName' },
	TeamSlug: { title: 'TeamSlug', abbrev: 'TeamSlug' },
	Conference: { title: 'Conference', abbrev: 'Conference' },
	ConferenceRecord: { title: 'ConferenceRecord', abbrev: 'ConferenceRecord' },
	PlayoffRank: { title: 'PlayoffRank', abbrev: 'PlayoffRank' },
	ClinchIndicator: { title: 'ClinchIndicator', abbrev: 'ClinchIndicator' },
	Division: { title: 'Division', abbrev: 'Division' },
	DivisionRecord: { title: 'DivisionRecord', abbrev: 'DivisionRecord' },
	DivisionRank: { title: 'DivisionRank', abbrev: 'DivisionRank' },
	WINS: { title: 'WINS', abbrev: 'WINS' },
	LOSSES: { title: 'LOSSES', abbrev: 'LOSSES' },
	WinPCT: { title: 'WinPCT', abbrev: 'WinPCT' },
	LeagueRank: { title: 'LeagueRank', abbrev: 'LeagueRank' },
	Record: { title: 'Record', abbrev: 'Record' },
	HOME: { title: 'HOME', abbrev: 'HOME' },
	ROAD: { title: 'ROAD', abbrev: 'ROAD' },
	L10: { title: 'L10', abbrev: 'L10' },
	Last10Home: { title: 'Last10Home', abbrev: 'Last10Home' },
	Last10Road: { title: 'Last10Road', abbrev: 'Last10Road' },
	OT: { title: 'OT', abbrev: 'OT' },
	ThreePTSOrLess: { title: 'ThreePTSOrLess', abbrev: 'ThreePTSOrLess' },
	TenPTSOrMore: { title: 'TenPTSOrMore', abbrev: 'TenPTSOrMore' },
	LongHomeStreak: { title: 'LongHomeStreak', abbrev: 'LongHomeStreak' },
	strLongHomeStreak: {
		title: 'strLongHomeStreak',
		abbrev: 'strLongHomeStreak',
	},
	LongRoadStreak: { title: 'LongRoadStreak', abbrev: 'LongRoadStreak' },
	strLongRoadStreak: {
		title: 'strLongRoadStreak',
		abbrev: 'strLongRoadStreak',
	},
	LongWinStreak: { title: 'LongWinStreak', abbrev: 'LongWinStreak' },
	LongLossStreak: { title: 'LongLossStreak', abbrev: 'LongLossStreak' },
	CurrentHomeStreak: {
		title: 'CurrentHomeStreak',
		abbrev: 'CurrentHomeStreak',
	},
	strCurrentHomeStreak: {
		title: 'strCurrentHomeStreak',
		abbrev: 'strCurrentHomeStreak',
	},
	CurrentRoadStreak: {
		title: 'CurrentRoadStreak',
		abbrev: 'CurrentRoadStreak',
	},
	strCurrentRoadStreak: {
		title: 'strCurrentRoadStreak',
		abbrev: 'strCurrentRoadStreak',
	},
	CurrentStreak: { title: 'CurrentStreak', abbrev: 'CurrentStreak' },
	strCurrentStreak: { title: 'strCurrentStreak', abbrev: 'strCurrentStreak' },
	ConferenceGamesBack: {
		title: 'ConferenceGamesBack',
		abbrev: 'ConferenceGamesBack',
	},
	DivisionGamesBack: {
		title: 'DivisionGamesBack',
		abbrev: 'DivisionGamesBack',
	},
	ClinchedConferenceTitle: {
		title: 'ClinchedConferenceTitle',
		abbrev: 'ClinchedConferenceTitle',
	},
	ClinchedDivisionTitle: {
		title: 'ClinchedDivisionTitle',
		abbrev: 'ClinchedDivisionTitle',
	},
	ClinchedPlayoffBirth: {
		title: 'ClinchedPlayoffBirth',
		abbrev: 'ClinchedPlayoffBirth',
	},
	ClinchedPlayIn: { title: 'ClinchedPlayIn', abbrev: 'ClinchedPlayIn' },
	EliminatedConference: {
		title: 'EliminatedConference',
		abbrev: 'EliminatedConference',
	},
	EliminatedDivision: {
		title: 'EliminatedDivision',
		abbrev: 'EliminatedDivision',
	},
	AheadAtHalf: { title: 'AheadAtHalf', abbrev: 'AheadAtHalf' },
	BehindAtHalf: { title: 'BehindAtHalf', abbrev: 'BehindAtHalf' },
	TiedAtHalf: { title: 'TiedAtHalf', abbrev: 'TiedAtHalf' },
	AheadAtThird: { title: 'AheadAtThird', abbrev: 'AheadAtThird' },
	BehindAtThird: { title: 'BehindAtThird', abbrev: 'BehindAtThird' },
	TiedAtThird: { title: 'TiedAtThird', abbrev: 'TiedAtThird' },
	Score100PTS: { title: 'Score100PTS', abbrev: 'Score100PTS' },
	OppScore100PTS: { title: 'OppScore100PTS', abbrev: 'OppScore100PTS' },
	OppOver500: { title: 'OppOver500', abbrev: 'OppOver500' },
	LeadInFGPCT: { title: 'LeadInFGPCT', abbrev: 'LeadInFGPCT' },
	LeadInReb: { title: 'LeadInReb', abbrev: 'LeadInReb' },
	FewerTurnovers: { title: 'FewerTurnovers', abbrev: 'FewerTurnovers' },
	PointsPG: { title: 'PointsPG', abbrev: 'PointsPG' },
	OppPointsPG: { title: 'OppPointsPG', abbrev: 'OppPointsPG' },
	DiffPointsPG: { title: 'DiffPointsPG', abbrev: 'DiffPointsPG' },
	vsEast: { title: 'vsEast', abbrev: 'vsEast' },
	vsAtlantic: { title: 'vsAtlantic', abbrev: 'vsAtlantic' },
	vsCentral: { title: 'vsCentral', abbrev: 'vsCentral' },
	vsSoutheast: { title: 'vsSoutheast', abbrev: 'vsSoutheast' },
	vsWest: { title: 'vsWest', abbrev: 'vsWest' },
	vsNorthwest: { title: 'vsNorthwest', abbrev: 'vsNorthwest' },
	vsPacific: { title: 'vsPacific', abbrev: 'vsPacific' },
	vsSouthwest: { title: 'vsSouthwest', abbrev: 'vsSouthwest' },
	Jan: { title: 'Jan', abbrev: 'Jan' },
	Feb: { title: 'Feb', abbrev: 'Feb' },
	Mar: { title: 'Mar', abbrev: 'Mar' },
	Apr: { title: 'Apr', abbrev: 'Apr' },
	May: { title: 'May', abbrev: 'May' },
	Jun: { title: 'Jun', abbrev: 'Jun' },
	Jul: { title: 'Jul', abbrev: 'Jul' },
	Aug: { title: 'Aug', abbrev: 'Aug' },
	Sep: { title: 'Sep', abbrev: 'Sep' },
	Oct: { title: 'Oct', abbrev: 'Oct' },
	Nov: { title: 'Nov', abbrev: 'Nov' },
	Dec: { title: 'Dec', abbrev: 'Dec' },
	Score_80_Plus: { title: 'Score_80_Plus', abbrev: 'Score_80_Plus' },
	Opp_Score_80_Plus: {
		title: 'Opp_Score_80_Plus',
		abbrev: 'Opp_Score_80_Plus',
	},
	Score_Below_80: { title: 'Score_Below_80', abbrev: 'Score_Below_80' },
	Opp_Score_Below_80: {
		title: 'Opp_Score_Below_80',
		abbrev: 'Opp_Score_Below_80',
	},
	TotalPoints: { title: 'TotalPoints', abbrev: 'TotalPoints' },
	OppTotalPoints: { title: 'OppTotalPoints', abbrev: 'OppTotalPoints' },
	DiffTotalPoints: { title: 'DiffTotalPoints', abbrev: 'DiffTotalPoints' },
};
