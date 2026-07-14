import { expect, test, vi } from "vitest";

const proxyCurrentBackendPath = vi.hoisted(() =>
  vi.fn(async () => Response.json({ success: true })),
);

vi.mock("@/app/api/backendCatchAll", () => ({ proxyCurrentBackendPath }));

import { POST } from "./route";

test("uses the authenticated billing transport", async () => {
  const request = new Request("http://localhost/api/billing/report-cost", {
    method: "POST",
    body: '{"value":100}',
  });

  const response = await POST(request);

  expect(await response.json()).toEqual({ success: true });
  expect(proxyCurrentBackendPath).toHaveBeenCalledWith(request);
});
