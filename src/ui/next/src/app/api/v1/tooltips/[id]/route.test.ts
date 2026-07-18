import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({})));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { DELETE } from "./route";

test("uses authenticated transport for tooltips DELETE", async () => {
  const request = new Request("http://localhost/api/v1/tooltips/test-id", {
    method: "DELETE"
  });
  await DELETE(request, { params: { id: "test-id" } });
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/tooltips/test-id");
});
