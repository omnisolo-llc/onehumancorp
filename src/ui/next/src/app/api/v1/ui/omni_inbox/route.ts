import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export const runtime = "nodejs";

export function GET(request: Request): Promise<Response> {
  const url = new URL(request.url);
  const forwardQuery = url.searchParams.has("mobile_optimized") || url.searchParams.has("fields");
  return proxyBackendRequest(request, "/api/v1/ui/omni_inbox", {
    forwardQuery,
    suppressRequestBody: true,
  });
}
