import type { Game, Team } from "../models/todaysScoreboard";

export function GameSummary({ g }: { g: Game }) {
  return (
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
        <GameSummaryTeamRow team={g.awayTeam} />
        <GameSummaryTeamRow team={g.homeTeam} />
      </tbody>
    </table>
  );
}

export const GameSummaryTeamRow = ({ team }: { team: Team }) => {
  return (
    <tr>
      <th scope="row">{team.teamName}</th>

      {team.periods.map((p) => (
        <td key={p.period}>{p.score}</td>
      ))}
      <td>{team.score}</td>
    </tr>
  );
};
