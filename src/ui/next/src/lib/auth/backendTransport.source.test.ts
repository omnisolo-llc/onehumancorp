import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join, relative } from "node:path";
import { describe, expect, it } from "vitest";

function getApiRoot(): string {
  let path = join(process.cwd(), "src/app/api");
  if (!existsSync(path)) {
    path = join(process.cwd(), "src/ui/next/src/app/api");
  }
  return path;
}

const API_ROOT = getApiRoot();
const BACKEND_CONFIGURATION =
  /process\.env\.[A-Z0-9_]*(?:URL|ORIGIN)|https?:\/\/(?:localhost|127\.0\.0\.1)(?::\d+)?/;
const BROWSER_IDENTITY =
  /headers\.get\(["'](?:authorization|cookie|x-tenant-id|x-user-id|x-spiffe-id|x-user-roles)["']\)/i;
const SHARED_TRANSPORT = /proxyBackend(?:Request|Get|Post|Patch|Put)/;

function routeFiles(directory: string): string[] {
  if (!existsSync(directory)) return [];
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
        (reason) => `${relative(process.cwd(), file)}: ${reason}`,
      );
    });

    expect(violations).toEqual([]);
  });

  it("does not reintroduce backend rewrites that bypass the server transport", () => {
    let configPath = join(process.cwd(), "next.config.mjs");
    if (!existsSync(configPath)) {
      configPath = join(process.cwd(), "src/ui/next/next.config.mjs");
    }
    const config = readFileSync(configPath, "utf8");
    expect(config).not.toMatch(/\brewrites\s*\(/);
    expect(config).not.toMatch(/destination\s*:.*BACKEND_URL/);
  });
});
