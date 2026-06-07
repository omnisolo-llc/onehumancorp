import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { GET } from "./route";

describe("GET /api/ui/inbox/messages", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("proxies inbox messages to the Rust backend", async () => {
    const backendResponse = [{ id: "msg-1", source: "whatsapp", status: "open" }];
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: async () => backendResponse,
    });

    const res = await GET(new Request("http://localhost/api/ui/inbox/messages?tenant_id=tenant-1"));

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith(
      "http://backend.internal/api/ui/inbox/messages?tenant_id=tenant-1",
      expect.objectContaining({ method: "GET" }),
    );
  });
});
