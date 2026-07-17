import { readFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";
import { describe, expect, it } from "vitest";

const API_ROOT = join(__dirname, "../../app/api");
const BACKEND_CONFIGURATION =
  /process\.env\.(?:BACKEND_URL|OHC_API_URL|NEXT_PUBLIC_API_URL|API_URL)/;
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
        (reason) => `${relative(process.cwd(), file)}: ${reason}`,
      );
    });

    // This exact inventory must shrink with every migration group. The final
    // authentication gate replaces the snapshot with an empty-array assertion.
    expect(violations.length).toBe(63);
  });

  it("does not reintroduce backend rewrites that bypass the server transport", () => {
    const config = readFileSync(join(__dirname, "../../../next.config.mjs"), "utf8");
    expect(config).not.toMatch(/\brewrites\s*\(/);
    expect(config).not.toMatch(/destination\s*:.*BACKEND_URL/);
  });
});
