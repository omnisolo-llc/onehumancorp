import {
  proxyBackendRequest,
  validateJsonRequestBody,
} from "@/lib/auth/backendTransport";

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/agents/guardrails/anthropic", {
    requestContentType: "application/json",
    forwardQuery: false,
    transformRequestBody: validateJsonRequestBody,
  });
}
