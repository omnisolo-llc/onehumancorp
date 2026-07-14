import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { normalizeChatBody } from "./chatBackend";

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/chat", {
    transformRequestBody: normalizeChatBody,
  });
}
