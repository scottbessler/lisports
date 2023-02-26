import { TeamBox } from "../components/TeamBox";
import type { BoxScoreGame } from "../models/boxScore";
import { getTeam } from "./GameSummary";
import { TeamLogo } from "./TeamLogo";
import { Winner } from "./Winner";

export function TeamGameDetails({
  game,
  isHome = false,
}: {
  game: BoxScoreGame;
  isHome?: boolean;
}) {
  const team = getTeam(game, isHome);
  const otherTeam = getTeam(game, !isHome);
  return (
    <div className="min-w-full bg-base-100 p-2 md:min-w-[700px] md:max-w-[700px]">
      <h1 className="flex flex-row items-center gap-2">
        <TeamLogo className="w-6" team={team} />
        <span className="">
          {team.teamCity} {team.teamName}{" "}
        </span>
        <span className=" font-bold">{team.score}</span>{" "}
        <Winner className="" game={game} isHome={isHome} />
      </h1>
      <TeamBox team={team} otherTeam={otherTeam} />
    </div>
  );
}
