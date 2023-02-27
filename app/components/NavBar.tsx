import { NavLink, useLocation } from "@remix-run/react";
import classNames from "classnames";
import { ThemePicker } from "./ThemePicker";

export function NavBar() {
  const loc = useLocation();
  return (
    <div className="navbar glass">
      <div className="navbar-start"></div>
      <div className="navbar-center flex flex-row gap-4">
        <div className="text-xl normal-case">LiSports</div>
        <ul className="menu menu-compact menu-horizontal">
          <li tabIndex={0}>
            <span
              className={classNames({
                active: loc.pathname.startsWith("/nba"),
              })}
            >
              NBA
            </span>
            <ul className="bg-base-100">
              <li>
                <NavLink to={`/nba/scoreboard`}>Scoreboard</NavLink>
              </li>
              <li>
                <NavLink to="/nba/standings">Standings</NavLink>
              </li>
            </ul>
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
