import type { LoaderArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import { Link, Outlet, useLoaderData } from "@remix-run/react";
import type { Game, Team } from "../../models/todaysScoreboard";

import { fetchDaysGames, fetchTodaysGames } from "../../stores/game.server";

export async function loader({ request, params }: LoaderArgs) {
  const day = params.day;
  if (day == null) {
    return json({ games: await fetchTodaysGames() });
  }
  // todo: validate day
  return json({ games: await fetchDaysGames(day) });
}

export default function Index() {
  const data = useLoaderData<typeof loader>();

  return (
    <div className="flex flex-row">
      <div>
        {data.games.map((g) => (
          <Link prefetch="intent" key={g.gameId} to={`game/${g.gameId}`}>
            <GameSummary g={g} />
          </Link>
        ))}
      </div>
      <div>
        <Outlet />
      </div>
    </div>
  );
}

export function GameSummary({ g }: { g: Game }) {
  return (
    <>
      {g.gameStatusText}
      <table className="min-w-full text-left text-sm text-gray-500 dark:text-gray-400">
        <thead className="bg-gray-50 text-xs uppercase text-gray-700 dark:bg-gray-700 dark:text-gray-400">
          <tr>
            <th scope="col" className="w-175 px-3 py-1">
              Team
            </th>
            {g.awayTeam.periods.map((p) => (
              <th scope="col" className="px-3 py-1" key={p.period}>
                {p.period}
              </th>
            ))}
            <th scope="col" className="px-3 py-1 text-left">
              Tot
            </th>
          </tr>
        </thead>
        <tbody>
          <GameSummaryTeamRow team={g.awayTeam} />
          <GameSummaryTeamRow team={g.homeTeam} />
        </tbody>
      </table>
    </>
  );
}

export const GameSummaryTeamRow = ({ team }: { team: Team }) => {
  return (
    <tr className="border-b bg-white dark:border-gray-700 dark:bg-gray-800">
      <th
        scope="row"
        className="whitespace-nowrap px-3 py-2 font-medium text-gray-900 dark:text-white"
      >
        {team.teamName}
      </th>

      {team.periods.map((p) => (
        <td className="px-3 py-2" key={p.period}>
          {p.score}
        </td>
      ))}
      <td className="px-3 py-2">{team.score}</td>
    </tr>
  );
};
