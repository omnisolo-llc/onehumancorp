import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { GET } from "./route";

describe("GET /api/finance/safe-to-spend", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("forwards the request to the Rust backend", async () => {
    const backendResponse = {
      money_in: 500,
      money_out: 100,
      tax_safe: 50,
    };
    (global.fetch as any).mockResolvedValueOnce({
      status: 200,
      json: async () => backendResponse,
    });

    const request = new Request("http://localhost/api/finance/safe-to-spend");
    const response = await GET(request);
    expect(response.status).toBe(200);

    const json = await response.json();
    expect(json).toEqual(backendResponse);

    expect(global.fetch).toHaveBeenCalledWith(
      "http://backend.internal/api/v1/payments/ledger/api/finance/safe-to-spend",
      expect.objectContaining({
        method: "GET",
      })
    );
  });
});
