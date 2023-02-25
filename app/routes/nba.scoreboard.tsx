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
    const isLiveOrCompletedGames = today.games.some((g) => g.gameStatus >= 2);
    if (isLiveOrCompletedGames) {
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
  console.log(data.today);
  const today = dayjs(data.today);
  const days: { ymd: string; label: string }[] = [];
  for (let i = -6; i < 0; i++) {
    const d = today.add(i, "day");
    days.push({ ymd: d.format("YYYY-MM-DD"), label: d.format("ddd, MMM DD") });
  }
  days.push({ ymd: "today", label: "Today" });
  for (let i = 0; i < 6; i++) {
    const d = today.add(1 + i, "day");
    days.push({ ymd: d.format("YYYY-MM-DD"), label: d.format("ddd, MMM DD") });
  }

  return (
    <div className="flex flex-1 flex-col">
      <ul className="menu menu-compact mb-1 flex flex-row gap-2 py-3 px-3 shadow">
        {days.map((d) => (
          <li key={d.ymd}>
            <NavLink to={d.ymd}>{d.label}</NavLink>
          </li>
        ))}
      </ul>
      <Outlet />
    </div>
  );
}
