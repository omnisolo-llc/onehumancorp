import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, test } from "vitest";

// Miser: cost-optimized memory authority source code path contract verification
const sensitivePages = [
  "src/app/knowledge/page.tsx",
  "src/app/memory/cross-session/page.tsx",
  "src/app/inbox/page.tsx",
];

describe("memory page authority source contract", () => {
  test.each(sensitivePages)("%s does not source identity from browser storage", (file) => {
    // Resolve relative to __dirname of this test file (src/ui/next/src/app/api/v1/memory)
    // Go up 4 levels to get to src/ui/next/src/app
    const absoluteFilePath = resolve(__dirname, "..", "..", "..", "..", file.replace("src/", ""));
    const source = readFileSync(absoluteFilePath, "utf8");
    expect(source).not.toMatch(/localStorage\s*\.\s*getItem\s*\(\s*["'](?:auth_token|token|tenant_id|tenant|user_id)["']/);
    expect(source).not.toMatch(/["']Authorization["']\s*:/);
    expect(source).not.toMatch(/["']X-(?:Tenant|User)-ID["']\s*:/i);
  });
});
