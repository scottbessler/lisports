import type { ReactNode } from "react";

export const highlightGoodGte = (value: number, goodGte: number) => {
  return {
    value,
    cell:
      value >= goodGte ? (
        <GoodValue>{value}</GoodValue>
      ) : (
        <NeutralValue>{value}</NeutralValue>
      ),
  };
};

export const highlightBadGte = (value: number, badGte: number) => {
  return {
    value,
    cell:
      value >= badGte ? (
        <BadValue>{value}</BadValue>
      ) : (
        <NeutralValue>{value}</NeutralValue>
      ),
  };
};

export const highlightBadLte = (value: number, badLte: number) => {
  return {
    value,
    cell:
      value <= badLte ? (
        <BadValue>{value}</BadValue>
      ) : (
        <NeutralValue>{value}</NeutralValue>
      ),
  };
};

export const NeutralValue = ({ children }: { children: ReactNode }) => {
  return <div className="">{children}</div>;
};

export const GoodValue = ({ children }: { children: ReactNode }) => {
  return (
    <div>
      <span className="relative inline-block before:absolute before:-inset-1 before:block before:-skew-y-3 before:bg-green-400 before:opacity-25">
        <span className="text-content relative">{children}</span>
      </span>
    </div>
  );
};

export const BadValue = ({ children }: { children: ReactNode }) => {
  return (
    <div>
      <span className="relative inline-block before:absolute before:-inset-1 before:block before:skew-y-3 before:bg-red-400 before:opacity-25">
        <span className="text-content relative">{children}</span>
      </span>
    </div>
  );
};
