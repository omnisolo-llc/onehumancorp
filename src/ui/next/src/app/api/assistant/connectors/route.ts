import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/assistant/connectors");
}

export function PATCH(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/assistant/connectors");
}
