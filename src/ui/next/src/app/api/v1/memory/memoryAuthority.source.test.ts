import { readFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

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

const sensitivePages = [
  "src/app/knowledge/page.tsx",
  "src/app/memory/cross-session/page.tsx",
  "src/app/inbox/page.tsx",
];

describe("memory page authority source contract", () => {
  test.each(sensitivePages)("%s does not source identity from browser storage", (file) => {
    const source = readFileSync(join(rootDir, file), "utf8");
    expect(source).not.toMatch(/localStorage\s*\.\s*getItem\s*\(\s*["'](?:auth_token|token|tenant_id|tenant|user_id)["']/);
    expect(source).not.toMatch(/["']Authorization["']\s*:/);
    expect(source).not.toMatch(/["']X-(?:Tenant|User)-ID["']\s*:/i);
  });
});
