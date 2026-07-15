import { beforeEach, expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({})));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET } from "./route";

beforeEach(() => proxyBackendRequest.mockClear());

test("uses authenticated transport for a confined proposal id", async () => {
  const request = new Request("http://localhost/api/v1/proposals/proposal-1");
  await GET(request, { params: Promise.resolve({ id: "proposal-1" }) });
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/proposals/proposal-1");
});

test("rejects an invalid proposal id", async () => {
  const response = await GET(new Request("http://localhost/api/v1/proposals/bad"), {
    params: Promise.resolve({ id: "../admin" }),
  });
  expect(response.status).toBe(400);
  expect(proxyBackendRequest).not.toHaveBeenCalled();
});
