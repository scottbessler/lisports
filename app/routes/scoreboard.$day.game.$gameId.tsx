import type { LoaderArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import invariant from "tiny-invariant";
import { fetchGame } from "../stores/game.server";
import orderBy from "lodash.orderby";
import type { Player, Team } from "../models/boxScore";
import { useCallback, useMemo, useState } from "react";

export async function loader({ request, params }: LoaderArgs) {
  invariant(params.gameId, "gameId not found");

  const result = await fetchGame(params.gameId);
  if (!result) {
    throw new Response("Not Found", { status: 404 });
  }
  return json({ game: result });
}

export default function GameDetailsPage() {
  const data = useLoaderData<typeof loader>();

  return (
    <div>
      <h1 className="text-lg font-bold">
        {data.game.awayTeam.teamName} {data.game.awayTeam.score}
      </h1>
      <TeamBox team={data.game.awayTeam} />
      <h1 className="text-lg font-bold">
        {data.game.homeTeam.teamName} {data.game.homeTeam.score}
      </h1>
      <TeamBox team={data.game.homeTeam} />
    </div>
  );
}

export const PrettyPct = ({ pct }: { pct: number | undefined }) => (
  <>{pct == null || isNaN(pct) ? null : `${Math.round(pct * 100)}%`}</>
);

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

interface ColumnDef<T extends { id: string }> {
  header: string;

  accessor: (row: T) => {
    value: string | number | boolean | null | undefined;
    cell?: React.ReactNode;
  };

  sortDescFirst?: boolean;
}

export const PrettyTable = <T extends { id: string }>({
  columns,
  data,
}: {
  columns: ColumnDef<T>[];
  data: T[];
}) => {
  const [sortHeader, setSortHeader] = useState<string | undefined>();
  const [sortDir, setSortDir] = useState<"asc" | "desc">("desc");

  const onHeaderClick = useCallback(
    (h: string) => {
      if (sortHeader === h) {
        setSortDir((curr) => (curr === "asc" ? "desc" : "asc"));
      } else {
        setSortHeader(h);
        setSortDir("desc");
      }
    },
    [sortHeader]
  );

  const columnsWithExtras = useMemo(
    () =>
      columns.map((c) => {
        // todo: could create a unique id for tracking which was clicked
        return {
          ...c,
          onClick: () => onHeaderClick(c.header),
        };
      }),
    [columns, onHeaderClick]
  );

  const sortByColumnDef = useMemo(
    () => columns.find((c) => c.header === sortHeader),
    [sortHeader, columns]
  );

  const sortedData = useMemo(() => {
    if (sortByColumnDef == null) {
      return data;
    }
    return orderBy(data, (d) => sortByColumnDef.accessor(d).value, sortDir);
  }, [data, sortByColumnDef, sortDir]);

  return (
    <table className="table-zebra">
      <thead>
        <tr>
          {columnsWithExtras.map((c) => (
            <th
              className="cursor-pointer px-1"
              key={c.header}
              onClick={c.onClick}
            >
              {c.header}
            </th>
          ))}
          <th></th>
        </tr>
      </thead>
      <tbody>
        {sortedData.map((row, i) => (
          <tr key={row.id}>
            {columns.map((c) => {
              const { value, cell } = c.accessor(row);
              return (
                <td
                  className="whitespace-nowrap px-1"
                  key={`${row.id}-${c.header}`}
                >
                  {cell || value}
                </td>
              );
            })}
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export const PrettyShooting = ({
  made,
  attempted,
}: {
  made: number;
  attempted: number;
}) => {
  if (attempted <= 0) {
    return null;
  }
  return (
    <>
      {made}-{attempted}
    </>
  );
};

export const trueShooting = ({
  points,
  fieldGoalsAttempted,
  freeThrowsAttempted,
}: {
  points: number;
  fieldGoalsAttempted: number;
  freeThrowsAttempted: number;
}) => {
  if (fieldGoalsAttempted + freeThrowsAttempted <= 0) {
    return undefined;
  }
  return (0.5 * points) / (fieldGoalsAttempted + 0.475 * freeThrowsAttempted);
};
