import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  authenticatedRequest,
  stubAuthEnvironment,
  TEST_BACKEND_ORIGIN,
} from "@/lib/auth/authTestFixtures";
import { GET } from "./route";

describe("GET /api/v1/ui/inbox/messages", () => {
  beforeEach(() => {
    stubAuthEnvironment();
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("proxies inbox messages to the Rust backend", async () => {
    const backendResponse = [{ id: "msg-1", source: "whatsapp", status: "open" }];
    vi.mocked(global.fetch).mockResolvedValueOnce(Response.json(backendResponse));

    const res = await GET(
      await authenticatedRequest("/api/v1/ui/inbox/messages?tenant_id=tenant-1"),
    );

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith(
      new URL(`${TEST_BACKEND_ORIGIN}/api/v1/ui/inbox/messages?tenant_id=tenant-7`),
      expect.objectContaining({ method: "GET" }),
    );
  });
});
