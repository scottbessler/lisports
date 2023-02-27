import { useMatches } from "@remix-run/react";
import dayjs from "dayjs";
import { useMemo } from "react";

const DEFAULT_REDIRECT = "/";

/**
 * This should be used any time the redirect path is user-provided
 * (Like the query string on our login/signup pages). This avoids
 * open-redirect vulnerabilities.
 * @param {string} to The redirect destination
 * @param {string} defaultRedirect The redirect to use if the to is unsafe.
 */
export function safeRedirect(
  to: FormDataEntryValue | string | null | undefined,
  defaultRedirect: string = DEFAULT_REDIRECT
) {
  if (!to || typeof to !== "string") {
    return defaultRedirect;
  }

  if (!to.startsWith("/") || to.startsWith("//")) {
    return defaultRedirect;
  }

  return to;
}

/**
 * This base hook is used in other hooks to quickly search for specific data
 * across all loader data using useMatches.
 * @param {string} id The route id
 * @returns {JSON|undefined} The router data or undefined if not found
 */
export function useMatchesData(
  id: string
): Record<string, unknown> | undefined {
  const matchingRoutes = useMatches();
  const route = useMemo(
    () => matchingRoutes.find((route) => route.id === id),
    [matchingRoutes, id]
  );
  return route?.data;
}

export function getTodayYMD() {
  return dayjs().format("YYYY-MM-DD");
}

export const getCurrentBreakpoint = () => {
  if (document.getElementById("breakpoint-0")?.offsetParent != null) return "0";
  if (document.getElementById("breakpoint-sm")?.offsetParent != null)
    return "sm";
  if (document.getElementById("breakpoint-md")?.offsetParent != null)
    return "md";
  if (document.getElementById("breakpoint-lg")?.offsetParent != null)
    return "lg";
  if (document.getElementById("breakpoint-xl")?.offsetParent != null)
    return "xl";
  if (document.getElementById("breakpoint-2xl")?.offsetParent != null)
    return "2xl";
  return "unknown";
};
