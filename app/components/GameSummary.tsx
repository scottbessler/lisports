import { Game, Team } from "../models/todaysScoreboard";

export function GameSummary({ g }: { g: Game }) {
  return (
    <>
      <table className="table-zebra table min-w-full text-xs">
        <thead>
          <tr>
            <th scope="col" className="px-3 py-1">
              Team
            </th>
            {g.awayTeam.periods.map((p) => (
              <th scope="col" className="px-3 py-1" key={p.period}>
                {p.period}
              </th>
            ))}
            <th scope="col" className="px-3 py-1">
              Tot
            </th>
          </tr>
        </thead>
        <tbody>
          <GameSummaryTeamRow team={g.awayTeam} />
          <GameSummaryTeamRow team={g.homeTeam} />
        </tbody>
      </table>
    </>
  );
}

export const GameSummaryTeamRow = ({ team }: { team: Team }) => {
  return (
    <tr>
      <th scope="row">{team.teamName}</th>

      {team.periods.map((p) => (
        <td className="px-3 py-2" key={p.period}>
          {p.score}
        </td>
      ))}
      <td className="px-3 py-2">{team.score}</td>
    </tr>
  );
};
