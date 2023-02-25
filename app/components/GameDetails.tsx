import { TeamBox } from "../components/TeamBox";
import type { Game, Team } from "../models/boxScore";

export function GameDetails({ game }: { game: Game }) {
  return (
    <div className="flex flex-col gap-2">
      <div className="card-compact card bg-base-100 shadow-xl">
        <div className="card-body max-w-[800px] overflow-x-scroll">
          <h1 className="card-title">
            {game.awayTeam.teamName} {game.awayTeam.score}{" "}
            <Winner game={game} isHome={false} />
          </h1>
          <TeamBox team={game.awayTeam} otherTeam={game.homeTeam} />
        </div>
      </div>
      <div className="card-compact card bg-base-100 shadow-xl">
        <div className="card-body max-w-[800px] overflow-x-scroll">
          <h1 className="card-title">
            {game.homeTeam.teamName} {game.homeTeam.score}{" "}
            <Winner game={game} isHome={true} />
          </h1>
          <TeamBox team={game.homeTeam} otherTeam={game.awayTeam} />
        </div>
      </div>
    </div>
  );
}

export function Winner({ game, isHome }: { game: Game; isHome: boolean }) {
  if (game.gameStatus !== 3) {
    return null;
  }
  if (isHome && game.homeTeam.score > game.awayTeam.score) {
    return <>🏅</>;
  } else if (!isHome && game.homeTeam.score < game.awayTeam.score) {
    return <>🏅</>;
  }
  return null;
}
