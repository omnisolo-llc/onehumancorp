import { readFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";
import { describe, expect, it } from "vitest";

const fs = require("node:fs");
let isMonorepoRoot = fs.existsSync(join(process.cwd(), "src/ui/next"));

const ROOTS = [
  join(process.cwd(), isMonorepoRoot ? "src/ui/next/src/app" : "src/app"),
  join(process.cwd(), isMonorepoRoot ? "src/ui/next/src/components" : "src/components"),
  join(process.cwd(), isMonorepoRoot ? "src/ui/next/src/hooks" : "src/hooks"),
  join(process.cwd(), isMonorepoRoot ? "src/ui/next/src/lib" : "src/lib"),
];
const SERVER_ONLY_FILES = new Set([
  join(process.cwd(), isMonorepoRoot ? "src/ui/next/src/lib/auth/backendTransport.ts" : "src/lib/auth/backendTransport.ts"),
  join(process.cwd(), isMonorepoRoot ? "src/ui/next/src/lib/auth/serverSession.ts" : "src/lib/auth/serverSession.ts"),
]);
const BROWSER_IDENTITY =
  /localStorage\s*\.\s*getItem\s*\(\s*["'](?:auth_token|ohc_token|organization_id|roles|spiffe_id|tenant|tenant_id|token|user_id)["']\s*\)/;
const BROWSER_IDENTITY_HEADER =
  /["']?(?:authorization|x-spiffe-id|x-tenant-id|x-user-id|x-user-roles)["']?\s*:/i;

function productionBrowserFiles(directory: string): string[] {
  if (!fs.existsSync(directory)) return [];
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) {
      if (path === join(process.cwd(), isMonorepoRoot ? "src/ui/next/src/app/api" : "src/app/api")) return [];
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
        (reason) => `${relative(process.cwd(), file)}: ${reason}`,
      );
    });

    expect(violations).toEqual([]);
  });
});
