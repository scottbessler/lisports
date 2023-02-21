import type { LoaderArgs } from "@remix-run/node";
import { redirect } from "@remix-run/node";
import { json } from "@remix-run/node";
import { NavLink, Outlet, useLoaderData } from "@remix-run/react";
import dayjs from "dayjs";
import { fetchTodaysScoreboard } from "../stores/scoreboard.server";

export async function loader({ request, params }: LoaderArgs) {
  const today = await fetchTodaysScoreboard();

  const url = new URL(request.url);
  if (url.pathname === "/nba/scoreboard") {
    const isLiveGames = today.games.some((g) => g.gameStatus === 2);
    if (isLiveGames) {
      return redirect(`/nba/scoreboard/today`);
    } else {
      const yesterday = dayjs(today.gameDate)
        .add(-1, "day")
        .format("YYYY-MM-DD");
      return redirect(`/nba/scoreboard/${yesterday}`);
    }
  }

  return json({ today: today.gameDate });
}

export default function Scoreboard() {
  const data = useLoaderData<typeof loader>();

  const today = dayjs(data.today);
  const days: string[] = [];
  for (let i = 0; i < 7; i++) {
    days.push(today.add(-7 + i, "day").format("YYYY-MM-DD"));
  }
  days.push("Today");

  return (
    <div className="flex flex-col">
      <ul className="menu menu-compact mb-1 flex flex-row py-3 px-3 shadow">
        {days.map((d) => (
          <li key={d}>
            <NavLink to={d.toLowerCase()}>{d}</NavLink>
          </li>
        ))}
      </ul>
      <Outlet />
    </div>
  );
}
