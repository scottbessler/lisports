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
import { BadValue, GoodValue, NeutralValue } from "../components/Stat";

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
      {
        header: "GB",
        accessor: (row) => {
          if (row.ConferenceGamesBack === 0) {
            return { value: 0, cell: <div>-</div> };
          }
          return { value: row.ConferenceGamesBack };
        },
      },

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
      {
        header: "HOME",
        accessor: (row) => ({
          value: row.HOME,
          cell: <DashedRecord val={row.HOME} />,
        }),
      },
      {
        header: "ROAD",
        accessor: (row) => ({
          value: row.ROAD,
          cell: <DashedRecord val={row.ROAD} />,
        }),
      },
      // { header: "OT", accessor: (row) => ({ value: row.OT }) },
      {
        header: "L10",
        accessor: (row) => ({
          value: row.L10,
          cell: <DashedRecord val={row.L10} />,
        }),
      },

      {
        header: "STREAK",
        accessor: (row) => {
          if (row.CurrentStreak < 0) {
            return {
              value: row.CurrentStreak,
              cell: <div>L{Math.abs(row.CurrentStreak)}</div>,
            };
          }
          return {
            value: row.CurrentStreak,
            cell: <div>W{Math.abs(row.CurrentStreak)}</div>,
          };
        },
      },
    ],
    []
  );

  return (
    <div className="flex w-full flex-row gap-2 px-2">
      <div className="card  w-full bg-base-100 shadow-xl">
        <div className="card-body">
          <h2 className="card-title">East</h2>
          <PrettyTable className="text-xs" columns={columns} data={east} />
        </div>
      </div>
      <div className="card  w-full bg-base-100 shadow-xl">
        <div className="card-body">
          <h2 className="card-title">West</h2>
          <PrettyTable className="text-xs" columns={columns} data={west} />
        </div>
      </div>
    </div>
  );
}

export function DashedRecord({ val }: { val: string }) {
  const [wins, losses] = val.split("-").map(Number);

  const wp = wins / (wins + losses);

  if (wp > 0.6) {
    return <GoodValue>{val}</GoodValue>;
  }
  if (wp < 0.4) {
    return <BadValue>{val}</BadValue>;
  }
  return <NeutralValue>{val}</NeutralValue>;
}
