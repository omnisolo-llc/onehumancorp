import {
  proxyBackendRequest,
  validateJsonRequestBody,
} from "@/lib/auth/backendTransport";

export const runtime = "nodejs";

// This explicit route must precede proposals/[id]. Next's catch-all cannot
// supply POST when the more-specific dynamic route only implements GET.
export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/proposals/intake", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: validateJsonRequestBody,
  });
}
