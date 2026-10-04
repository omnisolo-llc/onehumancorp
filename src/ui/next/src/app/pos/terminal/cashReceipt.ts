import type { CartItem } from '@/lib/business-records';
import type { QueueOwner } from '@/lib/sync/queueIdentity';

export type CashItem = { product_id: string; quantity: number; amount_cents: number };
export type CashRequest = { operation_id: string; tenant_id: string; amount_cents: number; items: CashItem[] };
export type CashAttempt = { owner: QueueOwner; request: CashRequest };
const identifier = /^[A-Za-z0-9._-]{1,128}$/;
const operation = /^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$/;
const record = (value: unknown): value is Record<string, unknown> => !!value && typeof value === 'object' && !Array.isArray(value);

function canonicalItems(value: unknown): CashItem[] {
  if (!Array.isArray(value) || !value.length || value.length > 100) throw new Error('A valid cash cart is required.');
  const result = value.map(item => {
    if (!record(item) || typeof item.product_id !== 'string' || !identifier.test(item.product_id)
      || !Number.isSafeInteger(item.quantity) || Number(item.quantity) < 1 || Number(item.quantity) > 100
      || !Number.isSafeInteger(item.amount_cents) || Number(item.amount_cents) < 0) throw new Error('A valid cash cart is required.');
    return { product_id: item.product_id, quantity: Number(item.quantity), amount_cents: Number(item.amount_cents) };
  }).sort((a, b) => a.product_id.localeCompare(b.product_id));
  if (new Set(result.map(item => item.product_id)).size !== result.length) throw new Error('Duplicate cash cart lines require review.');
  return result;
}

export function cashItems(cart: CartItem[] | undefined, productId: string, amount: number): CashItem[] {
  const items = canonicalItems(cart?.length ? cart.map(item => ({ product_id: item.product.id, quantity: item.quantity,
    amount_cents: Number(item.product.price_cents) * item.quantity })) : [{ product_id: productId, quantity: 1, amount_cents: amount }]);
  if (!Number.isSafeInteger(amount) || amount < 0 || items.reduce((sum, item) => sum + item.amount_cents, 0) !== amount) {
    throw new Error('The cash cart total needs review.');
  }
  return items;
}

const key = (owner: QueueOwner) => `omnisolo_cash_attempt_v1:${JSON.stringify([owner.userId, owner.tenantId])}`;

export function readCashAttempt(owner: QueueOwner): CashAttempt | null {
  const raw = localStorage.getItem(key(owner));
  if (raw === null) return null;
  const value: unknown = JSON.parse(raw);
  if (!record(value) || !record(value.owner) || value.owner.userId !== owner.userId || value.owner.tenantId !== owner.tenantId
    || !record(value.request) || typeof value.request.operation_id !== 'string' || !operation.test(value.request.operation_id)
    || value.request.tenant_id !== owner.tenantId || !Number.isSafeInteger(value.request.amount_cents)) {
    throw new Error('The previous cash sale record needs review before another sale.');
  }
  const items = canonicalItems(value.request.items);
  const amount = Number(value.request.amount_cents);
  if (amount < 0 || items.reduce((sum, item) => sum + item.amount_cents, 0) !== amount) throw new Error('The previous cash cart needs review.');
  return { owner: { ...owner }, request: { operation_id: value.request.operation_id, tenant_id: owner.tenantId, amount_cents: amount, items } };
}

export function persistCashAttempt(owner: QueueOwner, items: CashItem[], amount: number): CashAttempt {
  const attempt = { owner: { ...owner }, request: { operation_id: crypto.randomUUID(), tenant_id: owner.tenantId, amount_cents: amount, items } };
  const encoded = JSON.stringify(attempt);
  localStorage.setItem(key(owner), encoded);
  if (localStorage.getItem(key(owner)) !== encoded) throw new Error('Cash sale recovery storage is unavailable. No sale was submitted.');
  return attempt;
}

export function retireCashAttempt(attempt: CashAttempt): void {
  if (readCashAttempt(attempt.owner)?.request.operation_id !== attempt.request.operation_id) throw new Error('Cash sale recovery record changed.');
  localStorage.removeItem(key(attempt.owner));
  if (localStorage.getItem(key(attempt.owner)) !== null) throw new Error('Cash sale recovery record could not be cleared.');
}

export function confirmedCashReceipt(value: unknown, attempt: CashAttempt): boolean {
  if (!record(value) || value.success !== true || value.status !== 'completed' || !record(value.receipt)) return false;
  const receipt = value.receipt;
  if (receipt.operation_id !== attempt.request.operation_id || receipt.tenant_id !== attempt.owner.tenantId
    || receipt.amount_cents !== attempt.request.amount_cents || typeof receipt.order_id !== 'string' || !identifier.test(receipt.order_id)
    || (receipt.customer_id !== null && receipt.customer_id !== undefined)) return false;
  try {
    return Array.isArray(receipt.items) && receipt.items.every(item => record(item) && item.lock_id === '')
      && JSON.stringify(canonicalItems(receipt.items)) === JSON.stringify(attempt.request.items);
  } catch { return false; }
}

export const cashAttemptLock = (owner: QueueOwner) => key(owner);
