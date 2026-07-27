import { expect, test, vi } from "vitest";

const proxyCurrentBackendPath = vi.hoisted(() =>
  vi.fn(async () => Response.json({ checkout_url: "https://checkout.example/session" })),
);

vi.mock("@/app/api/backendCatchAll", () => ({ proxyCurrentBackendPath }));

import { POST } from "./route";

test("uses the authenticated billing transport", async () => {
  proxyCurrentBackendPath.mockResolvedValueOnce(
    Response.json({ checkout_url: "https://checkout.example/session" })
  );

  const request = new Request("http://localhost/api/v1/billing/create-checkout-session", {
    method: "POST",
    body: '{"tier":"Starter"}',
  });

  const response = await POST(request);

  expect(await response.json()).toEqual({
    checkout_url: "https://checkout.example/session",
  });
  expect(proxyCurrentBackendPath).toHaveBeenCalledWith(request);
});

test("fails closed with 503 instead of returning a mock fallback URL on failure", async () => {
  proxyCurrentBackendPath.mockResolvedValueOnce(
    new Response(JSON.stringify({ error: "backend unavailable" }), { status: 503 })
  );

  const request = new Request("http://localhost/api/v1/billing/create-checkout-session", {
    method: "POST",
    body: '{"tier":"Starter"}',
  });

  const response = await POST(request);

  expect(response.status).toBe(503);
  expect(await response.json()).toEqual({ error: "backend unavailable" });
});
