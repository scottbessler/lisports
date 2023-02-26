import classNames from "classnames";
import type { Game } from "../models/todaysScoreboard";
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
    <div className={classNames(className, "")}>
      <table className={"divide-y-gray-200 divide-y-2"}>
        <thead>
          <tr>
            <th scope="col"></th>
            {g.awayTeam.periods.map((p) => (
              <th className="text-center" scope="col" key={p.period}>
                {p.period}
              </th>
            ))}
            <th className="text-center" scope="col">
              T
            </th>
          </tr>
        </thead>
        <tbody className="divide-y-gray-200 divide-y-2">
          <GameSummaryTeamRow game={g} />
          <GameSummaryTeamRow game={g} isHome />
          {showStatus && (
            <tr>
              <th
                className="text-right"
                colSpan={g.awayTeam.periods.length + 2}
              >
                {g.gameStatusText}
              </th>
            </tr>
          )}
        </tbody>
      </table>
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
      <th scope="row whitespace-nowrap p-1">
        <div className="mr-2 flex flex-row gap-1">
          <TeamLogo className="w-5" team={team} />
          <div title={`${team.teamCity} ${team.teamName}`}>
            {team.teamTricode}
          </div>
          <Winner game={game} isHome={isHome} />
        </div>
      </th>

      {team.periods.map((p) => (
        <td className="p-1 text-right" key={p.period}>
          {p.score}
        </td>
      ))}
      <td className="text-right">{team.score}</td>
    </tr>
  );
};

export function getTeam<T extends { homeTeam: unknown; awayTeam: unknown }>(
  game: T,
  isHome: boolean
): T["homeTeam"] {
  if (isHome) return game.homeTeam;
  return game.awayTeam;
}
