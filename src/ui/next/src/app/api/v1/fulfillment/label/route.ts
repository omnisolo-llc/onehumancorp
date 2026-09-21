import {
  validateJsonRequestBody,
  proxyBackendRequest,
} from "@/lib/auth/backendTransport";

export async function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/fulfillment/label", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: validateJsonRequestBody,
  });
}
