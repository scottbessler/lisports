import type { Game } from "../models/boxScore";

export function Winner({
  game,
  isHome,
}: {
  game: {
    homeTeam: Pick<Game["homeTeam"], "score">;
    awayTeam: Pick<Game["awayTeam"], "score">;
    gameStatus: number;
  };
  isHome: boolean;
}) {
  if (game.gameStatus !== 3) {
    return null;
  }
  if (isHome && game.homeTeam.score > game.awayTeam.score) {
    return <>🏅</>;
  } else if (!isHome && game.homeTeam.score < game.awayTeam.score) {
    return <>🏅</>;
  }
  return null;
}
