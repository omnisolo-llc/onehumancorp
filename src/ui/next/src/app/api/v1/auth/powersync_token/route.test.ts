import { beforeEach, describe, expect, it, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({ ok: true })));

vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET } from "./route";

describe("GET /api/v1/auth/powersync_token", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  it("forwards the authenticated request without allowing browser query overrides", async () => {
    const request = new Request(
      "http://localhost/api/v1/auth/powersync_token?tenant_id=attacker",
    );

    await GET(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/auth/powersync_token",
      { forwardQuery: false, suppressRequestBody: true },
    );
  });
});
