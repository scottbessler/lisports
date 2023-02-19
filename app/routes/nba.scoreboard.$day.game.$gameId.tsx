import type { LoaderArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import invariant from "tiny-invariant";
import { fetchGame } from "../stores/game.server";
import { TeamBox } from "../components/TeamBox";

export async function loader({ request, params }: LoaderArgs) {
  invariant(params.gameId, "gameId not found");

  const result = await fetchGame(params.gameId);
  return json({ game: result });
}

export default function GameDetailsPage() {
  const data = useLoaderData<typeof loader>();
  if (!data.game) {
    return (
      <div className="flex h-screen">
        <div className="mx-auto">
          <h3>Game has not started yet.</h3>
        </div>
      </div>
    );
  }
  return (
    <div className="w-full">
      <h1 className="text-lg font-bold">
        {data.game.awayTeam.teamName} {data.game.awayTeam.score}{" "}
        {data.game.awayTeam.score > data.game.homeTeam.score && "🏅"}
      </h1>
      <TeamBox
        team={data.game.awayTeam}
        isWinner={data.game.awayTeam.score > data.game.homeTeam.score}
      />
      <h1 className="text-lg font-bold">
        {data.game.homeTeam.teamName} {data.game.homeTeam.score}{" "}
        {data.game.awayTeam.score < data.game.homeTeam.score && "🏅"}
      </h1>
      <TeamBox
        team={data.game.homeTeam}
        isWinner={data.game.awayTeam.score < data.game.homeTeam.score}
      />
    </div>
  );
}
