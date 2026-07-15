import {
  proxyBackendRequest,
} from "@/lib/auth/backendTransport";
import { sanitizeOnboardingZeroClickRequest } from "../../../onboarding/statePayload";

export const runtime = "nodejs";

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/onboarding/start_zero_click", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: sanitizeOnboardingZeroClickRequest,
  });
}
