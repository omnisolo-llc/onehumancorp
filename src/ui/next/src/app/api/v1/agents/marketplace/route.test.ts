import { describe, expect, it } from "vitest";
import { NextRequest } from "next/server";
import { GET } from "./route";

describe("Agent marketplace route", () => {
  it("returns 503 when query is sales", async () => {
    const req = new NextRequest("http://localhost/api/v1/agents/marketplace?q=sales");
    const res = await GET(req);
    expect(res.status).toBe(503);
    const data = await res.json();
    expect(data).toEqual({
      error: "Marketplace service temporarily unavailable",
    });
  });

  it("returns 503 when query param 'query' is sales", async () => {
    const req = new NextRequest("http://localhost/api/v1/agents/marketplace?query=sales");
    const res = await GET(req);
    expect(res.status).toBe(503);
    const data = await res.json();
    expect(data).toEqual({
      error: "Marketplace service temporarily unavailable",
    });
  });

  it("returns mock agent list when query is not sales", async () => {
    const req = new NextRequest("http://localhost/api/v1/agents/marketplace?q=rust");
    const res = await GET(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(Array.isArray(data)).toBe(true);
    expect(data.some((a: { name: string }) => a.name.includes("Rust"))).toBe(true);
  });
});
