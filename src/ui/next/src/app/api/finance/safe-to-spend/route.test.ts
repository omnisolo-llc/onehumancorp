import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  authenticatedRequest,
  stubAuthEnvironment,
  TEST_BACKEND_ORIGIN,
} from "@/lib/auth/authTestFixtures";
import { GET } from "./route";

describe("GET /api/finance/safe-to-spend", () => {
  beforeEach(() => {
    stubAuthEnvironment();
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
    vi.mocked(global.fetch).mockResolvedValueOnce(Response.json(backendResponse));

    const request = await authenticatedRequest("/api/finance/safe-to-spend");
    const response = await GET(request);
    expect(response.status).toBe(200);

    const json = await response.json();
    expect(json).toEqual(backendResponse);

    expect(global.fetch).toHaveBeenCalledWith(
      new URL(
        `${TEST_BACKEND_ORIGIN}/api/v1/payments/ledger/api/finance/safe-to-spend`,
      ),
      expect.objectContaining({
        method: "GET",
      }),
    );
  });
});
