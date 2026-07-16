import { beforeEach, expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json([])));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET } from "./route";

beforeEach(() => proxyBackendRequest.mockClear());

test("uses authenticated transport for a walkthrough page", async () => {
  const request = new Request("http://localhost/api/v1/walkthrough/dashboard");
  await GET(request, { params: Promise.resolve({ page: "dashboard" }) });
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/walkthrough/dashboard");
});

test("rejects an invalid walkthrough page", async () => {
  const response = await GET(new Request("http://localhost/api/v1/walkthrough/bad"), {
    params: Promise.resolve({ page: "../admin" }),
  });
  expect(response.status).toBe(400);
  expect(proxyBackendRequest).not.toHaveBeenCalled();
});
