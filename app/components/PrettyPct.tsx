export const PrettyPct = ({ pct }: { pct: number | undefined }) => (
  <>{pct == null || isNaN(pct) ? null : `${Math.round(pct * 100)}%`}</>
);
