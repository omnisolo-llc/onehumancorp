import { proxyBackendRequest } from "@/lib/auth/backendTransport";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

function normalizedIntent(body: Uint8Array<ArrayBuffer>): Uint8Array<ArrayBuffer> {
  const value = JSON.parse(decoder.decode(body));
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("invalid request");
  }
  const input = value as Record<string, unknown>;
  return new Uint8Array(
    encoder.encode(
      JSON.stringify({
        amount_cents: input.amount_cents ?? input.amount,
        currency: input.currency ?? "usd",
        product_id: input.product_id ?? null,
        quantity: input.quantity ?? null,
        order_id: input.order_id ?? null,
      }),
    ),
  );
}

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/payments/terminal/intent", {
    transformRequestBody: normalizedIntent,
  });
}
