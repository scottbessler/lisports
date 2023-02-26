import type { Player, BoxScoreTeam } from "../models/boxScore";
import { useMemo } from "react";

import { pie, pointsPerShot, usageRate } from "../models/stats";
import { PrettyShooting } from "./PrettyShooting";
import type { ColumnDef } from "./PrettyTable";
import { PrettyTable } from "./PrettyTable";

import { GoodGte, BadGte, BadLte, Highlighter } from "./Stat";
import { Link } from "@remix-run/react";
import { PLAYER_FIELD_DESCRIPTIONS } from "../models/PlayerStats";

export const TeamBox = ({
  team,
  otherTeam,
}: {
  team: BoxScoreTeam;
  otherTeam: BoxScoreTeam;
}) => {
  const isWinner = team.score > otherTeam.score;

  const columns = useMemo<ColumnDef<Player & { id: string }>[]>(
    () => [
      {
        header: "Name",
        isFrozen: true,
        isHiddenWhenSmall: true,
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
        accessor: (p) => GoodGte(p.statistics.points, 20),
        sortDescFirst: true,
      },

      {
        header: "PIE",
        isHiddenWhenSmall: true,
        description: "Player Impact Estimate",
        accessor: (p) => {
          const value = pie(
            p.statistics,
            team.statistics,
            otherTeam.statistics
          );
          return GoodGte(value, 10);
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
        isHiddenWhenSmall: true,
        description: "Points per Shot'",
        accessor: (p) => {
          const v = pointsPerShot(p.statistics);
          if (!v) {
            return { value: undefined };
          }
          return GoodGte(v, 1.5);
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
        header: "RB",
        accessor: (p) => GoodGte(p.statistics.reboundsTotal, 10),
        sortDescFirst: true,
      },
      {
        header: "AS",
        accessor: (p) => GoodGte(p.statistics.assists, 8),
        sortDescFirst: true,
      },
      {
        header: "TO",
        accessor: (p) => BadGte(p.statistics.turnovers, 3),
        sortDescFirst: true,
      },
      {
        header: "ST",
        accessor: (p) => GoodGte(p.statistics.steals, 3),
        sortDescFirst: true,
      },
      {
        header: "BK",
        accessor: (p) => GoodGte(p.statistics.blocks, 3),
        sortDescFirst: true,
      },
      {
        header: "PF",
        accessor: (p) => BadGte(p.statistics.foulsPersonal, 5),
        sortDescFirst: true,
      },
      {
        header: "+/-",
        isHiddenWhenSmall: true,
        description: PLAYER_FIELD_DESCRIPTIONS.PLUS_MINUS.title,
        accessor: (p) => {
          if (isWinner) {
            return BadLte(p.statistics.plusMinusPoints, 0);
          }
          return GoodGte(p.statistics.plusMinusPoints, 0);
        },
        sortDescFirst: true,
      },
      {
        header: "USG",
        isHiddenWhenSmall: true,
        description: PLAYER_FIELD_DESCRIPTIONS.USG_PCT.title,
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
    ],
    [isWinner, otherTeam.statistics, team.statistics]
  );

  const data = useMemo(
    () =>
      team.players
        .filter((p) => p.played === "1")
        .map((p) => ({ ...p, id: String(p.personId) })),
    [team.players]
  );

  const summaryCol = useMemo<ColumnDef<Player & { id: string }>>(
    () => ({
      header: "Summary",
      accessor: (p) => {
        const pieVal = pie(p.statistics, team.statistics, otherTeam.statistics);
        const pps = pointsPerShot(p.statistics);
        return {
          value: p.name,
          cell: (
            <div className="flex flex-row gap-4">
              <span className="basis-24">
                {p.name}
                {p.starter === "1" && "*"}
              </span>
              <div>
                <Highlighter isGood={pieVal >= 10} isBad={pieVal < 0}>
                  PIE:{pieVal}
                </Highlighter>
              </div>
              <span>
                <Highlighter isGood={(pps ?? 0) > 1.5}>
                  {pps ? `${pps}pps` : ""}
                </Highlighter>
              </span>
              <span>
                <Highlighter
                  isGood={!isWinner && p.statistics.plusMinusPoints > 0}
                  isBad={isWinner && p.statistics.plusMinusPoints < 0}
                >
                  {p.statistics.plusMinusPoints > 0
                    ? `+${p.statistics.plusMinusPoints}`
                    : p.statistics.plusMinusPoints}
                </Highlighter>
              </span>
            </div>
          ),
        };
      },
    }),
    []
  );

  return (
    <PrettyTable
      className="text-xs sm:text-sm"
      columns={columns}
      summaryColumn={summaryCol}
      data={data}
    />
  );
};
