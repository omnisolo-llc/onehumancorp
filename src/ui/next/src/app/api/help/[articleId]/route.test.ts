import { beforeEach, expect, test, vi } from "vitest";
const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({ id: "payments" })));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));
import { GET } from "./route";
const context = (articleId: string) => ({ params: Promise.resolve({ articleId }) });
beforeEach(() => proxyBackendRequest.mockClear());
test("uses a confined article path", async () => {
  const request = new Request("http://localhost/api/help/payments");
  await GET(request, context("payments"));
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/help/payments");
});
test("rejects article path injection", async () => {
  const response = await GET(new Request("http://localhost/api/help/bad"), context("../admin"));
  expect(response.status).toBe(400);
  expect(proxyBackendRequest).not.toHaveBeenCalled();
});
