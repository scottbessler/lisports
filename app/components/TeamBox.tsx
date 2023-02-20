import type { Player, Team } from "../models/boxScore";
import { useMemo } from "react";

import { pie, pointsPerShot, usageRate } from "../models/stats";
import { PrettyShooting } from "./PrettyShooting";
import type { ColumnDef } from "./PrettyTable";
import { PrettyTable } from "./PrettyTable";

import { highlightGoodGte, highlightBadGte, highlightBadLte } from "./Stat";
import { Link } from "@remix-run/react";

export const TeamBox = ({
  team,
  otherTeam,
}: {
  team: Team;
  otherTeam: Team;
}) => {
  const isWinner = team.score > otherTeam.score;

  const columns = useMemo<ColumnDef<Player & { id: string }>[]>(
    () => [
      {
        header: "Name",
        isFrozen: true,
        accessor: (p) => ({
          value: p.name,
          cell: (
            <div>
              <Link to={`/nba/player/${p.personId}`}>
                {p.name}
                {p.starter === "1" && "*"}
              </Link>
            </div>
          ),
        }),
      },
      {
        header: "MIN",
        accessor: (p) => ({
          value: Number(p.statistics.minutesCalculated.slice(2, -1)),
        }),
        sortDescFirst: true,
      },
      {
        header: "PTS",
        accessor: (p) => highlightGoodGte(p.statistics.points, 20),
        sortDescFirst: true,
      },
      {
        header: "USG",
        accessor: (p) => {
          const value = usageRate(
            {
              ...p.statistics,
              minutes: Number(p.statistics.minutesCalculated.slice(2, -1)),
            },
            {
              ...team.statistics,
              minutes: Number(team.statistics.minutesCalculated.slice(2, -1)),
            }
          );
          return { value };
        },
        sortDescFirst: true,
      },
      {
        header: "PIE",
        accessor: (p) => {
          const value = pie(
            p.statistics,
            team.statistics,
            otherTeam.statistics
          );
          return { value };
        },
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
        header: "PPS",
        accessor: (p) => {
          const v = pointsPerShot(p.statistics);
          if (!v) {
            return { value: undefined };
          }
          return highlightGoodGte(v, 1.5);
        },
        sortDescFirst: true,
      },
      // {
      //   header: "TS%",
      //   accessor: (p) => {
      //     const ts = trueShooting(p.statistics);
      //     if (!ts) {
      //       return { value: undefined };
      //     }
      //     return highlightGood(Math.round(ts * 100), 60);
      //   },

      //   sortDescFirst: true,
      // },
      // {
      //   header: "OREB",
      //   accessor: (p) => ({ value: p.statistics.reboundsOffensive }),
      //   sortDescFirst: true,
      // },
      // {
      //   header: "DREB",
      //   accessor: (p) => ({ value: p.statistics.reboundsDefensive }),
      //   sortDescFirst: true,
      // },
      {
        header: "REB",
        accessor: (p) => highlightGoodGte(p.statistics.reboundsTotal, 10),
        sortDescFirst: true,
      },
      {
        header: "AST",
        accessor: (p) => highlightGoodGte(p.statistics.assists, 8),
        sortDescFirst: true,
      },
      {
        header: "TO",
        accessor: (p) => highlightBadGte(p.statistics.turnovers, 3),
        sortDescFirst: true,
      },
      {
        header: "STL",
        accessor: (p) => highlightGoodGte(p.statistics.steals, 3),
        sortDescFirst: true,
      },
      {
        header: "BLK",
        accessor: (p) => highlightGoodGte(p.statistics.blocks, 3),
        sortDescFirst: true,
      },
      {
        header: "PF",
        accessor: (p) => highlightBadGte(p.statistics.foulsPersonal, 5),
        sortDescFirst: true,
      },
      {
        header: "+/-",
        accessor: (p) => {
          if (isWinner) {
            return highlightBadLte(p.statistics.plusMinusPoints, 0);
          }
          return highlightGoodGte(p.statistics.plusMinusPoints, 0);
        },
        sortDescFirst: true,
      },
    ],
    [isWinner, team.statistics]
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
