import { proxyBackendRequest } from "@/lib/auth/backendTransport";

import { sanitizeOnboardingLaunchRequest } from "../statePayload";

export const runtime = "nodejs";

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/onboarding/launch", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: sanitizeOnboardingLaunchRequest,
  });
}
