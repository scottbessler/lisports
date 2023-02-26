import { Link } from "@remix-run/react";
import type { BoxScoreGame } from "../models/boxScore";
import { TeamGameDetails } from "./TeamGameDetails";

export function GameDetails({ game }: { game: BoxScoreGame }) {
  return (
    <div className="mb-2 flex flex-col md:gap-2">
      <div className="flex flex-row flex-wrap gap-2">
        <TeamGameDetails game={game}></TeamGameDetails>
        <TeamGameDetails game={game} isHome></TeamGameDetails>
      </div>
      <div>
        <Link
          className="link"
          to={`https://www.nba.com/game/${game.awayTeam.teamTricode}-vs-${game.homeTeam.teamTricode}-${game.gameId}?watchFullGame`}
        >
          Watch on League Pass
        </Link>
      </div>
    </div>
  );
}
