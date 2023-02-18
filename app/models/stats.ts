export const trueShooting = ({
  points,
  fieldGoalsAttempted,
  freeThrowsAttempted,
}: {
  points: number;
  fieldGoalsAttempted: number;
  freeThrowsAttempted: number;
}) => {
  if (fieldGoalsAttempted + freeThrowsAttempted <= 0) {
    return undefined;
  }
  return (0.5 * points) / (fieldGoalsAttempted + 0.475 * freeThrowsAttempted);
};
