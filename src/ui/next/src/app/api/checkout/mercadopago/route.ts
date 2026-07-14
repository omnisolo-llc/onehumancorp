import {
  normalizeJsonRequestBody,
  proxyBackendRequest,
} from "@/lib/auth/backendTransport";

export async function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/checkout/mercadopago", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: normalizeJsonRequestBody,
  });
}
