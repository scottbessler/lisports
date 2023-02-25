import classNames from "classnames";
import type { Game, Team } from "../models/todaysScoreboard";
import { TeamLogo } from "./TeamLogo";
import { Winner } from "./Winner";

export function GameSummary({
  g,
  showStatus,
  className,
}: {
  g: Game;
  showStatus: boolean;
  className?: string;
}) {
  return (
    <div
      className={classNames(
        className,
        "card-compact card max-w-[500px] bg-base-100 shadow-xl"
      )}
    >
      <div className="card-body">
        <table className="table-zebra table-compact table min-w-full text-xs">
          <thead>
            <tr>
              <th scope="col">Team</th>
              {g.awayTeam.periods.map((p) => (
                <th scope="col" key={p.period}>
                  {p.period}
                </th>
              ))}
              <th scope="col">Tot</th>
            </tr>
          </thead>
          <tbody>
            <GameSummaryTeamRow game={g} />
            <GameSummaryTeamRow game={g} isHome />
          </tbody>
        </table>
        {showStatus && <h3>{g.gameStatusText}</h3>}
      </div>
    </div>
  );
}

export const GameSummaryTeamRow = ({
  game,
  isHome = false,
}: {
  game: Game;
  isHome?: boolean;
}) => {
  const team = getTeam(game, isHome);
  return (
    <tr>
      <th scope="row whitespace-nowrap">
        <div className="flex flex-row">
          <TeamLogo className="mr-1 w-5" team={team} />
          <span title={`${team.teamCity} ${team.teamName}`}>
            {team.teamTricode}
          </span>
          <Winner game={game} isHome={isHome} />
        </div>
      </th>

      {team.periods.map((p) => (
        <td key={p.period}>{p.score}</td>
      ))}
      <td>{team.score}</td>
    </tr>
  );
};

export function getTeam(game: Game, isHome: boolean) {
  if (isHome) return game.homeTeam;
  return game.awayTeam;
}
