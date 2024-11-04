import { LoaderFunction, type LoaderFunctionArgs, json } from '@remix-run/node';
import { useLoaderData } from '@remix-run/react';

import { useMemo } from 'react';
import invariant from 'tiny-invariant';
import type { ColumnDef } from '../components/PrettyTable';
import { PrettyTable } from '../components/PrettyTable';
import type { ResultSet, Row } from '../models/PlayerStats';
import { PLAYER_FIELD_DESCRIPTIONS } from '../models/PlayerStats';
import { fetchPlayerStats } from '../stores/player.server';

export const loader = async ({ request, params }: LoaderFunctionArgs) => {
	invariant(params.playerId, 'playerId not found');

	const result = await fetchPlayerStats(params.playerId);
	return json({ playerStats: result });
};

export default function PlayerDetail() {
	const { playerStats } = useLoaderData<typeof loader>();

	return (
		<div>
			{playerStats.resultSets.map((t, i) => {
				if (i < 5) {
					return null;
				}
				return <PlayerResultSet key={t.name} resultSet={t} />;
			})}
		</div>
	);
}

export function PlayerResultSet({ resultSet }: { resultSet: ResultSet }) {
	const data = useMemo(
		() => resultSet.rowSet.map((data, id) => ({ id: String(id), data })),
		[resultSet.rowSet],
	);
	const columns = useMemo<ColumnDef<{ id: string; data: Row }>[]>(() => {
		return resultSet.headers.flatMap((h, i) => {
			const def =
				PLAYER_FIELD_DESCRIPTIONS[h as keyof typeof PLAYER_FIELD_DESCRIPTIONS];
			if (def == null) {
				return [];
			}
			return [
				{
					header: def.abbrev.split('<br>').join(' '),

					accessor: (r) => ({
						value: r.data[i],
					}),
				},
			];
		});
	}, [resultSet.headers]);

	return (
		<div>
			<h2>{resultSet.name}</h2>
			<PrettyTable data={data} columns={columns} />
		</div>
	);
}
