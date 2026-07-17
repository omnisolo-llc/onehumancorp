import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function POST(request: Request) {
  return proxyBackendRequest(request, "/api/v1/field-ops/optimize-route");
}
