import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export const runtime = "nodejs";

export function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/assistant/memory", {
    forwardQuery: false,
    suppressRequestBody: true,
  });
}
