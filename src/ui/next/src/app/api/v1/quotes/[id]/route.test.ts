import { beforeEach, describe, expect, it, vi } from "vitest";
import type { BackendRequestOptions } from "@/lib/auth/backendTransport";
const { proxyBackendRequest } = vi.hoisted(() => ({
  proxyBackendRequest: vi.fn<(request: Request, path: string, options?: BackendRequestOptions) => Promise<Response>>(),
}));
vi.mock("@/lib/auth/backendTransport", () => ({
  proxyBackendRequest,
  validateJsonRequestBody: (body: Uint8Array<ArrayBuffer>) => { JSON.parse(new TextDecoder().decode(body)); return body; },
}));
import { PUT } from "./route";
import { POST as accept } from "./accept/route";
const context = () => ({ params: Promise.resolve({ id: "quote-1" }) });
const request = (method: string, body = "{}") => new Request("http://localhost/api/v1/quotes/quote-1", { method, body });
describe("quote mutation routes use authenticated backend outcomes", () => {
  beforeEach(() => { proxyBackendRequest.mockReset(); proxyBackendRequest.mockResolvedValue(Response.json({ success: true })); });
  it("forwards owner updates and preserves a real backend rejection", async () => {
    const body = { total_amount: 500, required_deposit: 100, line_items: [] };
    const req = request("PUT", JSON.stringify(body));
    const backend = Response.json({ success: false, reason: "accepted quote is immutable" }, { status: 409 });
    proxyBackendRequest.mockResolvedValue(backend);
    expect(await PUT(req, context())).toBe(backend);
    expect(proxyBackendRequest).toHaveBeenCalledWith(req, "/api/v1/quotes/quote-1", expect.objectContaining({ backendMethod: "PUT", forwardQuery: false }));
    const options = proxyBackendRequest.mock.calls[0][2]!;
    expect(options.suppressRequestBody).not.toBe(true);
    expect(JSON.parse(new TextDecoder().decode(await options.transformRequestBody!(new TextEncoder().encode(JSON.stringify(body)))))).toEqual(body);
  });
  it("forwards the exact reviewed acceptance timestamp", async () => {
    const body = { expected_updated_at: "2026-10-01T03:00:00.123456+00:00" };
    await accept(request("POST", JSON.stringify(body)), context());
    const options = proxyBackendRequest.mock.calls[0][2]!;
    expect(options.suppressRequestBody).not.toBe(true);
    expect(JSON.parse(new TextDecoder().decode(await options.transformRequestBody!(new TextEncoder().encode(JSON.stringify(body)))))).toEqual(body);
  });
  it("permits empty receipt replay and rejects malformed review tokens", async () => {
    await accept(request("POST"), context());
    const transform = proxyBackendRequest.mock.calls[0][2]!.transformRequestBody!;
    expect(JSON.parse(new TextDecoder().decode(await transform(new Uint8Array())))).toEqual({});
    for (const body of ["{", "[]", "null", '{"expected_updated_at":1}', '{"expected_updated_at":"tomorrow"}', '{"tenant_id":"forged"}']) {
      expect(() => transform(new TextEncoder().encode(body))).toThrow();
    }
  });
  it("rejects path escape before backend transport", async () => {
    expect((await PUT(request("PUT"), { params: Promise.resolve({ id: "../admin" }) })).status).toBe(400);
    expect(proxyBackendRequest).not.toHaveBeenCalled();
  });
});
