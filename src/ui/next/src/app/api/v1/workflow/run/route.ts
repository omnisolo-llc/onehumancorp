import {
  proxyBackendRequest,
  validateJsonRequestBody,
} from "@/lib/auth/backendTransport";

export const runtime = "nodejs";

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/workflow/run", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: validateJsonRequestBody,
  });
}
