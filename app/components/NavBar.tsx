import classNames from 'classnames';
import { NavLink } from 'react-router';
import { ThemePicker } from './ThemePicker';

export function NavBar() {
	return (
		<div className="navbar relative z-40 isolate border-t border-base-300 bg-base-100/85 backdrop-blur">
			<div className="navbar-start" />
			<div className="navbar-center flex flex-row items-center gap-3">
				<div className="px-2 text-xl normal-case">LiSports</div>
				<ul className="menu menu-horizontal items-center rounded-box bg-base-200 p-1">
					<li>
						<div className="dropdown dropdown-end dropdown-bottom">
							<button tabIndex={0} className="btn btn-ghost btn-sm">
								NBA
							</button>
							<ul className="dropdown-content menu rounded-box z-50 mt-2 w-40 border border-base-300 bg-base-100 p-2 shadow">
								<li>
									<NavLink to="/nba/scoreboard">Scoreboard</NavLink>
								</li>
								<li>
									<NavLink to="/nba/standings">Standings</NavLink>
								</li>
							</ul>
						</div>
					</li>
					<li>
						<NavLink
							to="/mlb/scoreboard"
							className={({ isActive }) =>
								classNames('btn btn-sm border-0', isActive ? 'btn-neutral' : 'btn-ghost')
							}
						>
							MLB
						</NavLink>
					</li>
					<li>
						<NavLink
							to="/nfl/scoreboard"
							className={({ isActive }) =>
								classNames('btn btn-sm border-0', isActive ? 'btn-neutral' : 'btn-ghost')
							}
						>
							NFL
						</NavLink>
					</li>
				</ul>
			</div>
			<div className="navbar-end">
				<ThemePicker />
			</div>
		</div>
	);
}
