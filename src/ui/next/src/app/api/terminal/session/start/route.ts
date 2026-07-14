import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/payments/terminal/session/start");
}
