import { beforeEach, expect, test, vi } from "vitest";

const proxyBackendGet = vi.hoisted(() => vi.fn(async () => Response.json({ items: [] })));
const proxyBackendPost = vi.hoisted(() => vi.fn(async () => Response.json({})));
vi.mock("@/app/api/v1/ui/backendProxy", () => ({ proxyBackendGet, proxyBackendPost }));

import { GET, POST } from "./route";

beforeEach(() => {
  proxyBackendGet.mockClear();
  proxyBackendPost.mockClear();
});

test("proxies GET /api/v1/agent-feed without trailing slash", async () => {
  const request = new Request("http://localhost/api/v1/agent-feed?limit=5&offset=0");
  await GET(request);
  expect(proxyBackendGet).toHaveBeenCalledWith(request, "/api/v1/agent-feed");
});

test("proxies POST /api/v1/agent-feed without trailing slash", async () => {
  const request = new Request("http://localhost/api/v1/agent-feed", { method: "POST", body: "{}" });
  await POST(request);
  expect(proxyBackendPost).toHaveBeenCalledWith(request, "/api/v1/agent-feed");
});
