import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/ui/triage/create");
}
