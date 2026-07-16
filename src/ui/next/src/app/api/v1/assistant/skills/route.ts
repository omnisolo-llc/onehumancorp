import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/assistant/skills");
}

export function PATCH(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/assistant/skills");
}
