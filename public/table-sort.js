(() => {
  const valueFor = (cell) => {
    const text = (cell?.innerText || "").trim();
    if (!text || text === "-") return { kind: "empty", value: "" };
    const record = text.match(/^(\d+)-(\d+)$/);
    if (record) {
      const wins = Number(record[1]);
      const losses = Number(record[2]);
      return { kind: "number", value: wins / Math.max(1, wins + losses) };
    }
    const streak = text.match(/^[WL](-?\d+)$/i);
    if (streak) {
      const amount = Number(streak[1]);
      return { kind: "number", value: text[0].toUpperCase() === "W" ? amount : -amount };
    }
    const numeric = Number(text.replace(/[%,$]/g, ""));
    if (Number.isFinite(numeric) && /^[-+]?[\d,.]+%?$/.test(text)) {
      return { kind: "number", value: numeric };
    }
    return { kind: "text", value: text.toLocaleLowerCase() };
  };

  document.querySelectorAll("table.sortable").forEach((table) => {
    const headers = Array.from(table.tHead?.rows?.[0]?.cells || []);
    const body = table.tBodies[0];
    if (!body) return;

    headers.forEach((header, index) => {
      header.tabIndex = 0;
      header.setAttribute("role", "button");
      header.setAttribute("aria-sort", "none");

      const sort = () => {
        const nextDir = header.dataset.sortDir === "asc" ? "desc" : "asc";
        headers.forEach((h) => {
          h.dataset.sortDir = "";
          h.setAttribute("aria-sort", "none");
        });
        header.dataset.sortDir = nextDir;
        header.setAttribute("aria-sort", nextDir === "asc" ? "ascending" : "descending");

        const rows = Array.from(body.rows);
        rows.sort((a, b) => {
          const left = valueFor(a.cells[index]);
          const right = valueFor(b.cells[index]);
          if (left.kind === "empty" && right.kind !== "empty") return 1;
          if (right.kind === "empty" && left.kind !== "empty") return -1;
          const result = left.kind === "number" && right.kind === "number"
            ? left.value - right.value
            : String(left.value).localeCompare(String(right.value), undefined, { numeric: true });
          return nextDir === "asc" ? result : -result;
        });
        rows.forEach((row) => body.appendChild(row));
      };

      header.addEventListener("click", sort);
      header.addEventListener("keydown", (event) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          sort();
        }
      });
    });
  });
})();
