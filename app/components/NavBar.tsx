import { NavLink, useLocation } from '@remix-run/react';
import classNames from 'classnames';
import { ThemePicker } from './ThemePicker';

export function NavBar() {
	const loc = useLocation();
	return (
		<div className="navbar glass">
			<div className="navbar-start" />
			<div className="navbar-center flex flex-row gap-4">
				<div className="text-xl normal-case">LiSports</div>
				<ul className="menu menu-compact menu-horizontal">
					<li>
						<div className="dropdown dropdown-end dropdown-bottom">
							<button tabIndex={0}>NBA</button>
							<ul className="bg-base-100 dropdown-content menu rounded-box z-[1]  shadow">
								<li>
									<NavLink to={'/nba/scoreboard'}>Scoreboard</NavLink>
								</li>
								<li>
									<NavLink to="/nba/standings">Standings</NavLink>
								</li>
							</ul>
						</div>
					</li>
					<li>
						<NavLink to="/mlb/scoreboard">MLB</NavLink>
					</li>
					<li>
						<NavLink to="/nfl/scoreboard">NFL</NavLink>
					</li>
				</ul>
			</div>
			<div className="navbar-end">
				<ThemePicker />
			</div>
		</div>
	);
}
