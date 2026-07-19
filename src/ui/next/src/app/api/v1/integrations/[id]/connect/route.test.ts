import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST } from "./route";

describe("integration connection transport", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("maps WhatsApp to Twilio without trusting browser identity", async () => {
    const upstream = new Response("{}", { status: 200 });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request("https://app.example.test/api/v1/integrations/whatsapp/connect", { method: "POST", body: "{}" });
    expect(await POST(request, { params: Promise.resolve({ id: "whatsapp" }) })).toBe(upstream);
    expect(proxyBackendRequest.mock.calls[0][1]).toBe("/api/v1/integrations/twilio/connect");
    const transformed = await proxyBackendRequest.mock.calls[0][2].transformRequestBody(new TextEncoder().encode("{}"));
    expect(JSON.parse(new TextDecoder().decode(transformed))).toEqual({ integration_id: "whatsapp" });
  });
});
