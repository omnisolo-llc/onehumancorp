import {
  proxyBackendRequest,
  validateJsonRequestBody,
} from "@/lib/auth/backendTransport";

export function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/mesh/v2/collective");
}

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/mesh/v2/collective", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: validateJsonRequestBody,
  });
}
