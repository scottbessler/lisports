import type { LoaderArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import invariant from "tiny-invariant";
import { fetchGame } from "../../../../stores/game.server";
// import orderBy from "lodash.orderby";
import type { Player, Team } from "../../../../models/boxScore";
import { useMemo } from "react";
import type { ColumnDef } from "@tanstack/react-table";
import { useReactTable } from "@tanstack/react-table";
import { createColumnHelper } from "@tanstack/react-table";

export async function loader({ request, params }: LoaderArgs) {
  invariant(params.gameId, "gameId not found");

  const result = await fetchGame(params.gameId);
  if (!result) {
    throw new Response("Not Found", { status: 404 });
  }
  return json({ game: result });
}

export default function NoteDetailsPage() {
  const data = useLoaderData<typeof loader>();

  return (
    <div>
      <h3 className="text-2xl font-bold">
        {data.game.awayTeam.teamName} @ {data.game.homeTeam.teamName}
      </h3>

      <TeamBox team={data.game.awayTeam} />
      <TeamBox team={data.game.homeTeam} />
    </div>
  );
}

export const PrettyPct = ({ pct }: { pct: number | undefined }) => (
  <>{pct == null || isNaN(pct) ? null : `${Math.round(pct * 100)}%`}</>
);

export const TeamBox = ({ team }: { team: Team }) => {
  const columnHelper = createColumnHelper<Player>();

  const columns = useMemo<ColumnDef<Player>[]>(
    () => [
      columnHelper.accessor("name", {
        header: "Name",
        cell: (props) => props.getValue(),
      }),
      columnHelper.accessor(
        (p) => Number(p.statistics.minutesCalculated.slice(2, -1)),
        { header: "MIN", cell: (props) => props.getValue() }
      ),
      columnHelper.accessor((p) => p.statistics.points, {
        header: "PTS",
        cell: (props) => props.getValue(),
      }),
      columnHelper.accessor((p) => p.statistics.fieldGoalsMade, {
        header: "        ",
        cell: (props) => props.getValue(),
      }),
      columnHelper.accessor((p) => p.statistics.threePointersMade, {
        header: "        ",
        cell: (props) => props.getValue(),
      }),
      columnHelper.accessor((p) => p.statistics.freeThrowsMade, {
        header: "        ",
        cell: (props) => props.getValue(),
      }),
      columnHelper.accessor((p) => trueShooting(p.statistics), {
        header: "        ",
        cell: (props) => props.getValue(),
      }),
      columnHelper.accessor((p) => p.statistics.reboundsOffensive, {
        header: "OREB",
        cell: (props) => props.getValue(),
      }),
      columnHelper.accessor((p) => p.statistics.reboundsDefensive, {
        header: "DREB",
        cell: (props) => props.getValue(),
      }),
      columnHelper.accessor((p) => p.statistics.reboundsTotal, {
        header: "REB",
        cell: (props) => props.getValue(),
      }),
      columnHelper.accessor((p) => p.statistics.assists, {
        header: "AST",
        cell: (props) => props.getValue(),
      }),
      columnHelper.accessor((p) => p.statistics.steals, {
        header: "STL",
        cell: (props) => props.getValue(),
      }),
      columnHelper.accessor((p) => p.statistics.blocks, {
        header: "BLK",
        cell: (props) => props.getValue(),
      }),
      columnHelper.accessor((p) => p.statistics.turnovers, {
        header: "TO",
        cell: (props) => props.getValue(),
      }),
      columnHelper.accessor((p) => p.statistics.foulsPersonal, {
        header: "PF",
        cell: (props) => props.getValue(),
      }),
      columnHelper.accessor((p) => p.statistics.plusMinusPoints, {
        header: '"+/-',
        cell: (props) => props.getValue(),
      }),
      // {
      //   eader: "MIN",

      //   sortDescFirst: true,
      // },
      // {
      //   eader: "PTS",

      //   sortDescFirst: true,
      // },
      // {
      //   eader: "FG",
      //   Cell: ({ row }: { row: Row<Player> }) => (
      //     <PrettyShooting
      //       made={row.original.statistics.fieldGoalsMade}
      //       attempted={row.original.statistics.fieldGoalsAttempted}
      //     />
      //   ),

      //   sortDescFirst: true,
      // },
      // {
      //   eader: "3P",
      //   Cell: ({ row }: { row: Row<Player> }) => (
      //     <PrettyShooting
      //       made={row.original.statistics.threePointersMade}
      //       attempted={row.original.statistics.threePointersAttempted}
      //     />
      //   ),

      //   sortDescFirst: true,
      // },
      // {
      //   eader: "FT",
      //   Cell: ({ row }: { row: Row<Player> }) => (
      //     <PrettyShooting
      //       made={row.original.statistics.freeThrowsMade}
      //       attempted={row.original.statistics.freeThrowsAttempted}
      //     />
      //   ),

      //   sortDescFirst: true,
      // },
      // {
      //   eader: "TS%",
      //   Cell: ({ value }: { value: number | undefined }) => (
      //     <PrettyPct pct={value} />
      //   ),

      //   sortType: "number",
      //   sortDescFirst: true,
      // },
      // {
      //   eader: "OREB",

      //   sortDescFirst: true,
      // },
      // {
      //   eader: "DREB",

      //   sortDescFirst: true,
      // },
      // {
      //   eader: "REB",

      //   sortDescFirst: true,
      // },
      // {
      //   eader: "AST",

      //   sortDescFirst: true,
      // },
      // {
      //   eader: "STL",

      //   sortDescFirst: true,
      // },
      // {
      //   eader: "BLK",

      //   sortDescFirst: true,
      // },
      // {
      //   eader: "TO",

      //   sortDescFirst: true,
      // },
      // {
      //   eader: "PF",

      //   sortDescFirst: true,
      // },
      // {
      //   eader: "+/-",

      //   sortType: "number",
      //   sortDescFirst: true,
      // },
    ],
    []
  );

  const data = useMemo(
    () => team.players.filter((p) => p.played === "1"),
    [team.players]
  );

  return <PrettyTable columns={columns} data={data} />;
};

export const PrettyTable = <T extends object>({
  columns,
  data,
}: {
  columns: ColumnDef<T>[];
  data: T[];
}) => {
  const { getTableProps, getTableBodyProps, headerGroups, rows, prepareRow } =
    useReactTable(
      {
        columns,
        data,
      },
      useSortBy
    );

  return (
    <table {...getTableProps()}>
      <thead>
        {headerGroups.map((headerGroup) => (
          // eslint-disable-next-line react/jsx-key
          <tr {...headerGroup.getHeaderGroupProps()}>
            {headerGroup.headers.map((column) => (
              // Add the sorting props to control sorting. For this example
              // we can add them into the header props
              // eslint-disable-next-line react/jsx-key
              <th {...column.getHeaderProps(column.getSortByToggleProps())}>
                {column.render("Header")}
                {/* Add a sort direction indicator */}
                <span>
                  {column.isSorted ? (column.isSortedDesc ? " 🔽" : " 🔼") : ""}
                </span>
              </th>
            ))}
          </tr>
        ))}
      </thead>
      <tbody {...getTableBodyProps()}>
        {rows.map((row, i) => {
          prepareRow(row);
          return (
            // eslint-disable-next-line react/jsx-key
            <tr {...row.getRowProps()}>
              {row.cells.map((cell) => {
                // eslint-disable-next-line react/jsx-key
                return <td {...cell.getCellProps()}>{cell.render("Cell")}</td>;
              })}
            </tr>
          );
        })}
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
