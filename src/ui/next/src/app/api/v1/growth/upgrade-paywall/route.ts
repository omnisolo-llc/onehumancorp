import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function GET(request: Request) {
  return proxyBackendRequest(request, "/api/v1/growth/upgrade-paywall", {
    suppressRequestBody: true,
  });
}
