import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export const runtime = "nodejs";

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/proposals/draft");
}
