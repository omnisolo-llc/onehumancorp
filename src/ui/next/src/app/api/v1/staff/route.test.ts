import { expect, test, vi } from "vitest";

const proxyCurrentBackendPath = vi.hoisted(() => vi.fn(async () => Response.json({ ok: true })));
vi.mock("@/app/api/backendCatchAll", () => ({ proxyCurrentBackendPath }));

import { GET, POST } from "./route";

test("uses authenticated transport for staff reads and writes", async () => {
  const read = new Request("http://localhost/api/v1/staff");
  const write = new Request("http://localhost/api/v1/staff", { method: "POST", body: "{}" });
  await GET(read);
  await POST(write);
  expect(proxyCurrentBackendPath).toHaveBeenNthCalledWith(1, read);
  expect(proxyCurrentBackendPath).toHaveBeenNthCalledWith(2, write);
});
