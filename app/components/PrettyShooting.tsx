export const PrettyShooting = ({
  made,
  attempted,
}: {
  made: number;
  attempted: number;
}) => {
  if (attempted <= 0) {
    return null;
  }
  return (
    <>
      {made}-{attempted}
    </>
  );
};
