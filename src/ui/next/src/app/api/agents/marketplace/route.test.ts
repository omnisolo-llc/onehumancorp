import { describe, it, expect } from "vitest";
import { GET } from "./route";

describe("GET /api/agents/marketplace", () => {
  it("returns all agents when no query is provided", async () => {
    const req = new Request("http://localhost:3000/api/agents/marketplace");
    const res = await GET(req);
    const json = await res.json();

    expect(res.status).toBe(200);
    expect(json.length).toBeGreaterThan(0);
    expect(json[0].name).toBe("Data Analyst");
  });

  it("filters agents based on the query", async () => {
    const req = new Request("http://localhost:3000/api/agents/marketplace?q=SEO");
    const res = await GET(req);
    const json = await res.json();

    expect(res.status).toBe(200);
    expect(json.length).toBe(1);
    expect(json[0].name).toBe("SEO Specialist");
  });

  it("returns empty array when query does not match", async () => {
    const req = new Request("http://localhost:3000/api/agents/marketplace?q=NonExistentAgent");
    const res = await GET(req);
    const json = await res.json();

    expect(res.status).toBe(200);
    expect(json.length).toBe(0);
  });
});
