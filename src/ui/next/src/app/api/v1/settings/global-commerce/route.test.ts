import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({ ok: true })));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET, PUT } from "./route";

test("global commerce settings use the authenticated backend transport", async () => {
  const getRequest = new Request("http://localhost/api/v1/settings/global-commerce");
  await GET(getRequest);
  expect(proxyBackendRequest).toHaveBeenLastCalledWith(
    getRequest,
    "/api/v1/settings/global-commerce",
  );

  const putRequest = new Request("http://localhost/api/v1/settings/global-commerce", {
    method: "PUT",
    body: JSON.stringify({ base_currency: "USD", enabled_currencies: ["USD", "EUR"] }),
    headers: { "content-type": "application/json" },
  });
  await PUT(putRequest);
  expect(proxyBackendRequest).toHaveBeenLastCalledWith(
    putRequest,
    "/api/v1/settings/global-commerce",
  );
});
