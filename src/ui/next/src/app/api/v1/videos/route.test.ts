import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json([])));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET } from "./route";

test("uses authenticated transport for videos", async () => {
  const request = new Request("http://localhost/api/v1/videos?mobile_optimized=true");
  await GET(request);
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/videos");
});
