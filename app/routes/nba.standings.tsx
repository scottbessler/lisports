import type { LoaderArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import { fetchStandings } from "../stores/nba.standings.server";

import zipObject from "lodash.zipobject";
import partition from "lodash.partition";
import type { ColumnDef } from "../components/PrettyTable";
import { PrettyTable } from "../components/PrettyTable";
import type { StandingsTeam } from "../models/standings";
import { useMemo } from "react";

export async function loader({ request, params }: LoaderArgs) {
  const standings = await fetchStandings();

  return json({ standings });
}

export default function Scoreboard() {
  const data = useLoaderData<typeof loader>();

  const rs = data.standings.resultSets[0];
  const zipped = rs.rowSet.map((r) =>
    zipObject(rs.headers, r)
  ) as unknown[] as StandingsTeam[];
  const withId = zipped.map((z) => ({ ...z, id: String(z.TeamID) }));

  const [west, east] = partition(withId, (t) => t.Conference === "West");

  const columns = useMemo<ColumnDef<StandingsTeam & { id: string }>[]>(
    () => [
      { header: "#", accessor: (row) => ({ value: row.PlayoffRank }) },
      {
        header: "Team",
        accessor: (row) => ({ value: row.TeamName }),
        isFrozen: true,
      },
      { header: "W", accessor: (row) => ({ value: row.WINS }) },
      { header: "L", accessor: (row) => ({ value: row.LOSSES }) },
      { header: "WIN%", accessor: (row) => ({ value: row.WinPCT }) },
      { header: "GB", accessor: (row) => ({ value: row.ConferenceGamesBack }) },

      {
        header: "DIFF",
        accessor: (row) => ({
          value: row.DiffPointsPG,
        }),
      },
      {
        header: "PPG",
        accessor: (row) => ({
          value: row.PointsPG,
        }),
      },
      {
        header: "OPPG",
        accessor: (row) => ({
          value: row.OppPointsPG,
        }),
      },
      { header: "CONF", accessor: (row) => ({ value: row.ConferenceRecord }) },
      { header: "HOME", accessor: (row) => ({ value: row.HOME }) },
      { header: "ROAD", accessor: (row) => ({ value: row.ROAD }) },
      { header: "OT", accessor: (row) => ({ value: row.OT }) },
      { header: "LAST10", accessor: (row) => ({ value: row.L10 }) },
      { header: "STREAK", accessor: (row) => ({ value: row.CurrentStreak }) },
    ],
    []
  );

  return (
    <div>
      <div>
        <h1>East</h1>
        <PrettyTable columns={columns} data={east} />
      </div>
      <div>
        <h1>West</h1>
        <PrettyTable columns={columns} data={west} />
      </div>
    </div>
  );
}
