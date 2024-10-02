import type { LoaderFunction, LoaderFunctionArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import { fetchStandings } from "../stores/nba.standings.server";

import zipObject from "lodash.zipobject";
import partition from "lodash.partition";
import type { ColumnDef, CustomRowFormatter } from "../components/PrettyTable";
import { PrettyTable } from "../components/PrettyTable";
import type { StandingsTeam } from "../models/standings";
import { useCallback, useMemo } from "react";
import {
  BadValue,
  GoodValue,
  Highlighter,
  NeutralValue,
} from "../components/Stat";
import { TeamLogo } from "../components/TeamLogo";

export const loader = async ({ request, params }: LoaderFunctionArgs) => {
  const standings = await fetchStandings();

  return json({ standings });
};

export default function Scoreboard() {
  const data = useLoaderData<typeof loader>();

  const rs = data.standings.resultSets[0];
  const zipped = rs.rowSet.map((r) =>
    zipObject(rs.headers, r)
  ) as unknown[] as StandingsTeam[];
  const withId = zipped.map((z) => ({ ...z, id: String(z.TeamID) }));

  const [west, east] = partition(withId, (t) => t.Conference === "West");
  type StandingsWithId = StandingsTeam & { id: string };

  const columns = useMemo<ColumnDef<StandingsWithId>[]>(
    () => [
      {
        header: "#",
        sortAscFirst: true,
        isHiddenWhenSmall: true,
        accessor: (row) => ({
          value: row.PlayoffRank,
          cell: (
            <div>
              {row.PlayoffRank <= 6 ? "*" : row.PlayoffRank <= 10 ? "+" : null}
              {row.PlayoffRank}
            </div>
          ),
        }),
      },
      {
        header: "Team",
        headerCell: <div className="text-left">Team</div>,
        sortAscFirst: true,
        isHiddenWhenSmall: true,
        accessor: (row) => ({
          value: row.TeamName,
          cell: (
            <div className="flex w-[120px] flex-row items-center gap-1">
              <TeamLogo
                className="w-5"
                team={{ teamId: row.TeamID, teamName: row.TeamName }}
              />
              <div>{row.TeamName}</div>
            </div>
          ),
        }),
        isFrozen: true,
      },
      { header: "W", accessor: (row) => ({ value: row.WINS }) },
      { header: "L", accessor: (row) => ({ value: row.LOSSES }) },
      { header: "%", accessor: (row) => ({ value: row.WinPCT }) },
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
        header: "PPG",
        accessor: (row) => ({
          value: row.PointsPG,
        }),
      },
      {
        header: "OPPG",
        sortAscFirst: true,
        accessor: (row) => ({
          value: row.OppPointsPG,
        }),
      },
      {
        header: "DIFF",
        accessor: (row) => ({
          value: row.DiffPointsPG,
          cell: (
            <Highlighter
              isGood={row.DiffPointsPG > 1}
              isBad={row.DiffPointsPG < -1}
            >
              {row.DiffPointsPG}
            </Highlighter>
          ),
        }),
      },
      // { header: "CONF", accessor: (row) => ({ value: row.ConferenceRecord }) },
      {
        header: "HM",
        accessor: (row) => ({
          value: row.HOME,
          cell: <DashedRecord val={row.HOME} />,
        }),
      },
      {
        header: "RD",
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
        header: "STR",
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

  const customRowFormatter = useCallback<CustomRowFormatter<StandingsWithId>>(
    ({ rowIndex, sortDir, sortHeader }) => {
      if ((sortHeader == null || sortHeader === "#") && sortDir === "desc") {
        if (rowIndex === 9) {
          return {
            trStyle: {
              borderBottomColor: "black",
              borderBottomStyle: "dotted",
              borderBottomWidth: "2px",
            },
          };
        }
        if (rowIndex === 5) {
          return {
            trStyle: {
              borderBottomColor: "black",
              borderBottomStyle: "solid",
              borderBottomWidth: "2px",
            },
          };
        }
      }
      return {};
    },
    []
  );

  const summaryCol = useMemo<ColumnDef<StandingsWithId & { id: string }>>(
    () => ({
      header: "Summary",
      accessor: (p) => {
        return {
          value: p.PlayoffRank,
          cell: (
            <div className="flex flex-row gap-4">
              <span className="flex-1 font-bold">
                {p.PlayoffRank <= 6 ? "*" : p.PlayoffRank <= 10 ? "+" : null}{" "}
                {p.PlayoffRank} {p.TeamName}
              </span>
            </div>
          ),
        };
      },
    }),
    []
  );

  return (
    <div className="flex w-full flex-row flex-wrap gap-2 px-2">
      <div className="bg-base-100 p-2 shadow-xl">
        <h2 className="">East</h2>
        <PrettyTable
          className="text-xs"
          columns={columns}
          data={east}
          summaryColumn={summaryCol}
          customRowFormatter={customRowFormatter}
        />
      </div>
      <div className="bg-base-100 p-2 shadow-xl">
        <h2 className="">West</h2>
        <PrettyTable
          className="text-xs"
          columns={columns}
          data={west}
          summaryColumn={summaryCol}
          customRowFormatter={customRowFormatter}
        />
      </div>
    </div>
  );
}

export function DashedRecord({ val }: { val: string }) {
  const [wins, losses] = val.split("-").map(Number);

  const wp = wins / (wins + losses);

  if (wp > 0.55) {
    return <GoodValue>{val}</GoodValue>;
  }
  if (wp < 0.4) {
    return <BadValue>{val}</BadValue>;
  }
  return <NeutralValue>{val}</NeutralValue>;
}
