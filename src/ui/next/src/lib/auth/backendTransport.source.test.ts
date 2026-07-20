import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it, test } from "vitest";

function getNextRoot() {
  const localPath = join(process.cwd(), "src/ui/next");
  if (existsSync(localPath)) {
    return localPath;
  }
  return process.cwd();
}

const API_ROOT = join(getNextRoot(), "src/app/api");
const BACKEND_CONFIGURATION =
  /process\.env\.[A-Z0-9_]*(?:URL|ORIGIN)|https?:\/\/(?:localhost|127\.0\.0\.1)(?::\d+)?/;
const BROWSER_IDENTITY =
  /\b(?:localStorage|sessionStorage|cookies)\b|\bdocument\.cookie\b/;

function routeFiles(directory: string): string[] {
  if (!existsSync(directory)) return [];
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) return routeFiles(path);
    if (entry.isFile() && /\broute\.[jt]sx?$/.test(entry.name)) return [path];
    return [];
  });
}

describe("protected backend transport source contract", () => {
  const routes = routeFiles(API_ROOT);

  it("checks at least one route file", () => {
    expect(routes.length).toBeGreaterThan(0);
  });

  test.each(routes)(
    "keeps backend origin and browser identity forwarding out of %s route handlers",
    (file) => {
      const source = readFileSync(file, "utf8");
      expect(source).not.toMatch(BACKEND_CONFIGURATION);
      expect(source).not.toMatch(BROWSER_IDENTITY);
    }
  );

  it("does not reintroduce backend rewrites that bypass the server transport", () => {
    const config = readFileSync(join(getNextRoot(), "next.config.mjs"), "utf8");
    expect(config).not.toMatch(/\brewrites\s*\(/);
    expect(config).not.toMatch(/destination\s*:.*BACKEND_URL/);
  });
});
