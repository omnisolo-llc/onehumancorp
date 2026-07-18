import { beforeEach, describe, expect, it, vi } from "vitest";
import type { BackendRequestOptions } from "@/lib/auth/backendTransport";

const { proxyBackendRequest } = vi.hoisted(() => ({
  proxyBackendRequest: vi.fn(
    async (
      _request: Request,
      _backendPath: string,
      _options?: BackendRequestOptions,
    ) => Response.json({ ok: true }),
  ),
}));

vi.mock("@/lib/auth/backendTransport", async (importOriginal) => ({
  ...(await importOriginal<typeof import("@/lib/auth/backendTransport")>()),
  proxyBackendRequest,
}));

import { proxyCurrentBackendPath } from "./backendCatchAll";
import { stripBrowserIdentityJsonRequestBody } from "@/lib/auth/backendTransport";

describe("proxyCurrentBackendPath", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  it("sanitizes browser identity from JSON bodies", async () => {
    const request = new Request("https://app.example.test/api/v1/example", {
      method: "POST",
      headers: { "content-type": "application/json; charset=utf-8" },
      body: JSON.stringify({ tenant_id: "forged" }),
    });

    await proxyCurrentBackendPath(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/example", {
      transformRequestBody: stripBrowserIdentityJsonRequestBody,
    });
  });

  it("does not apply a JSON transform to non-JSON bodies", async () => {
    const request = new Request("https://app.example.test/api/v1/example", {
      method: "POST",
      headers: { "content-type": "text/plain" },
      body: "hello",
    });

    await proxyCurrentBackendPath(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/example");
  });

  it("preserves a staff role while stripping forged identity from catch-all JSON", async () => {
    const request = new Request("https://app.example.test/api/v1/staff", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        name: "Alex",
        phone_number: "+15550001111",
        role: "Cashier",
        tenant_id: "forged",
        nested: { authorization: "Bearer forged", roles: ["trainer"] },
      }),
    });

    await proxyCurrentBackendPath(request);

    const transformRequestBody =
      proxyBackendRequest.mock.calls[0]?.[2]?.transformRequestBody;
    expect(transformRequestBody).toBeTypeOf("function");
    if (transformRequestBody === undefined) {
      throw new Error("expected the JSON body sanitizer");
    }
    const encoded = new TextEncoder().encode(await request.clone().text());
    const transformed = await transformRequestBody(encoded);
    expect(JSON.parse(new TextDecoder().decode(transformed))).toEqual({
      name: "Alex",
      phone_number: "+15550001111",
      role: "Cashier",
      nested: { roles: ["trainer"] },
    });
  });
});
