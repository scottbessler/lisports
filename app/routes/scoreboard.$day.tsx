import type { LoaderArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import { NavLink, Outlet, useLoaderData } from "@remix-run/react";

import type { Game, Team } from "../models/todaysScoreboard";

import { fetchDaysGames, fetchTodaysGames } from "../stores/game.server";
import { getTodayYMD } from "../utils";

export async function loader({ request, params }: LoaderArgs) {
  const day = params.day;
  const today = getTodayYMD();

  if (day == null || day == today) {
    return json({ games: await fetchTodaysGames() });
  }
  // todo: validate day
  return json({ games: await fetchDaysGames(day) });
}

export default function ScoreboardDay() {
  const data = useLoaderData<typeof loader>();

  return (
    <div className="flex w-80 flex-row gap-5 px-3">
      <ul className="menu">
        {data.games.map((g) => (
          <li key={g.gameId}>
            <NavLink to={`game/${g.gameId}`}>
              <GameSummary g={g} />
            </NavLink>
          </li>
        ))}
      </ul>
      <div>
        <Outlet />
      </div>
    </div>
  );
}

export function GameSummary({ g }: { g: Game }) {
  return (
    <>
      <table className="table-zebra min-w-full text-xs">
        <thead>
          <tr>
            <th scope="col" className="px-3 py-1">
              Team
            </th>
            {g.awayTeam.periods.map((p) => (
              <th scope="col" className="px-3 py-1" key={p.period}>
                {p.period}
              </th>
            ))}
            <th scope="col" className="px-3 py-1">
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
    <tr>
      <th scope="row">{team.teamName}</th>

      {team.periods.map((p) => (
        <td className="px-3 py-2" key={p.period}>
          {p.score}
        </td>
      ))}
      <td className="px-3 py-2">{team.score}</td>
    </tr>
  );
};