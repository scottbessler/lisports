import type { LoaderArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import invariant from "tiny-invariant";
import { fetchGame } from "../stores/game.server";
import { TeamBox } from "../components/TeamBox";

export async function loader({ request, params }: LoaderArgs) {
  invariant(params.gameId, "gameId not found");

  const result = await fetchGame(params.gameId);
  if (!result) {
    throw new Response("Not Found", { status: 404 });
  }
  return json({ game: result });
}

export default function GameDetailsPage() {
  const data = useLoaderData<typeof loader>();

  return (
    <div>
      <h1 className="text-lg font-bold">
        {data.game.awayTeam.teamName} {data.game.awayTeam.score}
      </h1>
      <TeamBox team={data.game.awayTeam} />
      <h1 className="text-lg font-bold">
        {data.game.homeTeam.teamName} {data.game.homeTeam.score}
      </h1>
      <TeamBox team={data.game.homeTeam} />
    </div>
  );
}
