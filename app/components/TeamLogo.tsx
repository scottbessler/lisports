import classNames from 'classnames';
import type { BoxScoreTeam } from '../models/boxScore';

export function TeamLogo({
	team,
	className,
}: {
	team: Pick<BoxScoreTeam, 'teamId' | 'teamName'>;
	className?: string;
}) {
	return (
		<img
			className={classNames('max-w-none', className)}
			src={`https://cdn.nba.com/logos/nba/${team.teamId}/primary/L/logo.svg`}
			alt={`${team.teamName} Logo`}
			loading="lazy"
		/>
	);
}
