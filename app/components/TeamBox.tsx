import type { Player, Team } from "../models/boxScore";
import { useMemo } from "react";
import { PrettyPct } from "./PrettyPct";
import { trueShooting } from "../models/stats";
import { PrettyShooting } from "./PrettyShooting";
import type { ColumnDef } from "./PrettyTable";
import { PrettyTable } from "./PrettyTable";

export const TeamBox = ({ team }: { team: Team }) => {
  const columns = useMemo<ColumnDef<Player & { id: string }>[]>(
    () => [
      { header: "Name", accessor: (p) => ({ value: p.name }) },
      {
        header: "MIN",
        accessor: (p) => ({
          value: Number(p.statistics.minutesCalculated.slice(2, -1)),
        }),
        sortDescFirst: true,
      },
      {
        header: "PTS",
        accessor: (p) => ({ value: p.statistics.points }),
        sortDescFirst: true,
      },
      {
        header: "FG",
        accessor: (p) => ({
          value: p.statistics.fieldGoalsMade,
          cell: (
            <PrettyShooting
              made={p.statistics.fieldGoalsMade}
              attempted={p.statistics.fieldGoalsAttempted}
            />
          ),
        }),
        sortDescFirst: true,
      },
      {
        header: "3P",

        accessor: (p) => ({
          value: p.statistics.threePointersMade,
          cell: (
            <PrettyShooting
              made={p.statistics.threePointersMade}
              attempted={p.statistics.threePointersAttempted}
            />
          ),
        }),
        sortDescFirst: true,
      },
      {
        header: "FT",

        accessor: (p) => ({
          value: p.statistics.freeThrowsMade,
          cell: (
            <PrettyShooting
              made={p.statistics.freeThrowsMade}
              attempted={p.statistics.freeThrowsAttempted}
            />
          ),
        }),
        sortDescFirst: true,
      },
      {
        header: "TS%",
        accessor: (p) => {
          const value = trueShooting(p.statistics);
          return { value, cell: <PrettyPct pct={value} /> };
        },

        sortType: "number",
        sortDescFirst: true,
      },
      {
        header: "OREB",
        accessor: (p) => ({ value: p.statistics.reboundsOffensive }),
        sortDescFirst: true,
      },
      {
        header: "DREB",
        accessor: (p) => ({ value: p.statistics.reboundsDefensive }),
        sortDescFirst: true,
      },
      {
        header: "REB",
        accessor: (p) => ({ value: p.statistics.reboundsTotal }),
        sortDescFirst: true,
      },
      {
        header: "AST",
        accessor: (p) => ({ value: p.statistics.assists }),
        sortDescFirst: true,
      },
      {
        header: "STL",
        accessor: (p) => ({ value: p.statistics.steals }),
        sortDescFirst: true,
      },
      {
        header: "BLK",
        accessor: (p) => ({ value: p.statistics.blocks }),
        sortDescFirst: true,
      },
      {
        header: "TO",
        accessor: (p) => ({ value: p.statistics.turnovers }),
        sortDescFirst: true,
      },
      {
        header: "PF",
        accessor: (p) => ({ value: p.statistics.foulsPersonal }),
        sortDescFirst: true,
      },
      {
        header: "+/-",
        accessor: (p) => ({ value: p.statistics.plusMinusPoints }),
        sortType: "number",
        sortDescFirst: true,
      },
    ],
    []
  );

  const data = useMemo(
    () =>
      team.players
        .filter((p) => p.played === "1")
        .map((p) => ({ ...p, id: String(p.personId) })),
    [team.players]
  );

  return <PrettyTable columns={columns} data={data} />;
};
