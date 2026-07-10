import { GET } from "./route";
import { NextRequest } from "next/server";
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

const mockFetch = vi.fn();

describe("Help API Route", () => {
  beforeEach(() => {
    vi.stubGlobal("fetch", mockFetch);
    process.env.BACKEND_URL = "http://test-backend";
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.clearAllMocks();
  });

  it("fetches help articles from backend successfully", async () => {
    const mockData = [{ id: "1", title: "Test Article" }];
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => mockData,
    });

    const request = new NextRequest("http://localhost/api/help");
    const response = await GET(request);
    const data = await response.json();

    expect(mockFetch).toHaveBeenCalledWith("http://test-backend/api/help");
    expect(response.status).toBe(200);
    expect(data).toEqual(mockData);
  });

  it("returns 500 and error object on backend error", async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 500,
    });

    const request = new NextRequest("http://localhost/api/help");
    const response = await GET(request);
    const data = await response.json();

    expect(response.status).toBe(500);
    expect(data).toEqual({ error: "Failed to fetch help" });
  });

  it("handles fetch exceptions gracefully with 500 error", async () => {
    mockFetch.mockRejectedValueOnce(new Error("Network error"));

    const request = new NextRequest("http://localhost/api/help");
    const response = await GET(request);
    const data = await response.json();

    expect(response.status).toBe(500);
    expect(data).toEqual({ error: "Failed to fetch help" });
  });
});
