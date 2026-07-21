import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  authenticatedRequest,
  stubAuthEnvironment,
  TEST_BACKEND_ORIGIN,
} from "@/lib/auth/authTestFixtures";
import { POST } from "./route";

describe("POST /api/v1/ui/inbox/messages/[messageId]/approve", () => {
  beforeEach(() => {
    stubAuthEnvironment();
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("proxies approve request to the Rust backend with message ID", async () => {
    const backendResponse = { success: true };
    vi.mocked(global.fetch).mockResolvedValueOnce(Response.json(backendResponse));

    const req = await authenticatedRequest(
      "/api/v1/ui/inbox/messages/msg-123/approve?tenant_id=tenant-1",
      { method: "POST" }
    );

    const context = { params: Promise.resolve({ messageId: "msg-123" }) };
    const res = await POST(req, context);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith(
      new URL(`${TEST_BACKEND_ORIGIN}/api/v1/ui/inbox/messages/msg-123/approve?tenant_id=tenant-7`),
      expect.objectContaining({ method: "POST" }),
    );
  });
});
