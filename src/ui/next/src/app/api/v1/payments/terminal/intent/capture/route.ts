import { proxyBackendRequest } from '@/lib/auth/backendTransport';
const decoder = new TextDecoder('utf-8', { fatal:true });
const encoder = new TextEncoder();
function normalizedCapture(body: Uint8Array<ArrayBuffer>): Uint8Array<ArrayBuffer> {
  const value: unknown = JSON.parse(decoder.decode(body));
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('Invalid request');
  const input = value as Record<string, unknown>;
  if (['items','cart','cart_payload'].some(key => input[key] != null)) throw new Error('Unbound cart');
  if (input.payment_intent_id !== undefined && input.paymentIntentId !== undefined && input.payment_intent_id !== input.paymentIntentId) throw new Error('Conflicting payment ID');
  const { paymentIntentId, ...remaining } = input;
  return encoder.encode(JSON.stringify({ ...remaining, payment_intent_id: input.payment_intent_id ?? paymentIntentId }));
}
export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, '/api/v1/payments/terminal/intent/capture', { transformRequestBody: normalizedCapture });
}
