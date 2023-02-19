import type { LinksFunction, MetaFunction } from "@remix-run/node";

import {
  Links,
  LiveReload,
  Meta,
  NavLink,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import { ThemePicker } from "./components/ThemePicker";

import tailwindStylesheetUrl from "./styles/tailwind.css";

export const links: LinksFunction = () => {
  return [{ rel: "stylesheet", href: tailwindStylesheetUrl }];
};

export const meta: MetaFunction = () => ({
  charset: "utf-8",
  title: "LiSports",
  viewport: "width=device-width,initial-scale=1",
});

export default function App() {
  return (
    <html lang="en" className="h-full">
      <head>
        <Meta />
        <Links />
      </head>
      <body className="h-full">
        <div className="navbar bg-base-100">
          <div className="navbar-start"></div>
          <div className="navbar-center flex flex-row gap-4">
            <div className="text-xl normal-case">LiSports</div>
            <ul className="menu rounded-box menu-compact menu-horizontal shadow">
              <li>
                <NavLink to="/nba/scoreboard">NBA</NavLink>
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
        <Outlet />
        <ScrollRestoration />
        <Scripts />
        <LiveReload />
      </body>
    </html>
  );
}
