import { GET } from "./route";
import { NextRequest } from "next/server";
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

const mockFetch = vi.fn();

describe("Help Article API Route", () => {
  beforeEach(() => {
    vi.stubGlobal("fetch", mockFetch);
    process.env.BACKEND_URL = "http://test-backend";
    // Silence expected console errors
    vi.spyOn(console, "error").mockImplementation(() => {});
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.restoreAllMocks();
  });

  it("fetches article from backend successfully", async () => {
    const mockData = { id: "test-id", title: "Test Article" };
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => mockData,
    });

    const request = new NextRequest("http://localhost/api/help/test-id");
    const response = await GET(request, {
      params: Promise.resolve({ articleId: "test-id" }),
    });
    const data = await response.json();

    expect(mockFetch).toHaveBeenCalledWith(
      "http://test-backend/api/help/test-id",
    );
    expect(response.status).toBe(200);
    expect(data).toEqual(mockData);
  });

  it("returns 404 on backend 404 error", async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 404,
    });

    const request = new NextRequest("http://localhost/api/help/unknown-id");
    const response = await GET(request, {
      params: Promise.resolve({ articleId: "unknown-id" }),
    });
    const data = await response.json();

    expect(response.status).toBe(404);
    expect(data.error).toBe("Article not found");
  });

  it("handles fetch exceptions gracefully with 404 error", async () => {
    mockFetch.mockRejectedValueOnce(new Error("Network error"));

    const request = new NextRequest("http://localhost/api/help/error-id");
    const response = await GET(request, {
      params: Promise.resolve({ articleId: "error-id" }),
    });
    const data = await response.json();

    expect(response.status).toBe(404);
    expect(data.error).toBe("Article not found");
  });
});
