import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join, relative, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const testDir = typeof __dirname !== "undefined" ? __dirname : dirname(fileURLToPath(import.meta.url));

function findNextProjectRoot() {
  let current = testDir;
  while (current && current !== "/") {
    if (existsSync(join(current, "next.config.mjs")) && existsSync(join(current, "src/app"))) {
      return current;
    }
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  const fallback = process.cwd();
  if (existsSync(join(fallback, "src/ui/next/next.config.mjs"))) {
    return join(fallback, "src/ui/next");
  }
  return fallback;
}

const rootDir = findNextProjectRoot();

const ROOTS = [
  join(rootDir, "src/app"),
  join(rootDir, "src/components"),
  join(rootDir, "src/hooks"),
  join(rootDir, "src/lib"),
];
const SERVER_ONLY_FILES = new Set([
  join(rootDir, "src/lib/auth/backendTransport.ts"),
  join(rootDir, "src/lib/auth/serverSession.ts"),
]);
const BROWSER_IDENTITY =
  /localStorage\s*\.\s*getItem\s*\(\s*["'](?:auth_token|ohc_token|organization_id|roles|spiffe_id|tenant|tenant_id|token|user_id)["']\s*\)/;
const BROWSER_IDENTITY_HEADER =
  /["']?(?:authorization|x-spiffe-id|x-tenant-id|x-user-id|x-user-roles)["']?\s*:/i;

function productionBrowserFiles(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) {
      if (path === join(rootDir, "src/app/api")) return [];
      return productionBrowserFiles(path);
    }
    if (!/\.(?:ts|tsx)$/.test(entry.name)) return [];
    if (/\.(?:test|spec)\.(?:ts|tsx)$/.test(entry.name)) return [];
    return SERVER_ONLY_FILES.has(path) ? [] : [path];
  });
}

describe("browser authentication authority", () => {
  it("keeps bearer credentials out of browser-managed storage and headers", () => {
    const violations = ROOTS.flatMap(productionBrowserFiles).flatMap((file) => {
      const source = readFileSync(file, "utf8");
      const reasons = [
        ...(BROWSER_IDENTITY.test(source) ? ["browser-managed identity"] : []),
        ...(BROWSER_IDENTITY_HEADER.test(source)
          ? ["browser-generated identity header"]
          : []),
      ];
      return reasons.map(
        (reason) => `${relative(rootDir, file)}: ${reason}`,
      );
    });

    expect(violations).toEqual([]);
  });
});
