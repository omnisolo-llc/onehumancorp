import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, test } from "vitest";

const sensitivePages = [
  "src/app/knowledge/page.tsx",
  "src/app/memory/cross-session/page.tsx",
  "src/app/inbox/page.tsx",
];

describe("memory page authority source contract", () => {
  test.each(sensitivePages)("%s does not source identity from browser storage", (file) => {
    let basePath = process.cwd();
    if (!basePath.endsWith('src/ui/next')) {
        basePath = resolve(basePath, 'src/ui/next');
    }
    const source = readFileSync(resolve(basePath, file), "utf8");
    expect(source).not.toMatch(/localStorage\s*\.\s*getItem\s*\(\s*["'](?:auth_token|token|tenant_id|tenant|user_id)["']/);
    expect(source).not.toMatch(/["']Authorization["']\s*:/);
    expect(source).not.toMatch(/["']X-(?:Tenant|User)-ID["']\s*:/i);
  });
});
