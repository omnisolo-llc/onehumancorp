import { describe, expect, it } from "vitest";
import { NextRequest } from "next/server";
import { PUT } from "./route";

describe("Quotes [id] route", () => {
  it("updates quote and defaults status to SENT", async () => {
    const req = new NextRequest("http://localhost/api/v1/quotes/quote-1", {
      method: "PUT",
      body: JSON.stringify({ amount: 500, title: "Test Quote" }),
    });
    const res = await PUT(req, { params: { id: "quote-1" } });
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data).toEqual({
      id: "quote-1",
      amount: 500,
      title: "Test Quote",
      status: "SENT",
    });
  });

  it("preserves explicit status when provided", async () => {
    const req = new NextRequest("http://localhost/api/v1/quotes/quote-2", {
      method: "PUT",
      body: JSON.stringify({ amount: 1000, status: "ACCEPTED" }),
    });
    const res = await PUT(req, { params: { id: "quote-2" } });
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data).toEqual({
      id: "quote-2",
      amount: 1000,
      status: "ACCEPTED",
    });
  });
});
