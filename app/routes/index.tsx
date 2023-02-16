import { Link } from "@remix-run/react";

export default function IndexRoute() {
  return (
    <main>
      <Link to="/game">Scoreboard</Link>
    </main>
  );
}
