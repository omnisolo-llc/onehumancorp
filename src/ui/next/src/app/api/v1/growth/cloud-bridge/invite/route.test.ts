import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST } from "./route";

describe("cloud bridge invite transport", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("removes browser-selected team and inviter identities", async () => {
    const upstream = new Response("{}", { status: 200 });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request("https://app.example.test/api/v1/growth/cloud-bridge/invite", {
      method: "POST",
      body: JSON.stringify({ team_id: "forged", inviter_id: "forged", invitee_id: "person@example.test" }),
    });
    expect(await POST(request)).toBe(upstream);
    const options = proxyBackendRequest.mock.calls[0][2];
    const transformed = await options.transformRequestBody(
      new TextEncoder().encode(JSON.stringify({ team_id: "forged", inviter_id: "forged", invitee_id: "person@example.test" })),
    );
    expect(JSON.parse(new TextDecoder().decode(transformed))).toEqual({ invitee_id: "person@example.test" });
  });
});
