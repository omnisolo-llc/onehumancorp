import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST } from "./route";

describe("POST /api/v1/booking/engine/reserve", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("maps customer contact data without fabricating a UUID customer id", async () => {
    proxyBackendRequest.mockResolvedValue(new Response("{}", { status: 200 }));
    const request = new Request("https://app.example.test/api/v1/booking/engine/reserve", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        customer_name: "Jane Doe",
        customer_email: "jane@example.test",
        customer_id: "jane@example.test",
        product_id: "service-real",
        start_time: "2026-10-10T09:00:00Z",
        end_time: "2026-10-10T10:00:00Z",
      }),
    });

    await POST(request);

    const options = proxyBackendRequest.mock.calls[0]?.[2];
    const transformed = options.transformRequestBody(new TextEncoder().encode(await request.text()));
    expect(JSON.parse(new TextDecoder().decode(transformed))).toEqual({
      customer_name: "Jane Doe",
      customer_email: "jane@example.test",
      service_id: "service-real",
      start_time: "2026-10-10T09:00:00Z",
      end_time: "2026-10-10T10:00:00Z",
    });
  });
});
