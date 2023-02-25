import { TeamBox } from "../components/TeamBox";
import type { Game } from "../models/boxScore";
import { TeamLogo } from "./TeamLogo";
import { Winner } from "./Winner";

export function GameDetails({ game }: { game: Game }) {
  return (
    //https://www.nba.com/game/den-vs-cle-0022200886?watchFullGame
    <div className="flex flex-col gap-2">
      <div>
        <a
          href={`https://www.nba.com/game/${game.awayTeam.teamTricode}-vs-${game.homeTeam.teamTricode}-${game.gameId}?watchFullGame`}
        >
          League Pass
        </a>
      </div>
      <div className="card-compact card bg-base-100 shadow-xl">
        <div className="card-body max-w-[800px] overflow-x-scroll">
          <h1 className="card-title">
            <TeamLogo className="w-6" team={game.awayTeam} />
            {game.awayTeam.teamName} {game.awayTeam.score}{" "}
            <Winner game={game} isHome={false} />
          </h1>
          <TeamBox team={game.awayTeam} otherTeam={game.homeTeam} />
        </div>
      </div>
      <div className="card-compact card bg-base-100 shadow-xl">
        <div className="card-body max-w-[800px] overflow-x-scroll">
          <h1 className="card-title">
            <TeamLogo className="w-6" team={game.homeTeam} />
            {game.homeTeam.teamName} {game.homeTeam.score}{" "}
            <Winner game={game} isHome={true} />
          </h1>
          <TeamBox team={game.homeTeam} otherTeam={game.awayTeam} />
        </div>
      </div>
    </div>
  );
}
