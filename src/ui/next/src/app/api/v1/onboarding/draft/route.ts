import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { sanitizeOnboardingStateRequest } from "../statePayload";

export const runtime = "nodejs";

export function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/onboarding/draft", {
    forwardQuery: false,
    suppressRequestBody: true,
  });
}

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/onboarding/draft", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: sanitizeOnboardingStateRequest,
  });
}
