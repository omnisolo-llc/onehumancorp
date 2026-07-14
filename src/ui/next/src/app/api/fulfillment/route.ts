import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/fulfillment");
}
