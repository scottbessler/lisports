import { TeamBox } from "../components/TeamBox";
import { Game } from "../models/boxScore";

export function GameDetails({ game }: { game: Game }) {
  return (
    <div className="w-full">
      <h1 className="text-lg font-bold">
        {game.awayTeam.teamName} {game.awayTeam.score}{" "}
        {game.awayTeam.score > game.homeTeam.score && "🏅"}
      </h1>
      <TeamBox
        team={game.awayTeam}
        isWinner={game.awayTeam.score > game.homeTeam.score}
      />
      <h1 className="text-lg font-bold">
        {game.homeTeam.teamName} {game.homeTeam.score}{" "}
        {game.awayTeam.score < game.homeTeam.score && "🏅"}
      </h1>
      <TeamBox
        team={game.homeTeam}
        isWinner={game.awayTeam.score < game.homeTeam.score}
      />
    </div>
  );
}
