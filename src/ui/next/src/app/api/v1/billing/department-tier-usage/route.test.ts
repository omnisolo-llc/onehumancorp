import { expect, test, vi } from "vitest";

const proxyCurrentBackendPath = vi.hoisted(() =>
  vi.fn(async () => Response.json({ current_plan: "Free", departments: [] })),
);

vi.mock("@/app/api/backendCatchAll", () => ({ proxyCurrentBackendPath }));

import { GET } from "./route";

test("uses the authenticated billing transport", async () => {
  const request = new Request("http://localhost/api/v1/billing/department-tier-usage");

  const response = await GET(request);

  expect(await response.json()).toEqual({ current_plan: "Free", departments: [] });
  expect(proxyCurrentBackendPath).toHaveBeenCalledWith(request);
});
