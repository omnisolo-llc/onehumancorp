import { readFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

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

function source(relativePath: string): string {
  return readFileSync(join(rootDir, relativePath), "utf8");
}

describe("product-shell overlay authority", () => {
  it("mounts one global help and chat launcher", () => {
    const layout = source("src/app/layout.tsx");
    expect(layout.match(/<HelpWidget\b/g)).toHaveLength(1);
    expect(layout.match(/<HelpChat\b/g)).toHaveLength(1);
  });

  it("does not load API documentation assets on every page", () => {
    const layout = source("src/app/layout.tsx");
    expect(layout).not.toMatch(/swagger-ui/);
  });

  it("does not duplicate header and limit actions as dashboard overlays", () => {
    const dashboard = source("src/app/dashboard/page.tsx");
    expect(dashboard).not.toMatch(/<FloatingActionButton\b/);
    expect(dashboard).not.toMatch(/<AIPaywallWidget\b/);
  });

  it("uses the app-shell voice control without a dashboard duplicate", () => {
    const dashboard = source("src/app/dashboard/page.tsx");
    expect(dashboard).not.toMatch(/<VoiceAssistantFAB\b/);
  });

  it("keeps immersive pages inside the product shell", () => {
    const wrapped = source("src/app/wrapped/page.tsx");
    expect(wrapped).not.toMatch(/className="fixed inset-0/);
  });
});
