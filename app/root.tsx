import type { LinksFunction, MetaFunction } from '@remix-run/node';

import {
	Links,
	Meta,
	Outlet,
	Scripts,
	ScrollRestoration,
} from '@remix-run/react';
import { NavBar } from './components/NavBar';

import './styles/tailwind.css';

export const meta: MetaFunction = () => [
	{
		charset: 'utf-8',
		title: 'LiSports',
		viewport: 'width=device-width,initial-scale=1',
	},
];

export default function App() {
	return (
		<html lang="en" className="h-full">
			<head>
				<Meta />
				<Links />
			</head>
			<body className="bg-base-300 flex min-h-full flex-col-reverse justify-end">
				<div id="breakpoint-0" className="h-0 w-0 sm:hidden" />
				<div id="breakpoint-sm" className="hidden h-0 w-0 sm:max-md:block" />
				<div id="breakpoint-md" className="hidden h-0 w-0 md:max-lg:block" />
				<div id="breakpoint-lg" className="hidden h-0 w-0 lg:max-xl:block" />
				<div id="breakpoint-xl" className="hidden h-0 w-0 xl:max-2xl:block" />
				<div id="breakpoint-2xl" className="hidden h-0 w-0 2xl:block" />

				<Outlet />
				<NavBar />
				<ScrollRestoration />
				<Scripts />
			</body>
		</html>
	);
}
