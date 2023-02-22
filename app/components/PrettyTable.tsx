import classNames from "classnames";
import orderBy from "lodash.orderby";
import { useCallback, useMemo, useState } from "react";

export const PrettyTable = <T extends { id: string }>({
  columns,
  data,
  className,
}: {
  columns: ColumnDef<T>[];
  data: T[];
  className?: string;
}) => {
  const [sortHeader, setSortHeader] = useState<string | undefined>();
  const [sortDir, setSortDir] = useState<"asc" | "desc">("desc");

  const onHeaderClick = useCallback(
    (h: string) => {
      if (sortHeader === h) {
        setSortDir((curr) => (curr === "asc" ? "desc" : "asc"));
      } else {
        setSortHeader(h);
        setSortDir("desc");
      }
    },
    [sortHeader]
  );

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
      className={classNames(
        "table-zebra table-compact table min-w-full",
        className
      )}
    >
      <thead>
        <tr>
          {columnsWithExtras.map((c) => (
            <th
              className="cursor-pointer text-center"
              key={c.header}
              onClick={c.onClick}
            >
              {c.header}
            </th>
          ))}
        </tr>
      </thead>
      <tbody>
        {sortedData.map((row, i) => (
          <tr key={row.id}>
            {columns.map((c) => {
              const { value, cell } = c.accessor(row);
              if (c.isFrozen) {
                return (
                  <th
                    className="whitespace-nowrap text-left"
                    key={`${row.id}-${c.header}`}
                  >
                    {cell || value}
                  </th>
                );
              } else {
                return (
                  <td
                    className="whitespace-nowrap text-right"
                    key={`${row.id}-${c.header}`}
                  >
                    {cell || value}
                  </td>
                );
              }
            })}
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export interface ColumnDef<T extends { id: string }> {
  header: string;
  isFrozen?: boolean;

  accessor: (row: T) => {
    value: string | number | boolean | null | undefined;
    cell?: React.ReactNode;
  };

  sortDescFirst?: boolean;
}
