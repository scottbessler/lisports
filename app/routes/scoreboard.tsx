import type { LoaderArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import { NavLink, Outlet, useLoaderData } from "@remix-run/react";
import dayjs from "dayjs";
import { fetchTodaysScoreboard } from "../stores/game.server";

export async function loader({ request, params }: LoaderArgs) {
  const today = await fetchTodaysScoreboard();

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
      <ul className="menu menu-compact flex flex-row py-3 px-3">
        {days.map((d) => (
          <li key={d}>
            <NavLink className="rounded" to={d.toLowerCase()}>
              {d}
            </NavLink>
          </li>
        ))}
      </ul>
      <Outlet />
    </div>
  );
}
