import Proxifly from 'proxifly';

export type ReqResult =
	| { success: false; response: Response; data?: unknown; jsonError?: unknown }
	| { success: true; response: Response; data: unknown };

async function initProxy() {
	const options = {
		protocol: 'http', // http | socks4 | socks5
		anonymity: 'anonymous', // transparent | anonymous | elite
		country: 'US', // https://www.nationsonline.org/oneworld/country_code_list.htm
		https: true, // true | false
		speed: 10000, // 0 - 60000
		quantity: 1, // 1 - 20
	};

	const proxifly = new Proxifly();

	console.log({ result: await proxifly.getProxy() });
}

// const jsonMime = "application/json";
export async function getJSON(
	url: string,
	options?: Omit<RequestInit, 'method'>,
): Promise<ReqResult> {
	const proxyInfo = {
		proxy: 'http://185.238.228.243:80',
		protocol: 'http',
		ip: '185.238.228.243',
		port: 80,
		https: false,
		anonymity: 'elite',
		score: 1,
		geolocation: { country: 'ES', city: '' },
	};
	const headers = new Headers({ ...options?.headers });

	const opts = {
		...options,
		headers: { ...options?.headers },
		method: 'GET',
		proxy: 'http://185.238.228.243:80',
	};
	const start = Date.now();
	console.log('starting', url, headers, opts);

	const response = await fetch(url, opts);
	console.log('done', url, headers, opts, Date.now() - start);
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

export function successOrThrow<TResult>(input: ReqResult): TResult {
	if (!input.success) {
		console.error(
			`request failed: ${input.response.status} ${input.response.statusText}`,
		);
		throw new Error('request failed');
	}
	// TODO: validate its TResult?
	return input.data as TResult;
}

export function successOrUndefined<TResult>(
	input: ReqResult,
): TResult | undefined {
	if (!input.success) {
		console.error(
			`request failed: ${input.response.status} ${input.response.statusText}`,
		);
		return undefined;
	}
	// TODO: validate its TResult?
	return input.data as TResult;
}
