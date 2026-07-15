import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() =>
  vi.fn(async () => Response.json({ status: "running" }, { status: 201 })),
);

vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST } from "./route";

test("uses the authenticated backend transport for agent hiring", async () => {
  const request = new Request("http://localhost/api/v1/agents/hire", {
    method: "POST",
    body: '{"name":"Growth Strategist","role":"Operator"}',
  });

  const response = await POST(request);

  expect(response.status).toBe(201);
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/agents/hire");
});
