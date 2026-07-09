import { GET } from "./route";
import { NextRequest } from "next/server";
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

const mockFetch = vi.fn();

describe("API Docs Spec Route", () => {
  beforeEach(() => {
    vi.stubGlobal("fetch", mockFetch);
    process.env.BACKEND_URL = "http://test-backend";
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.clearAllMocks();
  });

  it("fetches spec from backend successfully", async () => {
    const mockData = { openapi: "3.0.0" };
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => mockData,
    });

    const request = new NextRequest("http://localhost/api/api-docs-spec");
    const response = await GET(request);
    const data = await response.json();

    expect(mockFetch).toHaveBeenCalledWith(
      "http://test-backend/api/api-docs-spec",
    );
    expect(response.status).toBe(200);
    expect(data).toEqual(mockData);
  });

  it("returns empty object on backend error", async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 500,
    });

    const request = new NextRequest("http://localhost/api/api-docs-spec");
    const response = await GET(request);
    const data = await response.json();

    expect(response.status).toBe(200);
    expect(data).toEqual({});
  });

  it("handles fetch exceptions gracefully with empty object", async () => {
    mockFetch.mockRejectedValueOnce(new Error("Network error"));

    const request = new NextRequest("http://localhost/api/api-docs-spec");
    const response = await GET(request);
    const data = await response.json();

    expect(response.status).toBe(200);
    expect(data).toEqual({});
  });
});
