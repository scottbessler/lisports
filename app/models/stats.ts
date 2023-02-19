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

export const usageRate = ({
  fieldGoalsAttempted,
  freeThrowsAttempted,
  turnovers,
  minutes,
  teamFieldGoalsAttempted,
  teamFreeThrowsAttempted,
  teamTurnovers,
  teamMinutes,
}: {
  fieldGoalsAttempted: number;
  freeThrowsAttempted: number;
  turnovers: number;
  minutes: number;
  teamFieldGoalsAttempted: number;
  teamFreeThrowsAttempted: number;
  teamTurnovers: number;
  teamMinutes: number;
}) => {
  return Math.round(
    (100 *
      ((fieldGoalsAttempted + 0.44 * freeThrowsAttempted + turnovers) *
        (teamMinutes / 5))) /
      (minutes *
        (teamFieldGoalsAttempted +
          0.44 * teamFreeThrowsAttempted +
          teamTurnovers))
  );
};

export const pointsPerShot = ({
  points,
  fieldGoalsAttempted,
}: {
  points: number;
  fieldGoalsAttempted: number;
}) => {
  if (fieldGoalsAttempted <= 0) {
    return undefined;
  }
  return Math.round((points * 100) / fieldGoalsAttempted) / 100;
};
