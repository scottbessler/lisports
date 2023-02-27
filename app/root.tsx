import type { LinksFunction, MetaFunction } from "@remix-run/node";

import {
  Links,
  LiveReload,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import { NavBar } from "./components/NavBar";

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
      <body className="flex min-h-full flex-col-reverse justify-end bg-base-300">
        <div id="breakpoint-0" className="h-0 w-0 sm:hidden"></div>
        <div
          id="breakpoint-sm"
          className="hidden h-0 w-0 sm:max-md:block"
        ></div>
        <div
          id="breakpoint-md"
          className="hidden h-0 w-0 md:max-lg:block"
        ></div>
        <div
          id="breakpoint-lg"
          className="hidden h-0 w-0 lg:max-xl:block"
        ></div>
        <div
          id="breakpoint-xl"
          className="xl:max-2xl:block hidden h-0 w-0"
        ></div>
        <div id="breakpoint-2xl" className="2xl:block hidden h-0 w-0"></div>

        <Outlet />
        <NavBar />
        <ScrollRestoration />
        <Scripts />
        <LiveReload />
      </body>
    </html>
  );
}
