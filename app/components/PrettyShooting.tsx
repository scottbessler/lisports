import { BadValue, GoodValue, NeutralValue } from "./Stat";

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
  if (attempted <= 3) {
    return (
      <NeutralValue>
        {made}/{attempted}
      </NeutralValue>
    );
  }
  if (made / attempted > 0.6) {
    return (
      <GoodValue>
        {made}/{attempted}
      </GoodValue>
    );
  }
  if (made / attempted < 0.4) {
    return (
      <BadValue>
        {made}/{attempted}
      </BadValue>
    );
  }
  return (
    <NeutralValue>
      {made}/{attempted}
    </NeutralValue>
  );
};
