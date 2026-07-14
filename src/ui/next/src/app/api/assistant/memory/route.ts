import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/assistant/memory");
}

export function PATCH(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/assistant/memory");
}
