import path from "node:path";
import { expect, test } from "vitest";
import {
  discoverPageRoutes,
  routeFromPageFile,
  shardAuditCases,
} from "./visual-audit-routes.mjs";

test("converts static and dynamic page files into deterministic routes", () => {
  expect(routeFromPageFile("page.tsx")).toBe("/");
  expect(routeFromPageFile("dashboard/page.tsx")).toBe("/dashboard");
  expect(routeFromPageFile("bio/[tenant]/page.tsx")).toBe("/bio/visual-audit-business");
  expect(routeFromPageFile("help/[articleId]/page.tsx")).toBe("/help/visual-audit-article");
});

test("shards cases deterministically without gaps or overlap", () => {
  const cases = Array.from({ length: 10 }, (_, index) => ({ index }));
  const shards = Array.from({ length: 4 }, (_, index) => shardAuditCases(cases, 4, index));
  expect(shards.flat().map((item) => item.index).sort((a, b) => a - b))
    .toEqual(cases.map((item) => item.index));
  expect(new Set(shards.flat()).size).toBe(cases.length);
  expect(() => shardAuditCases(cases, 0, 0)).toThrow(/invalid audit shard/);
  expect(() => shardAuditCases(cases, 4, 4)).toThrow(/invalid audit shard/);
});

test("discovers the complete application page inventory", async () => {
  const routes = await discoverPageRoutes(path.resolve(import.meta.dirname, "../src/app"));
  expect(routes.length, `expected at least 195 routes, received ${routes.length}`).toBeGreaterThanOrEqual(195);
  expect(routes).toContain("/");
  expect(routes).toContain("/dashboard");
  expect(routes).toContain("/orders/visual-audit-id");
  expect(routes.length).toBe(new Set(routes).size);
});
