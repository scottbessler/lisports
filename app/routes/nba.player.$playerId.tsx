import type { LoaderArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import { result } from "lodash";
import { useMemo } from "react";
import invariant from "tiny-invariant";
import type { ResultSet, Row } from "../components/PlayerStats";
import { PLAYER_FIELD_DESCRIPTIONS } from "../components/PlayerStats";
import type { ColumnDef } from "../components/PrettyTable";
import { PrettyTable } from "../components/PrettyTable";
import { fetchPlayerStats } from "../stores/player.server";

export async function loader({ request, params }: LoaderArgs) {
  invariant(params.playerId, "playerId not found");

  const result = await fetchPlayerStats(params.playerId);
  return json({ playerStats: result });
}

export default function PlayerDetail() {
  const { playerStats } = useLoaderData<typeof loader>();

  return (
    <div>
      {playerStats.resultSets.map((t, i) => {
        if (i < 5) {
          return null;
        }
        return <PlayerResultSet key={i} resultSet={t} />;
      })}
    </div>
  );
}

export function PlayerResultSet({ resultSet }: { resultSet: ResultSet }) {
  const data = useMemo(
    () => resultSet.rowSet.map((data, id) => ({ id: String(id), data })),
    [resultSet.rowSet]
  );
  const columns = useMemo<ColumnDef<{ id: string; data: Row }>[]>(() => {
    return resultSet.headers.flatMap((h, i) => {
      const def = PLAYER_FIELD_DESCRIPTIONS[h];
      if (def == null) {
        return [];
      }
      return [
        {
          header: def.abbrev.split("<br>").join(" "),

          accessor: (r) => ({
            value: r.data[i],
          }),
        },
      ];
    });
  }, [resultSet.headers]);

  return <PrettyTable data={data} columns={columns} />;
}
