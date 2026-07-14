import { beforeEach, describe, expect, test, vi } from "vitest";
import type { BackendRequestOptions } from "@/lib/auth/backendTransport";

const { normalizeJsonRequestBody, proxyBackendRequest } = vi.hoisted(() => ({
  normalizeJsonRequestBody: vi.fn((body: Uint8Array<ArrayBuffer>) => body),
  proxyBackendRequest: vi.fn<
    (request: Request, path: string, options?: BackendRequestOptions) => Promise<Response>
  >(async () => Response.json({ plans: [] })),
}));
vi.mock("@/lib/auth/backendTransport", () => ({
  normalizeJsonRequestBody,
  proxyBackendRequest,
}));

import { POST as action } from "./[id]/action/route";
import { GET as getById } from "./[id]/route";
import { GET } from "./route";

const context = (id: string) => ({ params: Promise.resolve({ id }) });

describe("authenticated subscription routes", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  test("preserves fixed legacy list and detail paths without inbound queries", async () => {
    const listRequest = new Request(
      "http://localhost/api/subscriptions?tenant_id=attacker",
    );
    const detailRequest = new Request(
      "http://localhost/api/subscriptions/sub-7?expand=attacker",
    );

    await GET(listRequest);
    await getById(detailRequest, context("sub-7"));

    expect(proxyBackendRequest.mock.calls).toEqual([
      [listRequest, "/api/subscriptions", { forwardQuery: false }],
      [detailRequest, "/api/subscriptions/sub-7", { forwardQuery: false }],
    ]);
  });

  test("preserves action JSON normalization without inbound queries", async () => {
    const request = new Request(
      "http://localhost/api/subscriptions/sub-7/action?dry_run=true",
      { method: "POST", body: ' { "action": "pause" } ' },
    );

    await action(request, context("sub-7"));

    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/subscriptions/sub-7/action",
      {
        forwardQuery: false,
        requestContentType: "application/json",
        transformRequestBody: normalizeJsonRequestBody,
      },
    );
  });
});
