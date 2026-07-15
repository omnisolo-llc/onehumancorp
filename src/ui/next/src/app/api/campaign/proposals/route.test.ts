import { beforeEach, describe, expect, it, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() =>
  vi.fn(async () => Response.json({ proposals: [] })),
);
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET } from "./route";

describe("GET /api/campaign/proposals", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  it("uses claims-backed identity without forwarding arbitrary inbound queries", async () => {
    const request = new Request(
      "http://localhost/api/campaign/proposals?tenant_id=attacker&limit=9999",
    );

    await GET(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/proposals/social/list",
      {
        forwardQuery: false,
        requestContentType: "application/json",
      },
    );
  });
});
