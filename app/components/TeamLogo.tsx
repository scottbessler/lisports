import type { Team } from "../models/boxScore";

export function TeamLogo({
  team,
  className,
}: {
  team: Pick<Team, "teamId" | "teamName">;
  className?: string;
}) {
  return (
    <img
      className={className}
      src={`https://cdn.nba.com/logos/nba/${team.teamId}/primary/L/logo.svg`}
      alt={`${team.teamName} Logo`}
      loading="lazy"
    />
  );
}
