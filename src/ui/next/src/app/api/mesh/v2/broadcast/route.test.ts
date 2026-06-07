import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { POST } from "./route";

describe("POST /api/mesh/v2/broadcast", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("forwards validated mesh broadcasts to the Rust backend transport", async () => {
    const backendResponse = { success: true };
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => backendResponse,
    });

    const body = {
      agent_id: "agent-1",
      channel: "ops",
      event_type: "task.created",
      data: { payload: "real" },
    };
    const req = new Request("http://localhost/api/mesh/v2/broadcast", {
      method: "POST",
      headers: {
        authorization: "Bearer token",
        "x-spiffe-id": "spiffe://ohc/org/example.org/agent/agent-1",
        "x-tenant-id": "tenant-1",
        "x-user-id": "user-1",
      },
      body: JSON.stringify(body),
    });

    const res = await POST(req as any);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/mesh/v2/broadcast", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        authorization: "Bearer token",
        "x-spiffe-id": "spiffe://ohc/org/example.org/agent/agent-1",
        "x-tenant-id": "tenant-1",
        "x-user-id": "user-1",
      },
      body: JSON.stringify(body),
    });
  });
});
