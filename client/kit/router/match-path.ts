import { normalizePathname } from "./normalize-pathname.ts";

/**
 * @description
 * Will match a url declaration with an incoming pathname
 *   /users/:id/edit
 *   /users/5345/edit
 */
export const matchPath = (
  baseHref: string,
  pattern: string,
  pathname: string,
): Record<string, string> | undefined => {
  const [normalizedBase, normalizedPattern] = normalizePathname(
    baseHref,
    pattern,
  );
  let strippedPath = pathname.substring(normalizedBase.length);
  if (!strippedPath.startsWith("/")) {
    strippedPath = `/${strippedPath}`;
  }

  // No args
  if (strippedPath === normalizedPattern) {
    return {};
  }

  const params: Record<string, string> = {};
  const source = normalizedPattern.split("/");
  const test = strippedPath.split("/");

  if (source.length !== test.length && !normalizedPattern.includes("**")) {
    return;
  }

  for (const i in source) {
    if (source[i].startsWith(":") && strippedPath !== "/") {
      const paramName = source[i].slice(1);
      params[paramName] = test[i].toString();
      continue;
    }
    if (source[i].startsWith("**")) {
      return params;
    }
    if (source[i] !== test[i].toLowerCase()) {
      return;
    }
  }

  return params;
};
