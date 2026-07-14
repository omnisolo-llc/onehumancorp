import { expect, test, vi } from "vitest";

const proxyCurrentBackendPath = vi.hoisted(() =>
  vi.fn(async () => Response.json({ checkout_url: "https://checkout.example/session" })),
);

vi.mock("@/app/api/backendCatchAll", () => ({ proxyCurrentBackendPath }));

import { POST } from "./route";

test("uses the authenticated billing transport", async () => {
  const request = new Request("http://localhost/api/billing/create-checkout-session", {
    method: "POST",
    body: '{"tier":"Starter"}',
  });

  const response = await POST(request);

  expect(await response.json()).toEqual({
    checkout_url: "https://checkout.example/session",
  });
  expect(proxyCurrentBackendPath).toHaveBeenCalledWith(request);
});
