import { expect, test, vi } from "vitest";

const proxyCurrentBackendPath = vi.hoisted(() => vi.fn(async () => Response.json({ plans: [] })));
vi.mock("@/app/api/backendCatchAll", () => ({ proxyCurrentBackendPath }));

import { GET } from "./route";

test("uses authenticated transport for subscription reads", async () => {
  const request = new Request("http://localhost/api/subscriptions");
  const response = await GET(request);
  expect(await response.json()).toEqual({ plans: [] });
  expect(proxyCurrentBackendPath).toHaveBeenCalledWith(request);
});
