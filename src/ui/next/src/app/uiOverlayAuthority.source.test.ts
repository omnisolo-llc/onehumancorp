import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";

function source(relativePath: string): string {
  const basePath = process.cwd().endsWith("src/ui/next") ? process.cwd() : join(process.cwd(), "src/ui/next");
  return readFileSync(join(basePath, relativePath), "utf8");
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
