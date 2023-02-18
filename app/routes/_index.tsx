import type { LoaderFunction } from "@remix-run/node";
import { redirect } from "@remix-run/node";
import { Link } from "@remix-run/react";
import dayjs from "dayjs";

export const loader: LoaderFunction = async ({ request }) => {
  const url = new URL(request.url);
  const yesterday = dayjs().add(-1, "day").format("YYYY-MM-DD");
  if (url.pathname === "/") {
    return redirect(`/scoreboard/${yesterday}`);
  }
  return null;
};

export default function IndexRoute() {
  return (
    <main>
      <Link to="/scoreboard">Scoreboard</Link>
    </main>
  );
}
