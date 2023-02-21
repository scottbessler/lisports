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
      <div className="flex flex-1">
        <div className="m-auto">
          <h3>No Games Scheduled</h3>
        </div>
      </div>
    );
  }
  const isAllCompleted = data.games.every((g) => g.gameStatus === 3);
  return (
    <div className="flex flex-1 flex-col gap-2 px-3 xl:flex-row">
      <ul
        className={classNames("menu flex content-start", {
          "min-w-full flex-row": !hasSelectedGame,
          "flex-col xl:basis-[330px]": hasSelectedGame,
        })}
      >
        {data.games.map((g) => (
          <li
            key={g.gameId}
            className={classNames({
              "hidden xl:block": hasSelectedGame && g.gameId !== params.gameId,
            })}
          >
            <Link
              className="flex hover:bg-inherit focus:bg-inherit"
              to={`game/${g.gameId}`}
            >
              <GameSummary
                className={classNames(
                  "mx-auto w-[330px] hover:shadow-xl hover:shadow-primary",
                  {
                    "shadow-lg shadow-primary": g.gameId === params.gameId,
                  }
                )}
                g={g}
                showStatus={!isAllCompleted}
              />
            </Link>
          </li>
        ))}
      </ul>

      <div
        className={classNames({
          // grow: hasSelectedGame,
          hidden: !hasSelectedGame,
        })}
      >
        <Outlet />
      </div>
    </div>
  );
}
