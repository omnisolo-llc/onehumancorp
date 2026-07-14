import {
  normalizeJsonRequestBody,
  proxyBackendRequest,
} from "@/lib/auth/backendTransport";

export async function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/checkout/delivery-quote", {
    forwardQuery: false,
    transformRequestBody: normalizeJsonRequestBody,
  });
}
