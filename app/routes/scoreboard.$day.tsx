import type { LoaderArgs } from "@remix-run/node";
import { redirect } from "@remix-run/node";
import { json } from "@remix-run/node";
import {
  NavLink,
  Outlet,
  useLoaderData,
  useMatches,
  useParams,
  useResolvedPath,
} from "@remix-run/react";
import classNames from "classnames";

import { fetchDaysGames, fetchTodaysScoreboard } from "../stores/game.server";
import { GameSummary } from "../components/GameSummary";

export async function loader({ request, params }: LoaderArgs) {
  const day = params.day;
  if (day == null) {
    return redirect("/");
  }
  // todo: validate day

  const games =
    day === "today"
      ? (await fetchTodaysScoreboard()).games
      : await fetchDaysGames(day);

  return json({ games });
}

export default function ScoreboardDay() {
  const data = useLoaderData<typeof loader>();

  const params = useParams();
  const hasSelectedGame = params.gameId != null;

  if (data.games.length === 0) {
    return (
      <div className="flex h-screen">
        <div className="m-auto">
          <h3>No Games Scheduled</h3>
        </div>
      </div>
    );
  }
  return (
    <div className="flex flex-col gap-5 px-3 lg:flex-row">
      <div>
        <ul className="menu w-full">
          {data.games.map((g) => (
            <li
              key={g.gameId}
              className={classNames({
                "hidden lg:block":
                  hasSelectedGame && g.gameId !== params.gameId,
              })}
            >
              <NavLink className="rounded-lg" to={`game/${g.gameId}`}>
                <GameSummary g={g} />
              </NavLink>
            </li>
          ))}
        </ul>
      </div>
      <div
        className={classNames({
          grow: hasSelectedGame,
          hidden: !hasSelectedGame,
        })}
      >
        <Outlet />
      </div>
    </div>
  );
}
