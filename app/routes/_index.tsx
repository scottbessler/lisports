import type { LoaderFunction } from '@remix-run/node';
import { redirect } from '@remix-run/node';
import { Link } from '@remix-run/react';
import dayjs from 'dayjs';

export const loader: LoaderFunction = async ({ request }) => {
	const url = new URL(request.url);
	if (url.pathname === '/') {
		return redirect('/nba/scoreboard');
	}
	return null;
};

export default function IndexRoute() {
	return (
		<main>
			<Link to="/nba/scoreboard">Scoreboard</Link>
		</main>
	);
}
