import { useMatches } from '@remix-run/react';
import dayjs from 'dayjs';
import { useMemo } from 'react';

const DEFAULT_REDIRECT = '/';

/**
 * This should be used any time the redirect path is user-provided
 * (Like the query string on our login/signup pages). This avoids
 * open-redirect vulnerabilities.
 * @param {string} to The redirect destination
 * @param {string} defaultRedirect The redirect to use if the to is unsafe.
 */
export function safeRedirect(
	to: FormDataEntryValue | string | null | undefined,
	defaultRedirect: string = DEFAULT_REDIRECT,
) {
	if (!to || typeof to !== 'string') {
		return defaultRedirect;
	}

	if (!to.startsWith('/') || to.startsWith('//')) {
		return defaultRedirect;
	}

	return to;
}

export function getTodayYMD() {
	return dayjs().format('YYYY-MM-DD');
}

export const getCurrentBreakpoint = () => {
	if (document.getElementById('breakpoint-0')?.offsetParent != null) return '0';
	if (document.getElementById('breakpoint-sm')?.offsetParent != null)
		return 'sm';
	if (document.getElementById('breakpoint-md')?.offsetParent != null)
		return 'md';
	if (document.getElementById('breakpoint-lg')?.offsetParent != null)
		return 'lg';
	if (document.getElementById('breakpoint-xl')?.offsetParent != null)
		return 'xl';
	if (document.getElementById('breakpoint-2xl')?.offsetParent != null)
		return '2xl';
	return 'unknown';
};
