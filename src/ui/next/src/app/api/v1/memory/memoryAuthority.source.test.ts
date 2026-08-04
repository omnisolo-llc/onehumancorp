import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, test } from "vitest";

const sensitivePages = [
  "src/ui/next/src/app/knowledge/page.tsx",
  "src/ui/next/src/app/memory/cross-session/page.tsx",
  "src/ui/next/src/app/inbox/page.tsx",
];

describe("memory page authority source contract", () => {
  test.each(sensitivePages)("%s does not source identity from browser storage", (file) => {
    const source = readFileSync(resolve(process.cwd(), file), "utf8");
    expect(source).not.toMatch(/localStorage\s*\.\s*getItem\s*\(\s*["'](?:auth_token|token|tenant_id|tenant|user_id)["']/);
    expect(source).not.toMatch(/["']Authorization["']\s*:/);
    expect(source).not.toMatch(/["']X-(?:Tenant|User)-ID["']\s*:/i);
  });
});
