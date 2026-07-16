import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export const runtime = "nodejs";

export function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/ui/omni_inbox", {
    forwardQuery: false,
    suppressRequestBody: true,
  });
}
