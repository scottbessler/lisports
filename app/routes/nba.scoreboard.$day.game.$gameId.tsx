import { type LoaderFunction, json } from '@remix-run/node';
import { useLoaderData } from '@remix-run/react';
import invariant from 'tiny-invariant';
import { GameDetails } from '../components/GameDetails';
import { fetchGame } from '../stores/scoreboard.server';

export const loader: LoaderFunction = async ({ request, params }) => {
	invariant(params.gameId, 'gameId not found');

	const result = await fetchGame(params.gameId);
	return json({ game: result });
};

export default function GameDetailsPage() {
	const data = useLoaderData<typeof loader>();
	if (!data.game) {
		return (
			<div className="flex flex-1">
				<div className="my-24 mx-auto">
					<h3>Game has not started yet.</h3>
				</div>
			</div>
		);
	}
	return <GameDetails game={data.game} />;
}
