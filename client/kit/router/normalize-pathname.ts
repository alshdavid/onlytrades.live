export function normalizePathname(
  baseHref: string,
  path: string,
): [string, string, string] {
  // Normalize baseHref: remove trailing slash (unless it's just "/"), lowercase, default empty to "/"
  let normalizedBase: string;
  if (!baseHref || baseHref === "/") {
    normalizedBase = "/";
  } else {
    normalizedBase = baseHref.replace(/\/$/, "").toLowerCase();
  }

  // Normalize path: lowercase
  const normalizedPath = path.toLowerCase();

  // Calculate full path
  let fullPath: string;
  if (normalizedPath === "/") {
    // If path is just "/", fullPath is the base
    fullPath = normalizedBase;
  } else {
    // Combine base and path
    fullPath =
      normalizedBase === "/" ? normalizedPath : normalizedBase + normalizedPath;
  }

  return [normalizedBase, normalizedPath, fullPath];
}
