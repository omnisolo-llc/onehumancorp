import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() =>
  vi.fn(async () => Response.json({ results: ["success"] })),
);

vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST } from "./route";

test("uses the authenticated backend transport for code-native execution", async () => {
  const request = new Request("http://localhost/api/v1/agents/code-native", {
    method: "POST",
    body: '{"test":true}',
  });

  const response = await POST(request);

  expect(await response.json()).toEqual({ results: ["success"] });
  expect(proxyBackendRequest).toHaveBeenCalledWith(
    request,
    "/api/v1/agents/code-native",
  );
});
