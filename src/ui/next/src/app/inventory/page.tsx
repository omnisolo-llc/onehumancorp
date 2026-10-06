"use client";

import { useEffect, useRef, useState } from 'react';
import { AppShell } from '../components/AppShell';
import { currentVerifiedQueueLease, QUEUE_IDENTITY_EPOCH_KEY, readQueueOwner, sameOwner, type QueueOwner } from '@/lib/sync/queueIdentity';

type Product = { id: string; name: string; description: string | null; price_cents: number | null; currency: string | null; stock: number; inventory_version: string };
type Adjustment = { id: string; payload: { item_id: string; quantity_change: number; expected_version: string } };
const object = (value: unknown): value is Record<string, unknown> => !!value && typeof value === 'object' && !Array.isArray(value);
const validVersion = (value: unknown): value is string => typeof value === 'string' && /^[a-f0-9]{64}$/.test(value);
const quantity = (value: unknown): value is number => typeof value === 'number' && Number.isSafeInteger(value) && value >= 0 && value <= 2147483647;
const pendingKey = (owner: QueueOwner) => `inventory-adjustment-v1:${encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId]))}`;
function parseInventory(value: unknown): Product[] {
  if (!object(value) || value.error != null || value.success === false || !Array.isArray(value.inventory)) throw new Error('Inventory is unavailable. Reload to try again.');
  const ids = new Set<string>();
  return value.inventory.map(row => {
    if (!object(row) || typeof row.id !== 'string' || !row.id || ids.has(row.id) || typeof row.name !== 'string' || !quantity(row.stock) || (row.price_cents != null && (!Number.isSafeInteger(row.price_cents) || (row.price_cents as number) < 0)) || (row.description != null && typeof row.description !== 'string') || (row.currency != null && (typeof row.currency !== 'string' || !/^[A-Z]{3}$/.test(row.currency))) || !validVersion(row.inventory_version)) throw new Error('Inventory is unavailable. Reload to try again.');
    ids.add(row.id);
    return row as Product;
  });
}
function parseAdjustment(raw: string): Adjustment {
  const value: unknown = JSON.parse(raw);
  if (!object(value) || Object.keys(value).some(key => !['id', 'payload'].includes(key)) || typeof value.id !== 'string' || !/^[a-zA-Z0-9_-]{1,128}$/.test(value.id) || !object(value.payload) || Object.keys(value.payload).some(key => !['item_id', 'quantity_change', 'expected_version'].includes(key)) || typeof value.payload.item_id !== 'string' || !value.payload.item_id || ![-1, 1].includes(value.payload.quantity_change as number) || !validVersion(value.payload.expected_version)) throw new Error('The saved adjustment needs review. No further stock change was sent.');
  return value as Adjustment;
}

export default function InventoryDashboard() {
  const [products, setProducts] = useState<Product[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [message, setMessage] = useState('');
  const [pending, setPending] = useState<Adjustment | null>(null);
  const [saving, setSaving] = useState(false);
  const [retryAllowed, setRetryAllowed] = useState(false);
  const [ready, setReady] = useState(false);
  const owner = useRef<QueueOwner | null>(null);
  const generation = useRef(0);
  const busy = useRef(false);
  const held = useRef(false);
  const active = useRef(false);
  const expiry = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const current = (expected: QueueOwner, token: number) => active.current && generation.current === token && !!owner.current && sameOwner(owner.current, expected);

  function retireInventory(reason = 'Your session changed. Reopen inventory to verify your business.') {
    generation.current += 1; owner.current = null; held.current = true; busy.current = false;
    setReady(false); setLoading(false); setSaving(false); setRetryAllowed(false);
    setProducts([]); setPending(null); setMessage(''); setError(reason);
    clearTimeout(expiry.current);
    // Recovery envelopes remain under their original verified owner key. Never
    // erase an unknown operation merely because its account is no longer active.
  }
  async function verifyOwner(expected: QueueOwner, token: number): Promise<boolean> {
    if (!current(expected, token)) return false;
    try {
      const verified = await readQueueOwner();
      if (!current(expected, token)) return false;
      if (!sameOwner(verified, expected)) { retireInventory(); return false; }
      return true;
    } catch {
      if (current(expected, token)) retireInventory('Inventory access could not be verified. Reload to verify your business.');
      return false;
    }
  }

  useEffect(() => {
    active.current = true;
    const retire = () => retireInventory();
    const storage = (event: StorageEvent) => { if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) retire(); };
    window.addEventListener('omnisolo_auth_changed', retire);
    window.addEventListener('storage', storage);
    window.addEventListener('pagehide', retire);
    void loadInventory();
    return () => {
      active.current = false; generation.current += 1; clearTimeout(expiry.current);
      window.removeEventListener('omnisolo_auth_changed', retire);
      window.removeEventListener('storage', storage);
      window.removeEventListener('pagehide', retire);
    };
  }, []);

  async function loadInventory() {
    const token = ++generation.current;
    setLoading(true); setProducts([]); setError(''); setMessage(''); setReady(false); setRetryAllowed(false);
    try {
      const expected = await readQueueOwner();
      if (!active.current || generation.current !== token) return;
      owner.current = expected;
      const saved = sessionStorage.getItem(pendingKey(expected));
      const restored = saved ? parseAdjustment(saved) : null;
      held.current = !!restored; setPending(restored);
      const res = await fetch('/api/v1/ui/inventory', { headers: { 'x-ohc-expected-user': expected.userId, 'x-ohc-expected-tenant': expected.tenantId }, credentials: 'same-origin', cache: 'no-store', redirect: 'error' });
      if (res.status === 401 || res.status === 403) { if (current(expected, token)) retireInventory('Your session no longer permits inventory access. Reload to verify your business.'); return; }
      if (res.status !== 200) throw new Error('Inventory could not be loaded. Reload to try again.');
      const rows = parseInventory(await res.json());
      if (!await verifyOwner(expected, token)) return;
      const lease = currentVerifiedQueueLease();
      if (!lease || !sameOwner(lease.owner, expected)) throw new Error('Inventory access could not be verified.');
      clearTimeout(expiry.current);
      expiry.current = setTimeout(() => {
        retireInventory('Your session expired. Reload inventory to verify your business.');
      }, Math.min(lease.expiresAt - Date.now(), 2147483647));
      setProducts(rows); setReady(true);
      if (restored) setMessage('An earlier stock adjustment is unconfirmed. Check its saved result before making another change.');
    } catch (e) {
      if (active.current && generation.current === token) {
        retireInventory(e instanceof Error ? e.message : 'Inventory is unavailable.');
      }
    } finally { if (active.current && generation.current === token) setLoading(false); }
  }

  async function saveAdjustment(request: Adjustment, mode: 'write' | 'read' = 'write') {
    const expected = owner.current, token = generation.current;
    if (!expected || busy.current || !ready) return;
    const earlier = !!pending;
    busy.current = true; held.current = true; setSaving(true); setPending(request); setRetryAllowed(false);
    setMessage(mode === 'read' ? 'Checking recorded adjustment…' : 'Saving stock adjustment…');
    try {
      // Preserve intent under the original verified owner before checking fresh
      // identity. A silent account change must not discard its recovery envelope.
      if (mode === 'write') sessionStorage.setItem(pendingKey(expected), JSON.stringify(request));
      if (!await verifyOwner(expected, token)) return;
      const url = mode === 'read' ? `/api/v1/ui/inventory?adjustment_id=${encodeURIComponent(request.id)}` : '/api/v1/ui/inventory';
      const res = await fetch(url, { method: mode === 'read' ? 'GET' : 'POST', headers: { 'Content-Type': 'application/json', 'x-ohc-expected-user': expected.userId, 'x-ohc-expected-tenant': expected.tenantId }, credentials: 'same-origin', cache: 'no-store', redirect: 'error', ...(mode === 'write' ? { body: JSON.stringify([request]) } : {}) });
      if (res.status === 401 || res.status === 403) {
        if (current(expected, token)) retireInventory('Your session no longer permits inventory access. Reload to verify your business.');
        return;
      }
      const body: unknown = await res.json();
      if (!await verifyOwner(expected, token)) return;
      if (mode === 'read' && res.status === 404 && object(body) && body.receipt_status === 'not_found' && body.id === request.id) {
        setRetryAllowed(true);
        setMessage('No saved receipt was found. The original result is still unconfirmed. You can explicitly retry the same adjustment.');
        return;
      }
      const outcome = object(body) && Array.isArray(body.outcomes) && body.outcomes.length === 1 ? body.outcomes[0] : null;
      if (!object(outcome) || outcome.id !== request.id || outcome.item_id !== request.payload.item_id || outcome.quantity_change !== request.payload.quantity_change || outcome.previous_version !== request.payload.expected_version) throw new Error('Stock adjustment could not be confirmed.');
      if (mode === 'write' && res.status === 200 && outcome.status === 'blocked') {
        sessionStorage.removeItem(pendingKey(expected)); setPending(null); held.current = false;
        setMessage(outcome.reason === 'inventory_debt_requires_reconciliation' ? 'Stock adjustment is blocked by an unresolved inventory shortage. Reconcile the shortage before adjusting stock.' : 'Stock was not changed. Reload inventory and review the current quantity before trying again.');
        setReady(false); return;
      }
      if (res.status !== 200 || !object(body) || body.success !== true || body.error != null || outcome.status !== 'acknowledged' || !quantity(outcome.stock) || !validVersion(outcome.inventory_version)) throw new Error('Stock adjustment could not be confirmed.');
      sessionStorage.removeItem(pendingKey(expected)); setPending(null); held.current = false;
      if (mode === 'read' || earlier) {
        setReady(false);
        setMessage('Earlier stock adjustment was saved. Reload inventory for current stock.');
      } else {
        setProducts(rows => rows.map(row => row.id === request.payload.item_id ? { ...row, stock: outcome.stock as number, inventory_version: outcome.inventory_version as string } : row));
        setMessage('Stock adjustment saved.');
      }
    } catch {
      if (current(expected, token) && await verifyOwner(expected, token)) setMessage('Stock adjustment is unconfirmed. Check the saved adjustment before making another change.');
    } finally {
      if (current(expected, token)) { busy.current = false; setSaving(false); }
    }
  }
  function adjustStock(product: Product, delta: number) {
    if (!ready || busy.current || held.current || pending || product.stock + delta < 0) return;
    void saveAdjustment({ id: crypto.randomUUID(), payload: { item_id: product.id, quantity_change: delta, expected_version: product.inventory_version } });
  }
  const lowStockCount = products.filter(product => product.stock <= 5).length;
  const unavailable = loading ? 'Loading…' : error || !ready ? 'Unavailable' : null;
  return <AppShell title="Inventory" subtitle="Recorded inventory for your business" statusItems={[
    { label: 'Products', value: unavailable ?? String(products.length), tone: ready ? 'neutral' : 'warn' },
    { label: 'Low Stock', value: unavailable ?? String(lowStockCount), tone: ready && lowStockCount === 0 ? 'good' : 'warn' },
  ]}>
    <section className="app-panel w-full max-w-4xl backdrop-blur-[30px] bg-white/70 border border-white/50 rounded-2xl">
      <div className="app-panel-header p-6 border-b border-gray-200/50">
        <div><h2 className="app-panel-title text-2xl font-bold">Products &amp; Variants</h2><p className="app-list-subtitle">Recorded stock from your inventory ledger.</p></div>
        <button disabled={saving || loading} onClick={() => void loadInventory()} className="app-button">Reload inventory</button>
      </div>
      {error && <p role="alert" className="p-6 text-red-600">{error}</p>}
      {message && <p role="status" className="p-4">{message}</p>}
      {pending && ready && <button disabled={saving} onClick={() => void saveAdjustment(pending, 'read')} className="app-button m-4">Check saved adjustment</button>}
      {pending && ready && retryAllowed && <button disabled={saving} onClick={() => void saveAdjustment(pending, 'write')} className="app-button m-4">Retry same adjustment</button>}
      {loading ? <p role="status" className="p-12 text-center">Loading inventory…</p> : !error && ready && products.length === 0 ? <p className="p-12 text-center">No products found in the inventory ledger.</p> : <div className="divide-y divide-gray-100">
        {products.map(product => <div key={product.id} className="p-6 flex items-center justify-between" data-testid={`product-row-${product.id}`}>
          <div className="flex-1"><h3 className="font-bold text-lg">{product.name}</h3><p className="text-sm text-gray-500">{product.description || 'No description'}</p>
            <span>{product.price_cents != null && product.currency ? new Intl.NumberFormat('en-US', { style: 'currency', currency: product.currency }).format(product.price_cents / 100) : 'Price unavailable'}</span>
            {product.stock <= 5 && <span className="app-badge warn ml-3">Low Stock</span>}
          </div>
          <div className="flex items-center gap-4">
            <button disabled={!ready || saving || !!pending || product.stock === 0} onClick={() => adjustStock(product, -1)} aria-label="Decrease stock" data-testid={`decrease-btn-${product.id}`} className="app-button disabled:opacity-50">−</button>
            <span className="font-bold text-xl" data-testid={`stock-count-${product.id}`}>{product.stock}</span>
            <button disabled={!ready || saving || !!pending || product.stock === 2147483647} onClick={() => adjustStock(product, 1)} aria-label="Increase stock" data-testid={`increase-btn-${product.id}`} className="app-button disabled:opacity-50">+</button>
          </div>
        </div>)}
      </div>}
    </section>
  </AppShell>;
}
