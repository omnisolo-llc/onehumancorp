import { readdir } from "node:fs/promises";
import path from "node:path";

const SAMPLE_SEGMENTS = new Map([
  ["articleId", "visual-audit-article"],
  ["id", "visual-audit-id"],
  ["tenant", "visual-audit-business"],
]);

export function routeFromPageFile(relativeFile) {
  const normalized = relativeFile.split(path.sep).join("/");
  if (!normalized.endsWith("/page.tsx") && normalized !== "page.tsx") {
    throw new Error(`not a page file: ${relativeFile}`);
  }
  const directory = normalized === "page.tsx"
    ? ""
    : normalized.slice(0, -"/page.tsx".length);
  const segments = directory === "" ? [] : directory.split("/");
  const resolved = segments.map((segment) => {
    const match = /^\[([^\]]+)\]$/.exec(segment);
    if (match) return SAMPLE_SEGMENTS.get(match[1]) ?? "visual-audit-value";
    const catchAll = /^\[\.\.\.(.+)\]$/.exec(segment);
    if (catchAll) return SAMPLE_SEGMENTS.get(catchAll[1]) ?? "visual-audit-value";
    return segment;
  });
  return `/${resolved.join("/")}`;
}

async function pageFiles(root, directory = root) {
  const entries = await readdir(directory, { withFileTypes: true });
  const nested = await Promise.all(entries.map(async (entry) => {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) return pageFiles(root, file);
    return entry.name === "page.tsx" ? [path.relative(root, file)] : [];
  }));
  return nested.flat();
}

export async function discoverPageRoutes(appRoot) {
  const routes = (await pageFiles(appRoot)).map(routeFromPageFile);
  return [...new Set(routes)].sort();
}

export function shardAuditCases(cases, total, index) {
  if (!Number.isSafeInteger(total) || total < 1 ||
      !Number.isSafeInteger(index) || index < 0 || index >= total) {
    throw new Error("invalid audit shard");
  }
  return cases.filter((_item, caseIndex) => caseIndex % total === index);
}
