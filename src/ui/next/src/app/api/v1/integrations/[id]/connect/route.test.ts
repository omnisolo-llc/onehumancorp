import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock("@/lib/auth/backendTransport", async (importOriginal) => ({
  ...(await importOriginal<typeof import("@/lib/auth/backendTransport")>()),
  proxyBackendRequest,
}));

import { POST } from "./route";

describe("integration connection transport", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("keeps WhatsApp bound to its route id without trusting browser identity", async () => {
    const upstream = new Response("{}", { status: 200 });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request("https://app.example.test/api/v1/integrations/whatsapp/connect", { method: "POST", body: "{}" });
    expect(await POST(request, { params: Promise.resolve({ id: "whatsapp" }) })).toBe(upstream);
    expect(proxyBackendRequest.mock.calls[0][1]).toBe("/api/v1/integrations/whatsapp/connect");
    const transformed = await proxyBackendRequest.mock.calls[0][2].transformRequestBody(new TextEncoder().encode("{}"));
    expect(JSON.parse(new TextDecoder().decode(transformed))).toEqual({});
  });

  it("does not let the request body override the route integration", async () => {
    proxyBackendRequest.mockResolvedValue(new Response("{}", { status: 200 }));
    const request = new Request("https://app.example.test/api/v1/integrations/twilio/connect", {
      method: "POST",
      body: JSON.stringify({
        integration_id: "attacker-selected",
        tenant_id: "forged",
        nested: { user_id: "forged" },
        bot_token: "AC123",
        api_token: "secret",
      }),
    });

    await POST(request, { params: Promise.resolve({ id: "twilio" }) });

    const transformed = await proxyBackendRequest.mock.calls[0][2].transformRequestBody(
      new TextEncoder().encode(await request.clone().text()),
    );
    expect(JSON.parse(new TextDecoder().decode(transformed))).toEqual({
      bot_token: "AC123",
      api_token: "secret",
    });
  });

  it("rejects an invalid route integration id before proxying", async () => {
    const response = await POST(
      new Request("https://app.example.test/api/v1/integrations/bad/connect", { method: "POST" }),
      { params: Promise.resolve({ id: "../twilio" }) },
    );

    expect(response.status).toBe(400);
    expect(proxyBackendRequest).not.toHaveBeenCalled();
  });
});
