"use client";

import { useEffect, useRef, useState } from 'react';
import { QUEUE_IDENTITY_EPOCH_KEY, sameOwner, type QueueOwner } from '@/lib/sync/queueIdentity';

type Material = { id: string; name: string; current_quantity: number; reorder_threshold: number };
type Vendor = { id: string; name: string; contact_info?: string | null };
type Supply = { raw_materials: Material[]; vendors: Vendor[] };
type Identity = QueueOwner & { expiresAt: number };
const unavailable = 'Supply records are unavailable. Reload to try again.';
const object = (value: unknown): value is Record<string, unknown> => !!value && typeof value === 'object' && !Array.isArray(value);
const quantity = (value: unknown): value is number => typeof value === 'number' && Number.isSafeInteger(value) && value >= 0 && value <= 2147483647;

function parseSupply(value: unknown): Supply {
  if (!object(value) || value.error != null || ('success' in value && value.success !== true) || !Array.isArray(value.raw_materials) || !Array.isArray(value.vendors)) throw new Error(unavailable);
  for (const [rows, kind] of [[value.raw_materials, 'material'], [value.vendors, 'vendor']] as const) {
    const ids = new Set<string>();
    for (const row of rows) {
      if (!object(row) || typeof row.id !== 'string' || !row.id.trim() || ids.has(row.id) || typeof row.name !== 'string' || !row.name.trim()) throw new Error(unavailable);
      if (kind === 'material' && (!quantity(row.current_quantity) || !quantity(row.reorder_threshold))) throw new Error(unavailable);
      if (kind === 'vendor' && row.contact_info != null && typeof row.contact_info !== 'string') throw new Error(unavailable);
      ids.add(row.id);
    }
  }
  return value as Supply;
}

// A read-only identity check must not invalidate the product ledger's shared
// queue lease while a stock adjustment is verifying its owner.
async function readIdentity(signal: AbortSignal): Promise<Identity> {
  const response = await fetch('/api/v1/auth/session-identity', { credentials: 'same-origin', cache: 'no-store', redirect: 'error', signal });
  if (response.status !== 200) throw new Error('Supply access could not be verified. Reload to verify your business.');
  const value: unknown = await response.json();
  if (!object(value) || value.error != null || ('success' in value && value.success !== true) || typeof value.userId !== 'string' || !value.userId || typeof value.tenantId !== 'string' || !value.tenantId || typeof value.expiresAt !== 'number' || !Number.isSafeInteger(value.expiresAt) || value.expiresAt <= Date.now()) throw new Error('Supply access could not be verified. Reload to verify your business.');
  return value as Identity;
}

export function SupplyRecords() {
  const [records, setRecords] = useState<Supply | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const generation = useRef(0);
  const active = useRef(false);
  const request = useRef<AbortController | null>(null);
  const expiry = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

  function retire(reason = 'Your session changed. Reload supply records to verify your business.') {
    generation.current += 1; request.current?.abort(); clearTimeout(expiry.current);
    setRecords(null); setLoading(false); setError(reason);
  }
  async function reload() {
    const token = ++generation.current;
    request.current?.abort(); clearTimeout(expiry.current);
    const controller = new AbortController(); request.current = controller;
    const current = () => active.current && generation.current === token && !controller.signal.aborted;
    setRecords(null); setLoading(true); setError('');
    try {
      const expected = await readIdentity(controller.signal);
      if (!current()) return;
      const response = await fetch('/api/v1/ui/supply', { headers: { 'x-ohc-expected-user': expected.userId, 'x-ohc-expected-tenant': expected.tenantId }, credentials: 'same-origin', cache: 'no-store', redirect: 'error', signal: controller.signal });
      if (response.status !== 200) throw new Error(unavailable);
      const result = parseSupply(await response.json());
      const verified = await readIdentity(controller.signal);
      if (!current()) return;
      if (!sameOwner(expected, verified)) { retire(); return; }
      const remaining = Math.min(expected.expiresAt, verified.expiresAt) - Date.now();
      if (remaining <= 0) { retire('Your session expired. Reload supply records to verify your business.'); return; }
      expiry.current = setTimeout(() => retire('Your session expired. Reload supply records to verify your business.'), Math.min(remaining, 2147483647));
      setRecords(result);
    } catch (cause) {
      if (current()) setError(cause instanceof Error && cause.message.startsWith('Supply access') ? cause.message : unavailable);
    } finally { if (current()) setLoading(false); }
  }

  useEffect(() => {
    active.current = true;
    const changed = () => retire();
    const storage = (event: StorageEvent) => { if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) retire(); };
    window.addEventListener('omnisolo_auth_changed', changed);
    window.addEventListener('storage', storage);
    window.addEventListener('pagehide', changed);
    void reload();
    return () => {
      active.current = false; generation.current += 1; request.current?.abort(); clearTimeout(expiry.current);
      window.removeEventListener('omnisolo_auth_changed', changed);
      window.removeEventListener('storage', storage);
      window.removeEventListener('pagehide', changed);
    };
  }, []);

  return <section aria-labelledby="supply-records-heading" className="w-full max-w-4xl min-w-0 space-y-4 mt-6">
    <div className="flex flex-wrap items-center justify-between gap-3">
      <div><h2 id="supply-records-heading" className="text-2xl font-bold">Supply records</h2><p className="app-list-subtitle">Recorded materials and supply partners for your business.</p></div>
      <button type="button" disabled={loading} onClick={() => void reload()} className="app-button min-h-[44px]">Reload supply records</button>
    </div>
    {loading && <p role="status">Loading supply records…</p>}
    {error && <p role="alert" className="text-red-600">{error}</p>}
    <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
      <section aria-labelledby="raw-materials-heading" className="app-panel min-w-0">
        <div className="app-panel-header"><h3 id="raw-materials-heading" className="app-panel-title">Raw Materials</h3></div>
        {records && (records.raw_materials.length === 0 ? <p className="app-empty">No raw materials recorded for this business.</p> : <ul className="divide-y divide-gray-100">
          {records.raw_materials.map(material => <li key={material.id} data-testid={`material-row-${material.id}`} className="p-4 space-y-2 break-words">
            <h4 className="font-bold">{material.name}</h4>
            <dl className="flex flex-wrap gap-x-6 gap-y-2 text-sm">
              <div><dt>Recorded quantity</dt><dd data-testid={`material-quantity-${material.id}`}>{material.current_quantity}</dd></div>
              <div><dt>Reorder threshold</dt><dd data-testid={`material-threshold-${material.id}`}>{material.reorder_threshold}</dd></div>
            </dl>
            <span className={`app-badge ${material.current_quantity <= material.reorder_threshold ? 'warn' : 'good'}`}>{material.current_quantity <= material.reorder_threshold ? 'Low Stock' : 'Healthy'}</span>
          </li>)}
        </ul>)}
      </section>
      <section aria-labelledby="vendors-heading" className="app-panel min-w-0">
        <div className="app-panel-header"><h3 id="vendors-heading" className="app-panel-title">Vendors</h3></div>
        {records && (records.vendors.length === 0 ? <p className="app-empty">No vendors recorded for this business.</p> : <ul className="divide-y divide-gray-100">
          {records.vendors.map(vendor => <li key={vendor.id} data-testid={`vendor-row-${vendor.id}`} className="p-4 space-y-2 break-words">
            <h4 className="font-bold">{vendor.name}</h4><p className="text-sm whitespace-pre-wrap break-words">{vendor.contact_info?.trim() || 'No contact information recorded.'}</p>
          </li>)}
        </ul>)}
      </section>
    </div>
  </section>;
}
