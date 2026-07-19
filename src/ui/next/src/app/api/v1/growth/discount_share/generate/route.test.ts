import { expect, test, vi } from "vitest";

const proxyCurrentBackendPath = vi.hoisted(() =>
  vi.fn(async () => Response.json({ embed_code: "from backend" })),
);
vi.mock("@/app/api/backendCatchAll", () => ({ proxyCurrentBackendPath }));

import { POST } from "./route";

test("delegates discount-share generation to the authenticated backend", async () => {
  const request = new Request("http://localhost/api/v1/growth/discount_share/generate", {
    method: "POST",
    body: JSON.stringify({ campaignName: "Summer Sale" }),
  });

  const response = await POST(request);

  expect(proxyCurrentBackendPath).toHaveBeenCalledWith(request);
  await expect(response.json()).resolves.toEqual({ embed_code: "from backend" });
});
