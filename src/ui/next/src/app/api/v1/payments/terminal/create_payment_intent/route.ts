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

function normalizeIntent(body: Uint8Array<ArrayBuffer>): Uint8Array<ArrayBuffer> {
  const value = JSON.parse(decoder.decode(body));
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("invalid request");
  }
  const input = value as Record<string, unknown>;
  return new Uint8Array(
    encoder.encode(
      JSON.stringify({
        amount_cents: Number(input.amount ?? input.amount_cents),
        currency: String(input.currency ?? "usd").toLowerCase(),
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
    { transformRequestBody: normalizeIntent },
  );
  if (!response.ok) return response;
  const payload = await response.json().catch(() => null);
  const secret = clientSecret(payload);
  return secret === undefined
    ? privateJson(502, {
        error: "Backend response did not include a PaymentIntent client secret",
      })
    : privateJson(200, { client_secret: secret });
}
