import { beforeEach, expect, test, vi } from "vitest";
import { FaultInjector } from "@/lib/chaos";

const { proxyBackendRequest } = vi.hoisted(() => ({
  proxyBackendRequest: vi.fn(),
}));
vi.mock("@/lib/auth/backendTransport", async (importOriginal) => ({
  ...(await importOriginal<typeof import("@/lib/auth/backendTransport")>()),
  proxyBackendRequest,
}));

import { POST } from "./route";

beforeEach(() => {
  proxyBackendRequest.mockReset();
  vi.spyOn(FaultInjector, "applyFault").mockResolvedValue(undefined);
});

test("POST rejects a missing task during the bounded transform", async () => {
  proxyBackendRequest.mockImplementation(async (request, _path, options) => {
    options.transformRequestBody(new Uint8Array(await request.arrayBuffer()));
    return Response.json({});
  });
  const request = new Request("https://app.example.test/api/v1/ralph-loop", {
    method: "POST",
    body: JSON.stringify({}),
  });

  const response = await POST(request);
  expect(response.status).toBe(400);
  await expect(response.json()).resolves.toEqual({ error: "task is required" });
});

test("POST maps the authenticated JSON-RPC result", async () => {
  proxyBackendRequest.mockResolvedValue(Response.json({
    jsonrpc: "2.0",
    id: "rpc-1",
    result: { status: "success" },
  }));
  const request = new Request("https://app.example.test/api/v1/ralph-loop", {
    method: "POST",
    body: JSON.stringify({ task: "Do a long loop" }),
  });

  const response = await POST(request);
  expect(response.status).toBe(200);
  await expect(response.json()).resolves.toEqual({ result: { status: "success" } });
  expect(proxyBackendRequest).toHaveBeenCalledWith(
    request,
    "/api/v1/rpc",
    expect.objectContaining({ requestContentType: "application/json" }),
  );
});

test("POST maps a JSON-RPC error without exposing a direct backend", async () => {
  proxyBackendRequest.mockResolvedValue(Response.json({
    jsonrpc: "2.0",
    error: { message: "Internal RPC Error" },
  }));
  const request = new Request("https://app.example.test/api/v1/ralph-loop", {
    method: "POST",
    body: JSON.stringify({ task: "Do a long loop" }),
  });

  const response = await POST(request);
  expect(response.status).toBe(502);
  await expect(response.json()).resolves.toEqual({ error: "Internal RPC Error" });
});

test("POST preserves a transport outage response", async () => {
  const unavailable = Response.json({ error: "backend unavailable" }, { status: 503 });
  proxyBackendRequest.mockResolvedValue(unavailable);
  const request = new Request("https://app.example.test/api/v1/ralph-loop", {
    method: "POST",
    body: JSON.stringify({ task: "Do a long loop" }),
  });

  expect(await POST(request)).toBe(unavailable);
});
