import type { LoaderArgs } from "@remix-run/node";
import { redirect } from "@remix-run/node";
import { json } from "@remix-run/node";
import {
  NavLink,
  Outlet,
  useLoaderData,
  useMatches,
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

  const lastMatch = useMatches().pop();
  const resolvePath = useResolvedPath(".");
  const isLeaf = resolvePath.pathname === lastMatch?.pathname;

  return (
    <div className="flex w-80 flex-col gap-5 px-3 lg:flex-row">
      <ul
        className={classNames("menu", { hidden: !isLeaf, "lg:block": !isLeaf })}
      >
        {data.games.map((g) => (
          <li key={g.gameId}>
            <NavLink className="rounded-lg" to={`game/${g.gameId}`}>
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
