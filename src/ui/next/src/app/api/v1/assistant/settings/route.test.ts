import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({ settings: {} })));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET, PATCH } from "./route";

test("assistant settings uses the authenticated backend transport", async () => {
  const getRequest = new Request("http://localhost/api/v1/assistant/settings");
  await GET(getRequest);
  expect(proxyBackendRequest).toHaveBeenCalledWith(
    getRequest,
    "/api/v1/assistant/settings",
    { forwardQuery: false, suppressRequestBody: true },
  );

  const patchRequest = new Request("http://localhost/api/v1/assistant/settings", {
    method: "PATCH",
    body: JSON.stringify({ agentName: "OmniSolo Assistant" }),
  });
  await PATCH(patchRequest);
  expect(proxyBackendRequest).toHaveBeenCalledWith(
    patchRequest,
    "/api/v1/assistant/settings",
    { forwardQuery: false, requestContentType: "application/json" },
  );
});
