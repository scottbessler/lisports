import type { LoaderArgs, SerializeFrom } from "@remix-run/node";
import { redirect } from "@remix-run/node";
import { json } from "@remix-run/node";
import {
  Link,
  Outlet,
  useLoaderData,
  useParams,
  useRouteLoaderData,
} from "@remix-run/react";
import classNames from "classnames";

import type { loader as todayLoader } from "./nba.scoreboard";
import { fetchDaysGames } from "../stores/scoreboard.server";
import { GameSummary } from "../components/GameSummary";

export async function loader({ request, params }: LoaderArgs) {
  const day = params.day;
  if (day == null) {
    return redirect("/");
  }
  // todo: validate day

  return json({ games: await fetchDaysGames(day) });
}

export default function ScoreboardDay() {
  const { day } = useParams();
  const data = useLoaderData<typeof loader>();
  const { todaysScoreboard } = useRouteLoaderData(
    "routes/nba.scoreboard"
  ) as SerializeFrom<typeof todayLoader>;
  const params = useParams();
  const hasSelectedGame = params.gameId != null;

  let games =
    todaysScoreboard.gameDate === day ? todaysScoreboard.games : data.games;

  if (games.length === 0) {
    return (
      <div className="flex flex-1">
        <div className="m-auto">
          <h3>No Games Scheduled</h3>
        </div>
      </div>
    );
  }
  const isAllCompleted = games.every((g) => g.gameStatus === 3);
  return (
    <div className="flex flex-1 flex-col gap-2 lg:flex-row lg:px-3">
      <ul
        className={classNames("flex flex-wrap gap-4", {
          "min-w-full flex-row content-start": !hasSelectedGame,
          "flex-col  content-center ": hasSelectedGame,
        })}
      >
        {games.map((g) => (
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
