import { beforeEach, describe, expect, it, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() =>
  vi.fn(async () => Response.json({ success: true })),
);

vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST } from "./route";

const context = (id: string) => ({ params: Promise.resolve({ id }) });

describe("POST /api/fulfillment/execute/[id]", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  it("delegates the unchanged POST body and query to a confined fulfillment path", async () => {
    const body = JSON.stringify({ action: "mark_ready" });
    const request = new Request(
      "http://localhost/api/fulfillment/execute/ord-42?notify=true",
      { method: "POST", headers: { "content-type": "application/json" }, body },
    );

    const response = await POST(request, context("ord-42"));

    expect(response.status).toBe(200);
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/fulfillment/execute/ord-42",
    );
    expect(request.method).toBe("POST");
    expect(new URL(request.url).search).toBe("?notify=true");
    await expect(request.text()).resolves.toBe(body);
  });

  it.each([".", "..", "../admin", "order/next", "", "a".repeat(129)])(
    "rejects invalid fulfillment ID %j before proxying",
    async (id) => {
      const request = new Request(
        "http://localhost/api/fulfillment/execute/invalid",
        { method: "POST", body: JSON.stringify({ action: "mark_ready" }) },
      );

      const response = await POST(request, context(id));

      expect(response.status).toBe(400);
      expect(response.headers.get("cache-control")).toBe("private, no-store");
      expect(response.headers.get("pragma")).toBe("no-cache");
      expect(response.headers.get("x-content-type-options")).toBe("nosniff");
      await expect(response.json()).resolves.toEqual({
        error: "invalid fulfillment ID",
      });
      expect(proxyBackendRequest).not.toHaveBeenCalled();
    },
  );
});
