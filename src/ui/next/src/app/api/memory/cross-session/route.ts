import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { crossSessionMemoryRequest } from "../memoryPayload";

export const runtime = "nodejs";

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/assistant/memory/cross-session-search", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: crossSessionMemoryRequest,
  });
}
