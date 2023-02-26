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
    <div className="flex flex-1 flex-col gap-2 lg:flex-row lg:px-3">
      <ul
        className={classNames("flex flex-wrap gap-4", {
          "min-w-full flex-row content-start": !hasSelectedGame,
          "flex-col  content-center ": hasSelectedGame,
        })}
      >
        {data.games.map((g) => (
          <li
            key={g.gameId}
            className={classNames({
              "hidden lg:block": hasSelectedGame && g.gameId !== params.gameId,
            })}
          >
            <Link
              className="flex hover:bg-inherit hover:shadow hover:shadow-primary focus:bg-inherit"
              to={`game/${g.gameId}`}
            >
              <GameSummary
                className={classNames(
                  "mx-auto flex min-w-full justify-center bg-base-100 p-2 text-sm",
                  {
                    "shadow shadow-primary":
                      !hasSelectedGame && g.gameId === params.gameId,
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
        className={classNames("flex flex-1", {
          // grow: hasSelectedGame,
          hidden: !hasSelectedGame,
        })}
      >
        <Outlet />
      </div>
    </div>
  );
}
