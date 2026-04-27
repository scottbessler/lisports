export type ReqResult =
	| { success: false; response: Response; data?: unknown; jsonError?: unknown }
	| { success: true; response: Response; data: unknown };

export async function getJSON(
	url: string,
	options?: Omit<RequestInit, 'method'>,
): Promise<ReqResult> {
	const opts = {
		...options,
		headers: { ...options?.headers },
		method: 'GET',
	};
	const start = Date.now();
	console.log('starting', url);

	const response = await fetch(url, opts);
	console.log('done', url, Date.now() - start, 'ms');
	let jsonResult: unknown;
	let jsonError: unknown;
	try {
		jsonResult = await response.json();
	} catch (err) {
		jsonError = err;
	}

	if (response.status >= 400 || jsonError) {
		return { success: false, response, data: jsonResult, jsonError };
	}
	return { success: true, response, data: jsonResult };
}

export async function getJSONWithTimeout(
	url: string,
	timeoutMs: number,
	options?: Omit<RequestInit, 'method'>,
): Promise<ReqResult> {
	const controller = new AbortController();
	const timer = setTimeout(() => controller.abort(), timeoutMs);
	try {
		return await getJSON(url, { ...options, signal: controller.signal });
	} finally {
		clearTimeout(timer);
	}
}

export function successOrThrow<TResult>(input: ReqResult): TResult {
	if (!input.success) {
		console.error(`request failed: ${input.response.status} ${input.response.statusText}`);
		throw new Error('request failed');
	}
	// TODO: validate its TResult?
	return input.data as TResult;
}

export function successOrUndefined<TResult>(input: ReqResult): TResult | undefined {
	if (!input.success) {
		console.error(`request failed: ${input.response.status} ${input.response.statusText}`);
		return undefined;
	}
	// TODO: validate its TResult?
	return input.data as TResult;
}
