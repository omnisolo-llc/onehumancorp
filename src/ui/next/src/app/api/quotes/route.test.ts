import { beforeEach, describe, expect, test, vi } from "vitest";
import type { BackendRequestOptions } from "@/lib/auth/backendTransport";

const proxyBackendRequest = vi.hoisted(() =>
  vi.fn<
    (request: Request, path: string, options?: BackendRequestOptions) => Promise<Response>
  >(async () => Response.json({ ok: true })),
);
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST as accept } from "./[id]/accept/route";
import { POST as approve } from "./[id]/approve/route";
import { GET as getByPath } from "./[id]/route";
import { GET, POST } from "./route";

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

    expect(proxyBackendRequest.mock.calls.map(([, path]) => path)).toEqual([
      "/api/v1/quotes",
      "/api/v1/quotes",
      "/api/v1/quotes/quote-7",
      "/api/v1/quotes/quote-7",
    ]);
    expect(proxyBackendRequest.mock.calls[3][2]).toEqual({ backendMethod: "PUT" });
  });

  test("maps dynamic quote actions with confined IDs", async () => {
    await getByPath(new Request("http://localhost/api/quotes/quote-7"), context("quote-7"));
    await accept(new Request("http://localhost/api/quotes/quote-7/accept", { method: "POST" }), context("quote-7"));
    await approve(new Request("http://localhost/api/quotes/quote-7/approve", { method: "POST" }), context("quote-7"));

    expect(proxyBackendRequest.mock.calls.map(([, path]) => path)).toEqual([
      "/api/v1/quotes/quote-7",
      "/api/v1/quotes/quote-7/accept",
      "/api/v1/quotes/quote-7/approve",
    ]);
    expect(proxyBackendRequest.mock.calls[2][2]).toEqual({ backendMethod: "PATCH" });
  });

  test("rejects injected quote IDs before transport", async () => {
    const response = await GET(new Request("http://localhost/api/quotes?id=../admin"));
    expect(response.status).toBe(400);
    expect(proxyBackendRequest).not.toHaveBeenCalled();
  });
});
