import { readdirSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { join, relative } from "node:path";
import { describe, expect, it } from "vitest";

const ROOTS = [
  join(process.cwd(), "src/app"),
  join(process.cwd(), "src/components"),
  join(process.cwd(), "src/hooks"),
  join(process.cwd(), "src/lib"),
];
const SERVER_ONLY_FILES = new Set([
  join(process.cwd(), "src/lib/auth/backendTransport.ts"),
  join(process.cwd(), "src/lib/auth/serverSession.ts"),
]);
const BROWSER_IDENTITY =
  /localStorage\s*\.\s*getItem\s*\(\s*["'](?:auth_token|omnisolo_token|organization_id|roles|spiffe_id|tenant|tenant_id|token|user_id)["']\s*\)/;
const BROWSER_IDENTITY_HEADER =
  /["']?(?:authorization|x-spiffe-id|x-tenant-id|x-user-id|x-user-roles)["']?\s*:/i;

function productionBrowserFiles(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) {
      if (path === join(process.cwd(), "src/app/api")) return [];
      return productionBrowserFiles(path);
    }
    if (!/\.(?:ts|tsx)$/.test(entry.name)) return [];
    if (/\.(?:test|spec)\.(?:ts|tsx)$/.test(entry.name)) return [];
    return SERVER_ONLY_FILES.has(path) ? [] : [path];
  });
}

describe("browser authentication authority", () => {
  it("keeps bearer credentials out of browser-managed storage and headers", async () => {
    const files = ROOTS.flatMap(productionBrowserFiles);
    expect(files.length).toBeGreaterThan(0);
    const violations: string[] = [];
    // Bound open files while overlapping disk reads on cold CI filesystems.
    // Every production source file still receives both security assertions.
    for (let offset = 0; offset < files.length; offset += 8) {
      const findings = await Promise.all(files.slice(offset, offset + 8).map(async file => {
        const source = await readFile(file, "utf8");
        const reasons = [
          ...(BROWSER_IDENTITY.test(source) ? ["browser-managed identity"] : []),
          ...(BROWSER_IDENTITY_HEADER.test(source) ? ["browser-generated identity header"] : []),
        ];
        return reasons.map(reason => `${relative(process.cwd(), file)}: ${reason}`);
      }));
      violations.push(...findings.flat());
    }
    expect(violations).toEqual([]);
  });
});
