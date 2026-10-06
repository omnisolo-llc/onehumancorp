import { MANUAL_INBOX_REQUEST_ID } from "@/lib/inboxManualReceipt";
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
    (input.request_id !== undefined && (typeof input.request_id !== "string" || !MANUAL_INBOX_REQUEST_ID.test(input.request_id))) ||
    (input.prepare_only !== undefined && typeof input.prepare_only !== "boolean") ||
    (input.prepare_only === true && input.approved !== true) ||
    (input.edited_reply !== undefined &&
      (typeof input.edited_reply !== "string" || input.edited_reply.length > 16_000))
  ) {
    throw new Error("invalid inbox action");
  }
  return encoder.encode(
    JSON.stringify({
      message_id: input.message_id,
      approved: input.approved,
      ...(input.request_id === undefined ? {} : { request_id: input.request_id }),
      ...(input.prepare_only === undefined ? {} : { prepare_only: input.prepare_only }),
      ...(input.edited_reply === undefined ? {} : { edited_reply: input.edited_reply }),
    }),
  );
}

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/ui/omni_inbox/action", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: actionRequest,
  });
}

/** Receipt lookup never trusts caller-supplied identity, and is never cacheable. */
export function GET(request: Request): Promise<Response> {
  const url = new URL(request.url);
  const messageIds = url.searchParams.getAll("message_id");
  const requestIds = url.searchParams.getAll("request_id");
  if (messageIds.length !== 1 || !/^[A-Za-z0-9._-]{1,200}$/.test(messageIds[0])
    || requestIds.length > 1 || (requestIds.length === 1 && !MANUAL_INBOX_REQUEST_ID.test(requestIds[0]))) {
    return Promise.resolve(Response.json({ error: "Invalid receipt selectors" }, {
      status: 400, headers: { "cache-control": "private, no-store", pragma: "no-cache", "x-content-type-options": "nosniff" },
    }));
  }
  url.search = new URLSearchParams({ message_id: messageIds[0], ...(requestIds.length ? { request_id: requestIds[0] } : {}) }).toString();
  return proxyBackendRequest(new Request(url, request), "/api/v1/ui/omni_inbox/action", {
    forwardQuery: true,
    suppressRequestBody: true,
  });
}
