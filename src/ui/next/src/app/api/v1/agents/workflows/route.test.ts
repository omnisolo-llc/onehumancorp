import { beforeEach, describe, expect, it, vi } from "vitest";

const proxyCurrentBackendPath = vi.hoisted(() =>
  vi.fn(async () => Response.json({ source: "backend" }, { status: 202 })),
);

vi.mock("@/app/api/backendCatchAll", () => ({ proxyCurrentBackendPath }));

import { GET, POST } from "./route";

describe("agent workflow API", () => {
  beforeEach(() => {
    proxyCurrentBackendPath.mockClear();
  });

  it("delegates workflow reads to the authenticated backend", async () => {
    const request = new Request("http://localhost/api/v1/agents/workflows");

    const response = await GET(request);

    expect(response.status).toBe(202);
    expect(proxyCurrentBackendPath).toHaveBeenCalledOnce();
    expect(proxyCurrentBackendPath).toHaveBeenCalledWith(request);
  });

  it("delegates workflow creation to the authenticated backend", async () => {
    const request = new Request("http://localhost/api/v1/agents/workflows", {
      method: "POST",
      body: JSON.stringify({ name: "Branch review", task: "Review the branch" }),
    });

    const response = await POST(request);

    expect(response.status).toBe(202);
    expect(proxyCurrentBackendPath).toHaveBeenCalledOnce();
    expect(proxyCurrentBackendPath).toHaveBeenCalledWith(request);
  });
});
