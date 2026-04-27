import type { LoaderFunction } from 'react-router';
import { redirect, Link } from 'react-router';
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
