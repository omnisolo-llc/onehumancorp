import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { sanitizeOnboardingStartRequest } from "../statePayload";

export const runtime = "nodejs";

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/onboarding/start", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: sanitizeOnboardingStartRequest,
  });
}
