import { beforeEach, describe, expect, test, vi } from "vitest";
import type { BackendRequestOptions } from "@/lib/auth/backendTransport";

const proxyBackendRequest = vi.hoisted(() =>
  vi.fn<
    (request: Request, path: string, options?: BackendRequestOptions) => Promise<Response>
  >(async () => Response.json({ ok: true })),
);

vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST as decide } from "./[id]/route";
import { GET as activity } from "./activity/route";
import { approvalBackendPath } from "./approvalBackend";
import { GET as list } from "./route";
import { POST as simulateBooking } from "./simulate-booking-draft/route";
import { POST as simulateLeadRecovery } from "./simulate-lead-recovery/route";
import { POST as simulateQuote } from "./simulate-quote-draft/route";
import { POST as simulateStockout } from "./simulate-stockout-reorder/route";

const context = (id: string) => ({ params: Promise.resolve({ id }) });
const request = (path: string, method = "GET") =>
  new Request(`http://localhost${path}`, { method });

describe("authenticated approval routes", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  test("confines approval IDs", () => {
    expect(approvalBackendPath("approval-7")).toBe("/api/v1/agents/approvals/approval-7");
    expect(() => approvalBackendPath("../admin")).toThrow("invalid approval ID");
  });

  test("maps list, activity, decision, and simulation endpoints exactly", async () => {
    await list(request("/api/v1/agents/approvals?limit=20"));
    await activity(request("/api/v1/agents/approvals/activity"));
    await decide(request("/api/v1/agents/approvals/approval-7", "POST"), context("approval-7"));
    await simulateBooking(request("/api/v1/agents/approvals/simulate-booking-draft", "POST"));
    await simulateLeadRecovery(request("/api/v1/agents/approvals/simulate-lead-recovery", "POST"));
    await simulateQuote(request("/api/v1/agents/approvals/simulate-quote-draft", "POST"));
    await simulateStockout(request("/api/v1/agents/approvals/simulate-stockout-reorder", "POST"));

    expect(proxyBackendRequest.mock.calls.map(([, path]) => path)).toEqual([
      "/api/v1/agents/approvals",
      "/api/v1/agents/approvals/activity",
      "/api/v1/agents/approvals/approval-7",
      "/api/v1/agents/approvals/simulate-autonomous-booking-quote",
      "/api/v1/agents/approvals/simulate-lead-recovery",
      "/api/v1/agents/approvals/simulate-quote-draft",
      "/api/v1/agents/approvals/simulate-stockout-reorder",
    ]);
  });

  test("rejects invalid IDs before transport", async () => {
    const response = await decide(
      request("/api/v1/agents/approvals/bad", "POST"),
      context("../admin"),
    );

    expect(response.status).toBe(400);
    expect(proxyBackendRequest).not.toHaveBeenCalled();
  });

});
