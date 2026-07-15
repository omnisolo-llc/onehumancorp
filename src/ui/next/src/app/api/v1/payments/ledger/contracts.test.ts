import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({})));
vi.mock("@/lib/auth/backendTransport", () => ({
  proxyBackendRequest,
  validateJsonRequestBody: vi.fn(),
}));

import { GET as balance } from "./balance/route";
import { POST as intent } from "./intent/route";
import { POST as receipt } from "./receipt/route";
import { GET as safeToSpend } from "./safe-to-spend/route";

test("uses authenticated v1 transport for ledger operations", async () => {
  const balanceRequest = new Request("http://localhost/api/v1/payments/ledger/balance");
  await balance(balanceRequest);
  expect(proxyBackendRequest).toHaveBeenCalledWith(balanceRequest, "/api/v1/payments/ledger/balance", {
    forwardQuery: false,
    suppressRequestBody: true,
  });

  const intentRequest = new Request("http://localhost/api/v1/payments/ledger/intent", {
    method: "POST",
    body: JSON.stringify({ amount: 100 }),
  });
  await intent(intentRequest);
  expect(proxyBackendRequest).toHaveBeenCalledWith(intentRequest, "/api/v1/payments/ledger/intent", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: expect.any(Function),
  });

  const receiptRequest = new Request("http://localhost/api/v1/payments/ledger/receipt", {
    method: "POST",
    body: JSON.stringify({ vendor: "Store", amount: 10 }),
  });
  await receipt(receiptRequest);
  expect(proxyBackendRequest).toHaveBeenCalledWith(receiptRequest, "/api/v1/payments/ledger/receipt", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: expect.any(Function),
  });

  const safeRequest = new Request("http://localhost/api/v1/payments/ledger/safe-to-spend");
  await safeToSpend(safeRequest);
  expect(proxyBackendRequest).toHaveBeenCalledWith(safeRequest, "/api/v1/payments/ledger/safe-to-spend", {
    forwardQuery: false,
    suppressRequestBody: true,
  });
});
