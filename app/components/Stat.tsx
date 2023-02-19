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
  return <div className="badge-outline badge">{children}</div>;
};

export const GoodValue = ({ children }: { children: ReactNode }) => {
  return <div className="badge-success badge">{children}</div>;
};

export const BadValue = ({ children }: { children: ReactNode }) => {
  return <div className="badge-error badge">{children}</div>;
};
