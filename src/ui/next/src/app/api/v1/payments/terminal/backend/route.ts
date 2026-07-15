import { proxyBackendRequest } from "@/lib/auth/backendTransport";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

function normalizeBackend(body: Uint8Array<ArrayBuffer>): Uint8Array<ArrayBuffer> {
  const value = JSON.parse(decoder.decode(body)) as { backend?: unknown };
  if (value.backend !== "local" && value.backend !== "docker") {
    throw new Error("invalid terminal backend");
  }
  return encoder.encode(JSON.stringify({ backend: value.backend }));
}

export function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/payments/terminal/backend", {
    forwardQuery: false,
    suppressRequestBody: true,
  });
}

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/payments/terminal/backend", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: normalizeBackend,
  });
}
