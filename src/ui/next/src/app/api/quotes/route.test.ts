import { beforeEach, describe, expect, test, vi } from "vitest";
import type { BackendRequestOptions } from "@/lib/auth/backendTransport";

const { validateJsonRequestBody, proxyBackendRequest } = vi.hoisted(() => ({
  validateJsonRequestBody: vi.fn((body: Uint8Array<ArrayBuffer>) => body),
  proxyBackendRequest: vi.fn<
    (request: Request, path: string, options?: BackendRequestOptions) => Promise<Response>
  >(async () => Response.json({ ok: true })),
}));
vi.mock("@/lib/auth/backendTransport", () => ({
  validateJsonRequestBody,
  proxyBackendRequest,
}));

import { POST as accept } from "./[id]/accept/route";
import { PATCH as approve } from "./[id]/approve/route";
import { GET as getByPath } from "./[id]/route";
import { GET, POST } from "./route";
import * as quoteBackend from "./quoteBackend";

const context = (id: string) => ({ params: Promise.resolve({ id }) });

describe("authenticated quote routes", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  test("maps legacy list, create, read, and update requests", async () => {
    await GET(new Request("http://localhost/api/quotes"));
    await POST(new Request("http://localhost/api/quotes", { method: "POST", body: "{}" }));
    await GET(new Request("http://localhost/api/quotes?id=quote-7"));
    await POST(
      new Request("http://localhost/api/quotes?id=quote-7", { method: "POST", body: "{}" }),
    );

    const validateLegacyQuoteBody = (
      quoteBackend as typeof quoteBackend & {
        validateLegacyQuoteBody: (body: Uint8Array<ArrayBuffer>) => Uint8Array<ArrayBuffer>;
      }
    ).validateLegacyQuoteBody;
    expect(proxyBackendRequest.mock.calls).toEqual([
      [expect.any(Request), "/api/v1/quotes", {
        forwardQuery: false,
        requestContentType: "application/json",
      }],
      [expect.any(Request), "/api/v1/quotes", {
        backendMethod: "POST",
        forwardQuery: false,
        requestContentType: "application/json",
        transformRequestBody: validateLegacyQuoteBody,
      }],
      [expect.any(Request), "/api/v1/quotes/quote-7", {
        forwardQuery: false,
        requestContentType: "application/json",
      }],
      [expect.any(Request), "/api/v1/quotes/quote-7", {
        backendMethod: "PUT",
        forwardQuery: false,
        requestContentType: "application/json",
        transformRequestBody: validateLegacyQuoteBody,
      }],
    ]);
  });

  test("maps dynamic quote actions with confined IDs", async () => {
    await getByPath(new Request("http://localhost/api/quotes/quote-7"), context("quote-7"));
    await accept(new Request("http://localhost/api/quotes/quote-7/accept", { method: "POST" }), context("quote-7"));
    await approve(
      new Request("http://localhost/api/quotes/quote-7/approve", {
        method: "PATCH",
        body: '{"attacker":"payload"}',
      }),
      context("quote-7"),
    );

    expect(proxyBackendRequest.mock.calls).toEqual([
      [expect.any(Request), "/api/v1/quotes/quote-7", {
        forwardQuery: false,
        requestContentType: "application/json",
      }],
      [expect.any(Request), "/api/v1/quotes/quote-7/accept", {
        forwardQuery: false,
        requestContentType: "application/json",
        suppressRequestBody: true,
      }],
      [expect.any(Request), "/api/v1/quotes/quote-7/approve", {
        backendMethod: "PATCH",
        forwardQuery: false,
        requestContentType: "application/json",
        suppressRequestBody: true,
      }],
    ]);
  });

  test("rejects injected quote IDs before transport", async () => {
    const response = await GET(new Request("http://localhost/api/quotes?id=../admin"));
    expect(response.status).toBe(400);
    expect(proxyBackendRequest).not.toHaveBeenCalled();
  });
});
