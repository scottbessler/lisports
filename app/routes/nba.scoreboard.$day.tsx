import type { LoaderArgs } from "@remix-run/node";
import { redirect } from "@remix-run/node";
import { json } from "@remix-run/node";
import { Link, Outlet, useLoaderData, useParams } from "@remix-run/react";
import classNames from "classnames";

import {
  fetchDaysGames,
  fetchTodaysScoreboard,
} from "../stores/scoreboard.server";
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
      <ul
        className={classNames("menu flex", {
          "w-full flex-row": !hasSelectedGame,
          "flex-col lg:basis-[330px]": hasSelectedGame,
        })}
      >
        {data.games.map((g) => (
          <li
            key={g.gameId}
            className={classNames({
              "w-[330px]": !hasSelectedGame,
              "w-full": hasSelectedGame,
              "hidden  lg:block": hasSelectedGame && g.gameId !== params.gameId,
            })}
          >
            <Link
              className={classNames("rounded-lg px-2 py-2", {
                "bg-primary": g.gameId === params.gameId,
              })}
              to={`game/${g.gameId}`}
            >
              <GameSummary g={g} />
            </Link>
          </li>
        ))}
      </ul>

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
