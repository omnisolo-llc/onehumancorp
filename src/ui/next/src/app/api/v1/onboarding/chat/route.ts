import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export const runtime = "nodejs";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

function validateMessagesRequest(
  body: Uint8Array<ArrayBuffer>,
): Uint8Array<ArrayBuffer> {
  const payload = JSON.parse(decoder.decode(body)) as { messages?: unknown };
  if (!Array.isArray(payload.messages)) {
    throw new Error("messages array is required");
  }
  return encoder.encode(JSON.stringify({ messages: payload.messages }));
}

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/onboarding/chat", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: validateMessagesRequest,
  });
}
