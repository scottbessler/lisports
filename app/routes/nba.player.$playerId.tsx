import type { LoaderFunctionArgs } from 'react-router';
import { useLoaderData } from 'react-router';

import dayjs from 'dayjs';
import { useMemo } from 'react';
import invariant from 'tiny-invariant';
import type { ColumnDef } from '../components/PrettyTable';
import { PrettyTable } from '../components/PrettyTable';
import type { ResultSet, Row } from '../models/PlayerStats';
import { PLAYER_FIELD_DESCRIPTIONS } from '../models/PlayerStats';
import type { GameLogEntry } from '../stores/espn.server';
import {
	fetchPlayerGameLog,
	fetchPlayerInfo,
	fetchPlayerStats,
} from '../stores/player.server';

export const loader = async ({ params }: LoaderFunctionArgs) => {
	invariant(params.playerId, 'playerId not found');

	const [playerStats, playerInfo, gameLog] = await Promise.all([
		fetchPlayerStats(params.playerId),
		fetchPlayerInfo(params.playerId),
		fetchPlayerGameLog(params.playerId),
	]);
	return { playerStats, playerInfo: playerInfo ?? null, gameLog: gameLog ?? null };
};

export default function PlayerDetail() {
	const { playerStats, playerInfo, gameLog } = useLoaderData<typeof loader>();

	return (
		<div className="p-4">
			{playerInfo && (
				<div className="mb-6 flex items-center gap-4">
					{playerInfo.headshot && (
						<img
							src={playerInfo.headshot}
							alt={playerInfo.name}
							className="h-24 w-24 rounded-full"
						/>
					)}
					<div>
						<h1 className="text-2xl font-bold">{playerInfo.name}</h1>
						<p className="text-base-content/70">
							{[playerInfo.position, playerInfo.team]
								.filter(Boolean)
								.join(' · ')}
							{playerInfo.jersey && ` · #${playerInfo.jersey}`}
						</p>
					</div>
				</div>
			)}
			{gameLog && gameLog.games.length > 0 && (
				<PlayerGameLogTable labels={gameLog.labels} games={gameLog.games} />
			)}
			{playerStats.resultSets.map((t) => (
				<PlayerResultSet key={t.name} resultSet={t} />
			))}
		</div>
	);
}

function PlayerGameLogTable({
	labels,
	games,
}: { labels: string[]; games: GameLogEntry[] }) {
	const data = useMemo(
		() => games.map((g, i) => ({ id: String(i), ...g })),
		[games],
	);

	const columns = useMemo<ColumnDef<GameLogEntry & { id: string }>[]>(() => {
		const cols: ColumnDef<GameLogEntry & { id: string }>[] = [
			{
				header: 'Date',
				accessor: (g) => ({
					value: g.date,
					cell: dayjs(g.date).format('M/D'),
				}),
			},
			{
				header: 'Opp',
				accessor: (g) => ({
					value: g.opponent,
					cell: `${g.atVs === '@' ? '@' : 'vs'} ${g.opponent}`,
				}),
			},
			{
				header: 'Result',
				accessor: (g) => ({
					value: g.result,
					cell: (
						<span
							className={
								g.result === 'W'
									? 'text-success'
									: g.result === 'L'
										? 'text-error'
										: ''
							}
						>
							{g.result} {g.score}
						</span>
					),
				}),
			},
		];

		for (let i = 0; i < labels.length; i++) {
			const idx = i;
			cols.push({
				header: labels[idx],
				accessor: (g) => {
					const raw = g.stats[idx];
					const n = Number(raw);
					return { value: Number.isNaN(n) ? raw : n };
				},
			});
		}

		return cols;
	}, [labels]);

	return (
		<div className="mb-6">
			<h2 className="mb-2 text-lg font-semibold">Last {games.length} Games</h2>
			<PrettyTable data={data} columns={columns} />
		</div>
	);
}

export function PlayerResultSet({ resultSet }: { resultSet: ResultSet }) {
	const data = useMemo(
		() => resultSet.rowSet.map((data, id) => ({ id: String(id), data })),
		[resultSet.rowSet],
	);
	const columns = useMemo<ColumnDef<{ id: string; data: Row }>[]>(() => {
		return resultSet.headers.map((h, i) => {
			const def =
				PLAYER_FIELD_DESCRIPTIONS[h as keyof typeof PLAYER_FIELD_DESCRIPTIONS];
			return {
				header: def ? def.abbrev.split('<br>').join(' ') : h,
				description: def?.title,
				accessor: (r) => ({
					value: r.data[i],
				}),
			};
		});
	}, [resultSet.headers]);

	return (
		<div className="mb-6">
			<h2 className="mb-2 text-lg font-semibold">{resultSet.name}</h2>
			<PrettyTable data={data} columns={columns} />
		</div>
	);
}
