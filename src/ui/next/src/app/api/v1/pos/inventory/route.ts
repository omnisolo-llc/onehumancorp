import {
  proxyBackendRequest,
  validateJsonRequestBody,
} from "@/lib/auth/backendTransport";

const PATH = "/api/v1/pos/inventory";

export function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, PATH, { suppressRequestBody: true });
}

export function DELETE(request: Request): Promise<Response> {
  return proxyBackendRequest(request, PATH, {
    forwardQuery: false,
    suppressRequestBody: true,
  });
}

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, PATH, {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: validateJsonRequestBody,
  });
}
