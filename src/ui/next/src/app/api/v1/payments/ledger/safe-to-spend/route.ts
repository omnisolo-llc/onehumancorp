import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/payments/ledger/safe-to-spend", {
    forwardQuery: false,
    suppressRequestBody: true,
  });
}
