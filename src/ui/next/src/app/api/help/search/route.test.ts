import { expect, test, vi } from "vitest";
const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json([])));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));
import { GET } from "./route";
test("uses authenticated transport for help search", async () => {
  const request = new Request("http://localhost/api/help/search?q=payments");
  await GET(request);
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/help/search");
});
