export interface TodaysScoreboard {
  meta: Meta;
  scoreboard: Scoreboard;
}

export interface Scoreboard {
  gameDate: string;
  leagueId: string;
  leagueName: string;
  games: Game[];
}

export interface Game {
  gameId: string;
  gameCode: string;
  gameStatus: number;
  gameStatusText: string;
  period: number;
  gameClock: string;
  gameTimeUTC: string;
  gameEt: string;
  regulationPeriods: number;
  ifNecessary: boolean;
  seriesGameNumber: string;
  seriesText: string;
  homeTeam: Team;
  awayTeam: Team;
  gameLeaders: GameLeaders;
  pbOdds: PbOdds;
}

export interface PbOdds {
  team?: any;
  odds: number;
  suspended: number;
}

export interface GameLeaders {
  homeLeaders: Leaders;
  awayLeaders: Leaders;
}

export interface Leaders {
  personId: number;
  name: string;
  jerseyNum: string;
  position: string;
  teamTricode: string;
  playerSlug?: string;
  points: number;
  rebounds: number;
  assists: number;
}

export interface Team {
  teamId: number;
  teamName: string;
  teamCity: string;
  teamTricode: string;
  wins: number;
  losses: number;
  score: number;
  seed?: any;
  inBonus?: string;
  timeoutsRemaining: number;
  periods: Period[];
}

export interface Period {
  period: number;
  periodType: string;
  score: number;
}

export interface Meta {
  version: number;
  request: string;
  time: string;
  code: number;
}
