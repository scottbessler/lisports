import { TeamBox } from "../components/TeamBox";
import type { Game } from "../models/boxScore";

export function GameDetails({ game }: { game: Game }) {
  return (
    <div className="flex min-w-full flex-col gap-2">
      <div className="card card-compact bg-base-100 shadow-xl">
        <div className="card-body overflow-x-scroll">
          <h1 className="card-title">
            {game.awayTeam.teamName} {game.awayTeam.score}{" "}
            {game.awayTeam.score > game.homeTeam.score && "🏅"}
          </h1>
          <TeamBox team={game.awayTeam} otherTeam={game.homeTeam} />
        </div>
      </div>
      <div className="card card-compact bg-base-100 shadow-xl">
        <div className="card-body">
          <h1 className="card-title">
            {game.homeTeam.teamName} {game.homeTeam.score}{" "}
            {game.awayTeam.score < game.homeTeam.score && "🏅"}
          </h1>
          <TeamBox team={game.homeTeam} otherTeam={game.awayTeam} />
        </div>
      </div>
    </div>
  );
}
