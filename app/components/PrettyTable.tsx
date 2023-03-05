import classNames from "classnames";
import orderBy from "lodash.orderby";
import keyBy from "lodash.keyby";
import type { CSSProperties } from "react";
import { useCallback, useMemo, useState } from "react";

export const PrettyTable = <T extends { id: string }>({
  columns,
  summaryColumn = columns[0],
  customRowFormatter,
  data,
  className,
}: {
  columns: ColumnDef<T>[];

  /** Used for small screens */
  summaryColumn?: ColumnDef<T>;

  customRowFormatter?: CustomRowFormatter<T>;

  data: T[];
  className?: string;
}) => {
  const [sortHeader, setSortHeader] = useState<string | undefined>();
  const [sortDir, setSortDir] = useState<"asc" | "desc">("desc");
  const columnsByHeader = useMemo(() => keyBy(columns, "header"), [columns]);
  const onHeaderClick = useCallback(
    (h: string) => {
      if (sortHeader === h) {
        setSortDir((curr) => (curr === "asc" ? "desc" : "asc"));
      } else {
        setSortHeader(h);
        const c = columnsByHeader[h];
        if (c?.sortAscFirst) {
          setSortDir("asc");
        } else {
          setSortDir("desc");
        }
      }
    },
    [columnsByHeader, sortHeader]
  );

  if (summaryColumn === columns[0]) {
    columns[0] = { ...columns[0], isHiddenWhenSmall: true };
  }
  const columnsWithExtras = useMemo(
    () =>
      columns.map((c) => {
        // todo: could create a unique id for tracking which was clicked
        return {
          ...c,
          onClick: () => onHeaderClick(c.header),
        };
      }),
    [columns, onHeaderClick]
  );

  const sortByColumnDef = useMemo(
    () => columns.find((c) => c.header === sortHeader),
    [sortHeader, columns]
  );

  const sortedData = useMemo(() => {
    if (sortByColumnDef == null) {
      return data;
    }
    return orderBy(
      data,
      (d) => sortByColumnDef.accessor(d).value ?? 0,
      sortDir
    );
  }, [data, sortByColumnDef, sortDir]);

  return (
    <table
      className={classNames("min-w-full divide-y divide-gray-200", className)}
    >
      <thead>
        <tr className="divide-x divide-base-300">
          {columnsWithExtras.map((c) => (
            <th
              className={classNames("cursor-pointer px-1 text-right ", {
                "hidden md:table-cell": c.isHiddenWhenSmall,
              })}
              title={c.description}
              key={c.header}
              onClick={c.onClick}
            >
              {c.headerCell || c.header}
            </th>
          ))}
        </tr>
      </thead>
      <tbody className="divide-y divide-base-300">
        {sortedData.map((row, i) => {
          const { trStyle } =
            customRowFormatter?.({
              data: row,
              rowIndex: i,
              sortHeader,
              sortDir,
            }) ?? {};

          return (
            <>
              <tr className="md:hidden" key={`${row.id}-h`}>
                <td
                  colSpan={columns.length}
                  className="whitespace-nowrap py-1 px-1 text-left"
                  key={`${row.id}-summary`}
                >
                  <ColumnValue row={row} columnDef={summaryColumn} />
                </td>
              </tr>
              <tr
                key={row.id}
                style={trStyle}
                className="divide-x divide-base-300"
              >
                {columns.map((c) => {
                  return (
                    <td
                      className={classNames(
                        "whitespace-nowrap py-1 px-1 text-right  md:py-2",
                        {
                          "font-bold": c.isFrozen,
                          "hidden md:table-cell": c.isHiddenWhenSmall,
                        }
                      )}
                      key={`${row.id}-${c.header}`}
                    >
                      <ColumnValue row={row} columnDef={c} />
                    </td>
                  );
                })}
              </tr>
            </>
          );
        })}
      </tbody>
    </table>
  );
};

export function ColumnValue<T extends { id: string }>({
  row,
  columnDef,
}: {
  row: T;
  columnDef: ColumnDef<T>;
}) {
  const { value, cell } = columnDef.accessor(row);

  return <>{cell || value}</>;
}

export interface ColumnDef<T extends { id: string }> {
  header: string;
  headerCell?: React.ReactNode;

  description?: string;

  isFrozen?: boolean;
  isHiddenWhenSmall?: boolean;

  accessor: (row: T) => {
    value: string | number | boolean | null | undefined;
    cell?: React.ReactNode;
  };

  sortAscFirst?: boolean;
}

export type CustomRowFormatter<T> = (props: {
  rowIndex: number;
  data: T;
  sortHeader?: string;
  sortDir: "asc" | "desc";
}) => { trStyle?: CSSProperties | undefined };
