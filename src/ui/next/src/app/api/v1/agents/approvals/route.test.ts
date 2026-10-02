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

const context = (id: string) => ({ params: Promise.resolve({ id }) });
const request = (path: string, method = "GET") =>
  new Request(`http://localhost${path}`, { method });

describe("authenticated approval routes", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  test("confines approval IDs", () => {
    expect(approvalBackendPath("approval-7")).toBe("/api/v1/agents/approvals/approval-7");
    expect(() => approvalBackendPath("../admin")).toThrow("invalid approval ID");
  });

  test("maps supported list, activity, and decision endpoints exactly", async () => {
    await list(request("/api/v1/agents/approvals?limit=20"));
    await activity(request("/api/v1/agents/approvals/activity"));
    await decide(request("/api/v1/agents/approvals/approval-7", "POST"), context("approval-7"));

    expect(proxyBackendRequest.mock.calls.map(([, path]) => path)).toEqual([
      "/api/v1/agents/approvals",
      "/api/v1/agents/approvals/activity",
      "/api/v1/agents/approvals/approval-7",
    ]);
  });

  test.each([
    "simulate-booking-draft", "simulate-lead-recovery", "simulate-quote-draft", "simulate-stockout-reorder",
  ])("preserves the backend absence response for retired simulation ID %s", async (id) => {
    const absent = Response.json(
      { error: "not_found" },
      { status: 404, headers: { "cache-control": "private, no-store" } },
    );
    proxyBackendRequest.mockResolvedValueOnce(absent);
    const response = await decide(
      request(`/api/v1/agents/approvals/${id}`, "POST"), context(id),
    );
    expect(response).toBe(absent);
    expect(response.status).toBe(404);
    expect(response.headers.get("cache-control")).toBe("private, no-store");
    expect(proxyBackendRequest).toHaveBeenCalledExactlyOnceWith(
      expect.any(Request), `/api/v1/agents/approvals/${id}`,
    );
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
