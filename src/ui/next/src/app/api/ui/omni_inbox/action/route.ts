import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export const runtime = "nodejs";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

function actionRequest(body: Uint8Array<ArrayBuffer>): Uint8Array<ArrayBuffer> {
  const input = JSON.parse(decoder.decode(body)) as Record<string, unknown>;
  if (
    input === null ||
    typeof input !== "object" ||
    Array.isArray(input) ||
    typeof input.message_id !== "string" ||
    !/^[A-Za-z0-9._-]{1,200}$/.test(input.message_id) ||
    typeof input.approved !== "boolean" ||
    (input.edited_reply !== undefined &&
      (typeof input.edited_reply !== "string" || input.edited_reply.length > 16_000))
  ) {
    throw new Error("invalid inbox action");
  }
  return encoder.encode(
    JSON.stringify({
      message_id: input.message_id,
      approved: input.approved,
      ...(input.edited_reply === undefined ? {} : { edited_reply: input.edited_reply }),
    }),
  );
}

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/ui/omni_inbox/action", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: actionRequest,
  });
}
