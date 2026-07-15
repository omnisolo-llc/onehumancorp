import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { importMemoryRequest } from "../memoryPayload";

export const runtime = "nodejs";

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/assistant/memory", {
    backendMethod: "PATCH",
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: importMemoryRequest,
  });
}
