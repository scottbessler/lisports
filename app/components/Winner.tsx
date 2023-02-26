import type { BoxScoreGame } from "../models/boxScore";

export function Winner({
  game,
  className,
  isHome,
}: {
  game: {
    homeTeam: Pick<BoxScoreGame["homeTeam"], "score">;
    awayTeam: Pick<BoxScoreGame["awayTeam"], "score">;
    gameStatus: number;
  };
  className?: string;
  isHome: boolean;
}) {
  if (game.gameStatus !== 3) {
    return null;
  }
  if (isHome && game.homeTeam.score > game.awayTeam.score) {
    return <div className={className}>🏅</div>;
  } else if (!isHome && game.homeTeam.score < game.awayTeam.score) {
    return <div className={className}>🏅</div>;
  }
  return null;
}
