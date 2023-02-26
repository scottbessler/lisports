import type { ReactNode } from "react";

export const Highlighter = ({
  children,
  isGood = false,
  isBad = false,
  isEmpty = false,
}: {
  children: ReactNode;
  isGood?: boolean;
  isBad?: boolean;
  isEmpty?: boolean;
}) => {
  if (isGood) {
    return <GoodValue>{children}</GoodValue>;
  } else if (isBad) {
    return <BadValue>{children}</BadValue>;
  } else if (isEmpty) {
    return <>"-"</>;
  }
  return <NeutralValue>{children}</NeutralValue>;
};
export const GoodGte = (value: number, goodGte: number) => {
  return {
    value,
    cell:
      value === 0 ? (
        " "
      ) : value >= goodGte ? (
        <GoodValue>{value}</GoodValue>
      ) : (
        <NeutralValue>{value}</NeutralValue>
      ),
  };
};

export const BadGte = (value: number, badGte: number) => {
  return {
    value,
    cell:
      value === 0 ? (
        " "
      ) : value >= badGte ? (
        <BadValue>{value}</BadValue>
      ) : (
        <NeutralValue>{value}</NeutralValue>
      ),
  };
};

export const BadLte = (value: number, badLte: number) => {
  return {
    value,
    cell:
      value === 0 ? (
        " "
      ) : value <= badLte ? (
        <BadValue>{value}</BadValue>
      ) : (
        <NeutralValue>{value}</NeutralValue>
      ),
  };
};

export const NeutralValue = ({ children }: { children: ReactNode }) => {
  return <div>{children}</div>;
};

export const GoodValue = ({ children }: { children: ReactNode }) => {
  return (
    <span className="before:-skew-y-4 relative inline-block before:absolute before:-inset-1 before:block before:bg-green-400 before:opacity-25">
      <span className="text-content relative">{children}</span>
    </span>
  );
};

export const BadValue = ({ children }: { children: ReactNode }) => {
  return (
    <span className="before:skew-y-4 relative inline-block before:absolute before:-inset-1 before:block before:bg-red-400 before:opacity-25">
      <span className="text-content relative">{children}</span>
    </span>
  );
};
