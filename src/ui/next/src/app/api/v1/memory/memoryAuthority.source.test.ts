import { readFileSync, existsSync } from "node:fs";
import { resolve, join } from "node:path";
import { describe, expect, test } from "vitest";

function getNextRoot() {
  const localPath = join(process.cwd(), "src/ui/next");
  if (existsSync(localPath)) {
    return localPath;
  }
  return process.cwd();
}

const sensitivePages = [
  "src/app/knowledge/page.tsx",
  "src/app/memory/cross-session/page.tsx",
  "src/app/inbox/page.tsx",
];

describe("memory page authority source contract", () => {
  test.each(sensitivePages)("%s does not source identity from browser storage", (file) => {
    const source = readFileSync(resolve(getNextRoot(), file), "utf8");
    expect(source).not.toMatch(/localStorage\s*\.\s*getItem\s*\(\s*["']/);
    expect(source).not.toMatch(/["']Authorization["']\s*:/);
  });
});
