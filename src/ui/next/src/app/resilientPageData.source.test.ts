import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";

function page(relativePath: string): string {
  return readFileSync(join(process.cwd(), "src/ui/next/src/app", relativePath), "utf8");
}

describe("page rendering resilience", () => {
  it("does not read browser globals during subscription server rendering", () => {
    expect(page("subscriptions/manage/page.tsx")).not.toContain("window.location.search");
  });

  it("normalizes failed collection responses before rendering lists", () => {
    expect(page("agent-debug-trace/page.tsx"))
      .toContain("setEvents(Array.isArray(data) ? data : [])");
    const kds = page("pos/kds/page.tsx");
    expect(kds).toContain("setOrders(normalizedOrders)");
    expect(kds).toContain("setInventory(normalizedInventory)");
    expect(page("changelog/page.tsx"))
      .toContain("setSections(Array.isArray(data) ? data : [])");
  });

  it("uses a stable initial origin for share-link hydration", () => {
    const generator = page("share-to-unlock-generator/page.tsx");
    expect(generator).not.toContain("typeof window !== 'undefined' ? window.location.origin");
    expect(generator).toContain("setOrigin(window.location.origin)");

    const storeWrap = page("store-wrap/page.tsx");
    expect(storeWrap).not.toContain("typeof window !== 'undefined' ? window.location.origin");
    expect(storeWrap).toContain("setOrigin(window.location.origin)");
  });

  it("does not request a QR code for an empty target URL", () => {
    expect(page("qr-code-generator/page.tsx"))
      .toContain("useState('https://ohc.app/my-store')");
  });

  it("keeps handled optional-help failures out of the error console", () => {
    expect(page("help/page.tsx")).not.toContain("console.error(err)");
    expect(readFileSync(join(process.cwd(), "src/ui/next/src/components/TooltipRegistry.tsx"), "utf8"))
      .not.toContain("console.error('Failed to load tooltips'");
  });

  it("uses App Router metadata primitives and deterministic render values", () => {
    expect(page("crewai/page.tsx")).not.toContain("next/head");
    expect(readFileSync(join(process.cwd(), "src/ui/next/src/components/VoiceAssistant.tsx"), "utf8"))
      .not.toContain("Math.random()");

    const countdown = page("viral-countdown-widget/page.tsx");
    expect(countdown).toContain("useState('')");
    expect(countdown).not.toContain("useState(() => {");
  });

  it("does not log handled initial-data fallback failures as application errors", () => {
    for (const relativePath of [
      "agent-debug-trace/page.tsx",
      "ai-usage-paywall/page.tsx",
      "api-docs/page.tsx",
      "changelog/page.tsx",
      "cost-dashboard/page.tsx",
      "pos/kds/page.tsx",
      "products/page.tsx",
    ]) {
      expect(page(relativePath), relativePath).not.toMatch(/\.catch\(console\.error\)|console\.error\((?:err|"Failed to fetch (?:cost|plan) data|"Error fetching cost data|'Failed to fetch (?:orders|inventory))/);
    }
  });

  it("allows generator flex panels to shrink around long embed code", () => {
    for (const route of [
      "viral-challenge-generator",
      "viral-goal-tracker",
      "viral-powered-by-ohc-widget",
      "interactive-insight-widget",
      "viral-countdown-widget",
      "viral-leaderboard-generator",
    ]) {
      const source = page(`${route}/page.tsx`);
      expect(source.match(/flex-1 min-w-0/g), route).toHaveLength(2);
    }
  });

  it("gives immersive shell pages an explicit viewport-height canvas", () => {
    expect(page("store-wrap/page.tsx"))
      .toContain("h-[calc(100vh-7rem)]");
    expect(page("wrapped/page.tsx"))
      .toContain("h-[calc(100vh-7rem)]");
  });

  it("uses a responsive team canvas instead of forcing phone width on desktop", () => {
    const team = page("team/page.tsx");
    expect(team).not.toContain('w-[375px] max-w-[375px]');
    expect(team).toContain('lg:max-w-6xl');
    expect(team).toContain('lg:grid-cols-2');
  });
});
