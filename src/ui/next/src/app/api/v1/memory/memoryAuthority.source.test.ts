import { readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, test } from "vitest";

const sensitivePages = [
  "src/app/knowledge/page.tsx",
  "src/app/memory/cross-session/page.tsx",
  "src/app/inbox/page.tsx",
];

function findFile(relativePath: string): string {
  const paths = [
    resolve(process.cwd(), relativePath),
    resolve(process.cwd(), "src/ui/next", relativePath),
    resolve(__dirname, "../../../..", relativePath),
    resolve(__dirname, "../../..", relativePath),
    resolve(__dirname, "../..", relativePath),
    resolve(__dirname, "..", relativePath),
    resolve(__dirname, relativePath),
  ];
  for (const p of paths) {
    if (existsSync(p)) {
      return readFileSync(p, "utf8");
    }
  }
  throw new Error(`Could not find file: ${relativePath}`);
}

describe("memory page authority source contract", () => {
  test.each(sensitivePages)("%s does not source identity from browser storage", (file) => {
    const source = findFile(file);
    expect(source).not.toMatch(/localStorage\s*\.\s*getItem\s*\(\s*["'](?:auth_token|token|tenant_id|tenant|user_id)["']/);
    expect(source).not.toMatch(/["']Authorization["']\s*:/);
    expect(source).not.toMatch(/["']X-(?:Tenant|User)-ID["']\s*:/i);
  });
});
