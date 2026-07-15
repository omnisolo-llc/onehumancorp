import { beforeEach, describe, expect, test, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({
  proxyBackendRequest: vi.fn<
    (
      request: Request,
      backendPath: string,
      options?: {
        backendMethod?: string;
        forwardQuery?: boolean;
        requestContentType?: string;
        suppressRequestBody?: true;
        transformRequestBody?: (
          body: Uint8Array<ArrayBuffer>,
        ) => Uint8Array<ArrayBuffer>;
      },
    ) => Promise<Response>
  >(async () => Response.json({ ok: true })),
}));

vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { DELETE as forgetMemory } from "./[id]/route";
import { POST as searchMemory } from "./cross-session/route";
import { GET as listMemory } from "./route";
import { GET as customerSummary } from "./summary/[customerId]/route";
import { POST as importMemory } from "./upload/route";

const encode = (value: unknown) => new TextEncoder().encode(JSON.stringify(value));
const decode = (value: Uint8Array<ArrayBuffer> | undefined) =>
  JSON.parse(new TextDecoder().decode(value));

describe("authenticated memory backend routes", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  test("list uses the canonical assistant memory endpoint", async () => {
    const request = new Request("http://localhost/api/v1/memory?tenant_id=attacker");
    await listMemory(request);
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/assistant/memory",
      { forwardQuery: false, suppressRequestBody: true },
    );
  });

  test("delete validates the id and emits the typed forget mutation", async () => {
    const request = new Request("http://localhost/api/v1/memory/memory-1", {
      method: "DELETE",
    });
    await forgetMemory(request, { params: Promise.resolve({ id: "memory-1" }) });
    const options = proxyBackendRequest.mock.calls[0][2];
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/assistant/memory",
      expect.objectContaining({
        backendMethod: "PATCH",
        forwardQuery: false,
        requestContentType: "application/json",
      }),
    );
    expect(decode(options?.transformRequestBody?.(new Uint8Array()))).toEqual({
      action: "forget",
      id: "memory-1",
    });

    const invalid = await forgetMemory(request, {
      params: Promise.resolve({ id: "../attacker" }),
    });
    expect(invalid.status).toBe(400);
  });

  test("upload preserves bounded source metadata and strips extra fields", async () => {
    const request = new Request("http://localhost/api/v1/memory/upload", {
      method: "POST",
      body: "{}",
    });
    await importMemory(request);
    const options = proxyBackendRequest.mock.calls[0][2];
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/assistant/memory",
      expect.objectContaining({ backendMethod: "PATCH", forwardQuery: false }),
    );
    expect(
      decode(
        options?.transformRequestBody?.(
          encode({
            content: "Policy content",
            source_type: "policy.md",
            tenant_id: "attacker",
          }),
        ),
      ),
    ).toEqual({
      action: "import",
      content: "Policy content",
      scope: "global",
      source: "policy.md",
    });
  });

  test("cross-session search strips session authority and bounds inputs", async () => {
    const request = new Request("http://localhost/api/v1/memory/cross-session", {
      method: "POST",
      body: "{}",
    });
    await searchMemory(request);
    const options = proxyBackendRequest.mock.calls[0][2];
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/assistant/memory/cross-session-search",
      expect.objectContaining({ forwardQuery: false }),
    );
    expect(
      decode(
        options?.transformRequestBody?.(
          encode({ query: "vegan", limit: 10, summarize: true, session_id: "global" }),
        ),
      ),
    ).toEqual({ query: "vegan", limit: 10, summarize: true });
    expect(() =>
      options?.transformRequestBody?.(encode({ query: "x".repeat(501) })),
    ).toThrow();
  });

  test("customer summary derives tenant server-side", async () => {
    const request = new Request("http://localhost/api/v1/memory/summary/customer-1");
    await customerSummary(request, {
      params: Promise.resolve({ customerId: "customer-1" }),
    });
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/memory/summary/customer-1",
      { forwardQuery: false, suppressRequestBody: true },
    );
  });
});
