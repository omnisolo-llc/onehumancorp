import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({})));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET, POST } from "./route";

test("uses authenticated transport for tooltips GET", async () => {
  const request = new Request("http://localhost/api/v1/tooltips");
  await GET(request);
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/tooltips");
});

test("uses authenticated transport for tooltips POST", async () => {
  const request = new Request("http://localhost/api/v1/tooltips", {
    method: "POST",
    body: JSON.stringify({ id: "test", text: "text" })
  });
  await POST(request);
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/tooltips");
});
