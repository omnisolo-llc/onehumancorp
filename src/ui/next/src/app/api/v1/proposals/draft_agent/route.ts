import {
  proxyBackendRequest,
  validateJsonRequestBody,
} from "@/lib/auth/backendTransport";

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/proposals/draft_agent", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: validateJsonRequestBody,
  });
}
