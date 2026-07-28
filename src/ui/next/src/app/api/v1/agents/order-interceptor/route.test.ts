import { POST } from "./route";
import { proxyBackendPost } from "../../ui/backendProxy";
import { expect, test, vi } from "vitest";

vi.mock("../../ui/backendProxy", () => ({
  proxyBackendPost: vi.fn(),
}));

test("POST /api/v1/agents/order-interceptor calls proxyBackendPost", async () => {
  const req = new Request("http://localhost/api/v1/agents/order-interceptor", {
    method: "POST",
    body: JSON.stringify({ raw_input: "test" }),
  });
  await POST(req);
  expect(proxyBackendPost).toHaveBeenCalledWith(req, "/api/v1/agents/order-interceptor");
});
