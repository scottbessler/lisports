import type { Game, Team } from "../models/todaysScoreboard";

export function GameSummary({ g }: { g: Game }) {
  return (
    <div className="min-w-full">
      <table className="table-zebra table-compact table min-w-full text-xs">
        <thead>
          <tr>
            <th scope="col">Team</th>
            {g.awayTeam.periods.map((p) => (
              <th scope="col" key={p.period}>
                {p.period}
              </th>
            ))}
            <th scope="col">Tot</th>
          </tr>
        </thead>
        <tbody>
          <GameSummaryTeamRow
            team={g.awayTeam}
            isWinner={g.awayTeam.score > g.homeTeam.score}
          />
          <GameSummaryTeamRow
            team={g.homeTeam}
            isWinner={g.awayTeam.score < g.homeTeam.score}
          />
        </tbody>
      </table>
      <div className="badge">{g.gameStatusText}</div>
    </div>
  );
}

export const GameSummaryTeamRow = ({
  team,
  isWinner,
}: {
  team: Team;
  isWinner: boolean;
}) => {
  return (
    <tr>
      <th scope="row">
        {team.teamName}
        {isWinner && <span className="text-bold">🏅</span>}
      </th>

      {team.periods.map((p) => (
        <td key={p.period}>{p.score}</td>
      ))}
      <td>{team.score}</td>
    </tr>
  );
};
