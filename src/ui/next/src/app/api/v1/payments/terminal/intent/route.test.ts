import { beforeEach, afterEach, expect, it, vi } from 'vitest';
import { authenticatedCookie, stubAuthEnvironment, TEST_BACKEND_ORIGIN, TEST_WEB_ORIGIN } from '@/lib/auth/authTestFixtures';
import { POST } from './route';
import { POST as legacyIntent } from '@/app/api/v1/pos/terminal/payment-intent/route';
import { POST as legacyCapture } from '@/app/api/v1/pos/terminal/capture/route';

const transport = vi.fn<typeof fetch>();
beforeEach(() => { stubAuthEnvironment(); transport.mockReset(); vi.stubGlobal('fetch', transport); });
afterEach(() => { vi.unstubAllEnvs(); vi.unstubAllGlobals(); vi.restoreAllMocks(); });
async function request(path: string, body: unknown) {
  return new Request(`${TEST_WEB_ORIGIN}${path}`, { method:'POST', headers:{'content-type':'application/json',cookie:await authenticatedCookie()}, body:JSON.stringify(body) });
}
it('normalizes the actual Rust Result receipt without losing persisted identity', async () => {
  const receipt={ client_secret:'pi_abc_secret_def',payment_intent_id:'pi_abc',operation_id:'operation-one',amount_cents:5000,currency:'usd',lock_id:null };
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({Ok:receipt}));
  const res=await POST(await request('/api/v1/payments/terminal/intent',{amount_cents:5000,currency:'usd',idempotency_key:'operation-one'}));
  expect(res.status).toBe(200); expect(await res.json()).toEqual(receipt);
  const [url,init]=vi.mocked(fetch).mock.calls[0]; expect(String(url)).toBe(`${TEST_BACKEND_ORIGIN}/api/v1/payments/terminal/intent`);
  expect(JSON.parse(await new Response(init?.body).text())).toEqual(expect.objectContaining({idempotency_key:'operation-one'}));
});
it('never forwards a legacy HTTP-200 Err as a successful intent', async () => {
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({Err:'Provider is unavailable'}));
  const res=await POST(await request('/api/v1/payments/terminal/intent',{amount_cents:5000,idempotency_key:'operation-one'}));
  expect(res.status).toBe(502); expect(await res.json()).not.toHaveProperty('client_secret');
});
it('routes the legacy creation alias through authenticated canonical persistence',async()=>{
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({client_secret:'pi_abc_secret_def',payment_intent_id:'pi_abc',operation_id:'operation-one',amount_cents:5000,currency:'usd',lock_id:null}));
  const res=await legacyIntent(await request('/api/v1/pos/terminal/payment-intent',{amount:5000,currency:'usd',idempotency_key:'operation-one'}));
  expect(res.status).toBe(200); expect(String(vi.mocked(fetch).mock.calls[0]?.[0])).toBe(`${TEST_BACKEND_ORIGIN}/api/v1/payments/terminal/intent`);
});
it('routes the legacy capture alias through canonical ownership preflight',async()=>{
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({success:false,error:'Payment not found'},{status:404}));
  const res=await legacyCapture(await request('/api/v1/pos/terminal/capture',{paymentIntentId:'pi_foreign'}));
  expect(res.status).toBe(404); const [url,init]=vi.mocked(fetch).mock.calls[0];
  expect(String(url)).toBe(`${TEST_BACKEND_ORIGIN}/api/v1/payments/terminal/intent/capture`);
  expect(JSON.parse(await new Response(init?.body).text())).toEqual({payment_intent_id:'pi_foreign'});
});
it.each([
  { amount:5000, amount_cents:1, idempotency_key:'same' },
  { amount_cents:5000, idempotency_key:'same', cart_payload:[{product_id:'other'}] },
  { amount_cents:5000, idempotency_key:'same', items:[{product_id:'other'}] },
])('rejects ambiguous or unbound input before backend transport: %j',async body=>{
  const res=await POST(await request('/api/v1/payments/terminal/intent',body));
  expect(res.status).toBe(400);expect(fetch).not.toHaveBeenCalled();
});
it('rejects a secret without a persisted operation identity',async()=>{
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({client_secret:'pi_abc_secret_fixture'}));
  const res=await POST(await request('/api/v1/payments/terminal/intent',{amount_cents:5000,idempotency_key:'same'}));
  expect(res.status).toBe(502);
});
it('does not expose a secret from an explicit failed receipt',async()=>{
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({success:false,client_secret:'pi_abc_secret_fixture',payment_intent_id:'pi_abc',operation_id:'same',amount_cents:5000,currency:'usd'}));
  const res=await POST(await request('/api/v1/payments/terminal/intent',{amount_cents:5000,idempotency_key:'same'}));
  expect(res.status).toBe(502);expect(await res.json()).not.toHaveProperty('client_secret');
});
