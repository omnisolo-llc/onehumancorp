import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export const dynamic = "force-dynamic";

export function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/changelog");
}
