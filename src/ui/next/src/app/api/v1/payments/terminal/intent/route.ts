import { proxyBackendRequest } from '@/lib/auth/backendTransport';
const decoder = new TextDecoder('utf-8', { fatal: true });
const encoder = new TextEncoder();
function normalizedIntent(body: Uint8Array<ArrayBuffer>): Uint8Array<ArrayBuffer> {
  const value: unknown = JSON.parse(decoder.decode(body));
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('Invalid request');
  const input = value as Record<string, unknown>;
  const allowed = new Set(['amount_cents','amount','currency','product_id','productId','quantity','order_id','orderId','idempotency_key','total','tenant_id','tenantId']);
  if (Object.keys(input).some(key => !allowed.has(key))) throw new Error('Unsupported payment field');
  if (input.amount !== undefined && input.amount_cents !== undefined && input.amount !== input.amount_cents) throw new Error('Conflicting amount');
  if (input.product_id !== undefined && input.productId !== undefined && input.product_id !== input.productId) throw new Error('Conflicting product');
  return encoder.encode(JSON.stringify({
    amount_cents: input.amount_cents ?? input.amount,
    currency: input.currency ?? 'usd',
    product_id: input.product_id ?? input.productId ?? null,
    quantity: input.quantity ?? null,
    order_id: input.order_id ?? input.orderId ?? null,
    idempotency_key: input.idempotency_key,
    ...(input.total !== undefined ? { total: input.total } : {}),
  }));
}
function privateJson(status: number, body: unknown): Response {
  return Response.json(body, { status, headers: { 'cache-control':'private, no-store', pragma:'no-cache', 'x-content-type-options':'nosniff' } });
}
export async function POST(request: Request): Promise<Response> {
  const response = await proxyBackendRequest(request, '/api/v1/payments/terminal/intent', { transformRequestBody: normalizedIntent });
  if (!response.ok) return response;
  const body: unknown = await response.json().catch(() => null);
  if (!body || typeof body !== 'object' || Array.isArray(body) || 'Err' in body || 'error' in body) {
    return privateJson(502, { success:false, status:'unknown', error:'Backend did not confirm a persisted terminal intent.' });
  }
  const value = ('Ok' in body ? body.Ok : body) as Record<string, unknown> | null;
  if (!value || typeof value !== 'object' || Array.isArray(value) || value.success === false || value.error != null || 'Err' in value || typeof value.client_secret !== 'string'
    || typeof value.payment_intent_id !== 'string' || !/^pi_[A-Za-z0-9_]+$/.test(value.payment_intent_id)
    || !value.client_secret.startsWith(`${value.payment_intent_id}_secret_`)
    || typeof value.operation_id !== 'string' || !/^[A-Za-z0-9_-]{1,128}$/.test(value.operation_id)
    || !Number.isSafeInteger(value.amount_cents) || (value.amount_cents as number) <= 0
    || typeof value.currency !== 'string' || !/^[a-z]{3}$/.test(value.currency)) {
    return privateJson(502, { success:false, status:'unknown', error:'Backend did not confirm a persisted terminal intent.' });
  }
  return privateJson(200, value);
}
