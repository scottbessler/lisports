import type { LoaderArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import { Link, Outlet, useLoaderData } from "@remix-run/react";

import { fetchDaysGames, fetchTodaysGames } from "../stores/game.server";

export default function Index() {
  // const data = useLoaderData<typeof loader>();

  return (
    <div className="flex flex-col">
      <div className="flex flex-row gap-10">
        <Link to="2023-02-14">2/14</Link>
        <Link to="2023-02-15">2/15</Link>
        <Link to="2023-02-16">2/16</Link>
      </div>
      <Outlet />
    </div>
  );
}
