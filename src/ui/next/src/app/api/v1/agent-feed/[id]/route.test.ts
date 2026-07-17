import { beforeEach, expect, test, vi } from "vitest";

const proxyBackendPut = vi.hoisted(() => vi.fn(async () => Response.json({})));
vi.mock("@/app/api/v1/ui/backendProxy", () => ({ proxyBackendPut }));

import { PUT } from "./route";

beforeEach(() => proxyBackendPut.mockClear());

test("updates a confined agent-feed item through authenticated transport", async () => {
  const request = new Request("http://localhost/api/v1/agent-feed/item-1", { method: "PUT" });
  await PUT(request as never, { params: Promise.resolve({ id: "item-1" }) });
  expect(proxyBackendPut).toHaveBeenCalledWith(request, "/api/v1/agent-feed/item-1/state");
});

test("rejects an invalid agent-feed id", async () => {
  const response = await PUT(new Request("http://localhost/api/v1/agent-feed/bad", { method: "PUT" }) as never, {
    params: Promise.resolve({ id: "../admin" }),
  });
  expect(response.status).toBe(400);
  expect(proxyBackendPut).not.toHaveBeenCalled();
});
