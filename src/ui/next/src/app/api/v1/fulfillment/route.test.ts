import { beforeEach, expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() =>
  vi.fn(async () => Response.json({ to_pack: [], awaiting_pickup: [] })),
);

vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET } from "./route";

beforeEach(() => proxyBackendRequest.mockClear());

test("delegates GET without forwarding inbound queries", async () => {
  const request = new Request(
    "http://localhost/api/fulfillment?status=preparing&tenant_id=untrusted",
  );

  const response = await GET(request);

  expect(response.status).toBe(200);
  expect(proxyBackendRequest).toHaveBeenCalledWith(
    request,
    "/api/v1/fulfillment",
    { forwardQuery: false },
  );
});
