import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join, relative } from "node:path";
import { describe, expect, it } from "vitest";

function findFolder(relativePath: string): string {
  const paths = [
    join(process.cwd(), relativePath),
    join(process.cwd(), "src/ui/next", relativePath),
    join(__dirname, "../../../..", relativePath),
    join(__dirname, "../../..", relativePath),
  ];
  for (const p of paths) {
    if (existsSync(p)) {
      return p;
    }
  }
  throw new Error(`Could not find folder: ${relativePath}`);
}

const ROOTS = [
  findFolder("src/app"),
  findFolder("src/components"),
  findFolder("src/hooks"),
  findFolder("src/lib"),
];
const SERVER_ONLY_FILES = new Set([
  join(findFolder("src/lib"), "auth/backendTransport.ts"),
  join(findFolder("src/lib"), "auth/serverSession.ts"),
]);
const BROWSER_IDENTITY =
  /localStorage\s*\.\s*getItem\s*\(\s*["'](?:auth_token|ohc_token|organization_id|roles|spiffe_id|tenant|tenant_id|token|user_id)["']\s*\)/;
const BROWSER_IDENTITY_HEADER =
  /["']?(?:authorization|x-spiffe-id|x-tenant-id|x-user-id|x-user-roles)["']?\s*:/i;

function productionBrowserFiles(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) {
      if (path === join(findFolder("src/app"), "api")) return [];
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
