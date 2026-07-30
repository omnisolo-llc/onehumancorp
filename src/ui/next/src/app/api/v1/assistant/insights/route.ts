import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export function GET(request: Request): Promise<Response> {
  // We stream real insights from the backend AI job queue / data layer instead of using mock data
  return proxyBackendRequest(request, "/api/v1/assistant/insights");
}
