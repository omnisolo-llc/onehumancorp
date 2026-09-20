import { proxyBackendRequest } from "@/lib/auth/backendTransport";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

function privateJson(status: number, value: unknown): Response {
  return Response.json(value, {
    status,
    headers: {
      "cache-control": "private, no-store",
      pragma: "no-cache",
      "x-content-type-options": "nosniff",
    },
  });
}

function normalizedIntent(body: Uint8Array<ArrayBuffer>): Uint8Array<ArrayBuffer> {
  const value = JSON.parse(decoder.decode(body));
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("invalid request");
  }
  const input = value as Record<string, unknown>;
  return new Uint8Array(
    encoder.encode(
      JSON.stringify({
        amount_cents: Number(input.amount_cents ?? input.amount),
        currency: String(input.currency ?? "usd").toLowerCase(),
        product_id: input.product_id ?? null,
        quantity: input.quantity ?? null,
        order_id: input.order_id ?? null,
      }),
    ),
  );
}

function clientSecret(payload: unknown): string | undefined {
  if (payload === null || typeof payload !== "object" || Array.isArray(payload)) return;
  const value = payload as Record<string, unknown>;
  if (typeof value.client_secret === "string") return value.client_secret;
  if (typeof value.intent_id === "string") return value.intent_id;
  if (value.Ok !== null && typeof value.Ok === "object" && !Array.isArray(value.Ok)) {
    const ok = value.Ok as Record<string, unknown>;
    if (typeof ok.client_secret === "string") return ok.client_secret;
    if (typeof ok.intent_id === "string") return ok.intent_id;
  }
}

export async function POST(request: Request): Promise<Response> {
  const response = await proxyBackendRequest(
    request,
    "/api/v1/payments/terminal/intent",
    { transformRequestBody: normalizedIntent },
  );
  if (!response.ok) return response;
  const payload = await response.json().catch(() => null);
  const secret = clientSecret(payload);

  // Also forward lock_id if present
  let lockId = undefined;
  if (payload !== null && typeof payload === "object" && !Array.isArray(payload)) {
      const value = payload as Record<string, unknown>;
      if (typeof value.lock_id === "string") lockId = value.lock_id;
      else if (value.Ok !== null && typeof value.Ok === "object" && !Array.isArray(value.Ok)) {
          const ok = value.Ok as Record<string, unknown>;
          if (typeof ok.lock_id === "string") lockId = ok.lock_id;
      }
  }

  return secret === undefined
    ? privateJson(502, {
        error: "Backend response did not include a PaymentIntent client secret",
      })
    : privateJson(200, { client_secret: secret, lock_id: lockId });
}
