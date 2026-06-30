(() => {
	const scheduleReload = () => {
		const refreshAt = document.body?.dataset.refreshAt;
		if (!refreshAt) return;

		const refreshTime = new Date(refreshAt).valueOf();
		if (Number.isNaN(refreshTime)) return;

		const delay = Math.max(0, refreshTime - Date.now());
		window.setTimeout(() => {
			window.location.reload();
		}, delay);
	};

	const formatLocalTimes = () => {
		document.querySelectorAll('time[data-local-game-time]').forEach((time) => {
			const date = new Date(time.dateTime);
			if (Number.isNaN(date.valueOf())) return;
			const opts = { hour: 'numeric', minute: '2-digit', timeZoneName: 'short' };
			if (time.hasAttribute('data-show-date')) {
				opts.month = 'numeric';
				opts.day = 'numeric';
			}
			time.textContent = new Intl.DateTimeFormat(undefined, opts).format(date);
		});
	};

	const valueFor = (cell) => {
		const text = (cell?.innerText || '').trim();
		if (!text || text === '-') return { kind: 'empty', value: '' };
		const record = text.match(/^(\d+)-(\d+)$/);
		if (record) {
			const wins = Number(record[1]);
			const losses = Number(record[2]);
			return { kind: 'number', value: wins / Math.max(1, wins + losses) };
		}
		const streak = text.match(/^[WL](-?\d+)$/i);
		if (streak) {
			const amount = Number(streak[1]);
			return { kind: 'number', value: text[0].toUpperCase() === 'W' ? amount : -amount };
		}
		const numeric = Number(text.replace(/[%,$]/g, ''));
		if (Number.isFinite(numeric) && /^[-+]?[\d,.]+%?$/.test(text)) {
			return { kind: 'number', value: numeric };
		}
		return { kind: 'text', value: text.toLocaleLowerCase() };
	};

	const applySort = (table, index, direction) => {
		const headers = Array.from(table.tHead?.rows?.[0]?.cells || []);
		const body = table.tBodies[0];
		const header = headers[index];
		if (!body || !header) return;

		headers.forEach((h) => {
			h.dataset.sortDir = '';
			h.setAttribute('aria-sort', 'none');
		});
		header.dataset.sortDir = direction;
		header.setAttribute('aria-sort', direction === 'asc' ? 'ascending' : 'descending');
		table.dataset.activeSortIndex = String(index);
		table.dataset.activeSortDir = direction;

		const rows = Array.from(body.rows);
		rows.sort((a, b) => {
			const left = valueFor(a.cells[index]);
			const right = valueFor(b.cells[index]);
			if (left.kind === 'empty' && right.kind !== 'empty') return 1;
			if (right.kind === 'empty' && left.kind !== 'empty') return -1;
			const result =
				left.kind === 'number' && right.kind === 'number'
					? left.value - right.value
					: String(left.value).localeCompare(String(right.value), undefined, { numeric: true });
			return direction === 'asc' ? result : -result;
		});
		rows.forEach((row) => body.appendChild(row));
	};

	const linkedTables = (table) => {
		const group = table.dataset.sortGroup;
		const safeGroup = group?.replace(/["\\]/g, '\\$&');
		return group
			? Array.from(document.querySelectorAll(`table.sortable[data-sort-group="${safeGroup}"]`))
			: [table];
	};

	document.querySelectorAll('table.sortable').forEach((table) => {
		const headers = Array.from(table.tHead?.rows?.[0]?.cells || []);
		if (!table.tBodies[0]) return;

		headers.forEach((header, index) => {
			header.tabIndex = 0;
			header.setAttribute('role', 'button');
			header.setAttribute('aria-sort', 'none');

			const sort = () => {
				const nextDir = header.dataset.sortDir === 'asc' ? 'desc' : 'asc';
				linkedTables(table).forEach((linkedTable) => applySort(linkedTable, index, nextDir));
			};

			header.addEventListener('click', sort);
			header.addEventListener('keydown', (event) => {
				if (event.key === 'Enter' || event.key === ' ') {
					event.preventDefault();
					sort();
				}
			});
		});
	});

	document.querySelectorAll('table.sortable[data-default-sort-index]').forEach((table) => {
		const index = Number(table.dataset.defaultSortIndex);
		if (!Number.isInteger(index)) return;
		applySort(table, index, table.dataset.defaultSortDir === 'desc' ? 'desc' : 'asc');
	});

	formatLocalTimes();
	scheduleReload();
})();
