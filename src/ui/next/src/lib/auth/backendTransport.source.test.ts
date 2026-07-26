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

const API_ROOT = join(rootDir, "src/app/api");
const BACKEND_CONFIGURATION =
  /process\.env\.[A-Z0-9_]*(?:URL|ORIGIN)|https?:\/\/(?:localhost|127\.0\.0\.1)(?::\d+)?/;
const BROWSER_IDENTITY =
  /headers\.get\(["'](?:authorization|cookie|x-tenant-id|x-user-id|x-spiffe-id|x-user-roles)["']\)/i;
const SHARED_TRANSPORT = /proxyBackend(?:Request|Get|Post|Patch|Put)/;

function routeFiles(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) return routeFiles(path);
    return entry.name === "route.ts" ? [path] : [];
  });
}

describe("protected backend transport source contract", () => {
  it("keeps backend origin and browser identity forwarding out of route handlers", () => {
    const violations = routeFiles(API_ROOT).flatMap((file) => {
      const source = readFileSync(file, "utf8");
      if (SHARED_TRANSPORT.test(source)) return [];
      const reasons = [
        ...(BACKEND_CONFIGURATION.test(source) ? ["direct backend configuration"] : []),
        ...(BROWSER_IDENTITY.test(source) && /\bfetch\s*\(/.test(source)
          ? ["browser identity forwarding"]
          : []),
      ];
      return reasons.map(
        (reason) => `${relative(rootDir, file)}: ${reason}`,
      );
    });

    expect(violations).toEqual([]);
  });

  it("does not reintroduce backend rewrites that bypass the server transport", () => {
    const config = readFileSync(join(rootDir, "next.config.mjs"), "utf8");
    expect(config).not.toMatch(/\brewrites\s*\(/);
    expect(config).not.toMatch(/destination\s*:.*BACKEND_URL/);
  });
});
