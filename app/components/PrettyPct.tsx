export const PrettyPct = ({ pct }: { pct: number | undefined }) => (
	<>{pct == null || Number.isNaN(pct) ? null : `${Math.round(pct * 100)}%`}</>
);
