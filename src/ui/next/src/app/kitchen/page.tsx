"use client";
import type { OrderRecord, SaleProduct } from '@/lib/business-records';

import React, { useState, useEffect, useRef } from "react";
import { AppShell } from "../components/AppShell";
import { SyncManager } from "../../lib/sync/SyncManager";
import { sameOwner } from '../../lib/sync/queueIdentity';
import { openOnboardingSession, onboardingOwner, onboardingSessionEpoch, subscribeOnboardingInvalidation, fetchForOwnedBusinessRead, readOwnedOnboardingItem, writeOwnedOnboardingItem, type DraftOwner } from '../onboarding/draftSession';

type KitchenScope = { owner: DraftOwner; epoch: number };
function currentScope(scope: KitchenScope | null): scope is KitchenScope {
  const owner = onboardingOwner();
  return !!scope && !!owner && scope.epoch === onboardingSessionEpoch() && sameOwner(scope.owner, owner);
}
function assertScope(scope: KitchenScope): void {
  if (!currentScope(scope)) throw new Error('Your session changed. Reopen Kitchen to verify its saved data.');
}
function records<T extends OrderRecord | SaleProduct>(value: unknown): T[] {
  if (!Array.isArray(value) || value.some(row => {
    if (!row || typeof row !== 'object' || Array.isArray(row) || typeof row.id !== 'string' || !row.id) return true;
    for (const key of ['status','customer_name','notes','translated_notes','name','title','updated_at']) if (row[key] != null && typeof row[key] !== 'string') return true;
    if (row.is_sold_out != null && typeof row.is_sold_out !== 'boolean') return true;
    if (row.items != null && (!Array.isArray(row.items) || row.items.some((item: unknown) => !item || typeof item !== 'object' || Array.isArray(item) || ['name','product_id'].some(key => (item as Record<string, unknown>)[key] != null && typeof (item as Record<string, unknown>)[key] !== 'string')))) return true;
    return false;
  })) throw new Error('Saved kitchen data could not be read. Its original copy remains held.');
  return value as T[];
}
async function loadSnapshot<T extends OrderRecord | SaleProduct>(scope: KitchenScope, kind: 'orders' | 'inventory', active: () => boolean): Promise<{ value: T[] | null; notice?: string }> {
  const key = 'kitchen-' + kind + '-v1';
  const assertActive = () => { assertScope(scope); if (!active()) throw new Error('This kitchen view is no longer active.'); };
  try {
    const response = await fetchForOwnedBusinessRead('/api/v1/pos/' + kind, scope.owner);
    assertActive();
    if (!response.ok || response.status !== 200) throw new Error('Kitchen data unavailable');
    const body = await response.json();
    assertActive();
    if (body?.success === false || body?.error != null) throw new Error('Kitchen data unavailable');
    const value = records<T>(Array.isArray(body) ? body : kind === 'orders' ? body?.orders : body?.inventory ?? body?.items);
    try { writeOwnedOnboardingItem(key, JSON.stringify({ format: 1, records: value })); }
    catch { return { value, notice: 'Fresh kitchen data is shown, but this device could not save its cache.' }; }
    return { value };
  } catch {
    assertActive();
    const saved = readOwnedOnboardingItem(key);
    if (saved === null) return { value: null, notice: `Kitchen ${kind} could not be loaded. No saved data is available for this account.` };
    let parsed: { format?: unknown; records?: unknown };
    try { parsed = JSON.parse(saved); } catch { throw new Error('Saved kitchen data could not be read. Its original copy remains held.'); }
    if (parsed?.format !== 1) throw new Error('Saved kitchen data could not be read. Its original copy remains held.');
    return { value: records<T>(parsed.records), notice: `Showing saved ${kind} for this account while fresh kitchen data is unavailable.` };
  }
}

export default function KitchenView() {
  const [orders, setOrders] = useState<OrderRecord[]>([]);
  const [menu, setMenu] = useState<SaleProduct[]>([]);
  const [offlineQueueCount, setOfflineQueueCount] = useState(0);
  const [queueError, setQueueError] = useState<string | null>(null);
  const pending = useRef(new Set<string>());
  const scope = useRef<KitchenScope | null>(null);
  const [viewScope, setViewScope] = useState<KitchenScope | null>(null);
  const [loaded, setLoaded] = useState(false);
  const [sessionError, setSessionError] = useState('');
  const [cacheNotice, setCacheNotice] = useState('');
  const [ordersKnown, setOrdersKnown] = useState(false);
  const [menuKnown, setMenuKnown] = useState(false);
  const generation = useRef(0);

  useEffect(() => {
    let disposed = false;
    let countVersion = 0;
    const updateCount = async () => {
      const current = scope.current; const version = ++countVersion;
      if (!currentScope(current)) return;
      try {
        const count = await SyncManager.getInstance().getQueueLength();
        if (!disposed && currentScope(current) && version === countVersion) setOfflineQueueCount(count);
      } catch { if (!disposed && currentScope(current) && version === countVersion) setQueueError('The offline queue could not be read. Saved actions remain held.'); }
    };
    const load = async () => {
      const version = ++generation.current;
      setLoaded(false); setSessionError(''); setCacheNotice('');
      const active = () => !disposed && version === generation.current;
      try {
        const owner = await openOnboardingSession();
        if (!active()) return;
        const verified = { owner, epoch: onboardingSessionEpoch() };
        scope.current = verified; setViewScope(verified);
        const [nextOrders, nextMenu] = await Promise.all([loadSnapshot<OrderRecord>(verified, 'orders', active), loadSnapshot<SaleProduct>(verified, 'inventory', active)]);
        if (!active() || !currentScope(verified)) return;
        setOrders(nextOrders.value ?? []); setMenu(nextMenu.value ?? []);
        setOrdersKnown(nextOrders.value !== null); setMenuKnown(nextMenu.value !== null);
        const held = ['kds_orders_cache','kds_menu_cache'].some(key => localStorage.getItem(key) !== null);
        setCacheNotice([...new Set([held ? 'Older kitchen data remains held because its owner is unknown.' : '', nextOrders.notice, nextMenu.notice].filter(Boolean))].join(' '));
        setLoaded(true); void updateCount();
      } catch (error) {
        if (active()) setSessionError(error instanceof Error ? error.message : 'Kitchen data is unavailable. Your saved copies remain held.');
      }
    };
    const invalidate = (restart: boolean) => {
      generation.current += 1; countVersion += 1; scope.current = null; setViewScope(null); pending.current.clear();
      setOrders([]); setMenu([]); setOrdersKnown(false); setMenuKnown(false); setOfflineQueueCount(0); setQueueError(null); setCacheNotice(''); setLoaded(false);
      if (restart) void load(); else setSessionError('Your session could not be verified. Saved kitchen data remains held.');
    };
    const unsubscribe = subscribeOnboardingInvalidation(invalidate);
    window.addEventListener('omnisolo_queue_updated', updateCount);
    void load();
    return () => { disposed = true; generation.current += 1; countVersion += 1; scope.current = null; unsubscribe(); window.removeEventListener('omnisolo_queue_updated', updateCount); };
  }, []);

  const handleToggleSoldOut = async (itemId: string, currentStatus: boolean) => {
    const current = viewScope; const version = generation.current;
    if (!currentScope(current)) return;
    const active = () => version === generation.current && currentScope(current);
    const key = `product:${itemId}`;
    const original = menu.find(item => item.id === itemId);
    if (pending.current.has(key)) return;
    pending.current.add(key); setQueueError(null);
    setMenu(current => current.map(item => item.id === itemId ? { ...item, is_sold_out: !currentStatus } : item));
    try {
      await SyncManager.getInstance().enqueue({ id: crypto.randomUUID(), type: "TOGGLE_SOLD_OUT", payload: { item_id: itemId, is_sold_out: !currentStatus, expected_is_sold_out: currentStatus, ...(original?.updated_at ? { expected_updated_at: original.updated_at } : {}), ...(original?.base_version !== undefined || original?.version !== undefined ? { base_version: original.base_version ?? original.version } : {}) }, timestamp: Date.now() }, current.owner);
    } catch {
      if (!active()) return;
      setMenu(current => current.map(item => item.id === itemId ? { ...item, is_sold_out: currentStatus } : item));
      setQueueError('This change could not be saved. Check your connection and local storage.');
    } finally { if (active()) pending.current.delete(key); }
  };

  const handleMarkReady = async (orderId: string) => {
    const current = viewScope; const version = generation.current;
    if (!currentScope(current)) return;
    const active = () => version === generation.current && currentScope(current);
    const key = `order:${orderId}`;
    const original = orders.find(order => order.id === orderId);
    if (!original || pending.current.has(key)) return;
    pending.current.add(key); setQueueError(null);
    setOrders(current => current.map(order => order.id === orderId ? { ...order, status: "ready" } : order));
    try {
      await SyncManager.getInstance().enqueue({ id: crypto.randomUUID(), type: "UPDATE_ORDER_STATUS", payload: { order_id: orderId, status: "ready", expected_status: original.status, ...(original.updated_at ? { expected_updated_at: original.updated_at } : {}), ...(original.base_version !== undefined || original.version !== undefined ? { base_version: original.base_version ?? original.version } : {}) }, timestamp: Date.now() }, current.owner);
    } catch {
      if (!active()) return;
      setOrders(current => current.map(order => order.id === orderId ? { ...order, status: original.status } : order));
      setQueueError('This change could not be saved. Check your connection and local storage.');
    } finally { if (active()) pending.current.delete(key); }
  };

  if (sessionError) return <AppShell title="Kitchen Command Center"><div role="alert" className="app-panel p-4">{sessionError}</div></AppShell>;
  if (!loaded) return <AppShell title="Kitchen Command Center"><div role="status" className="app-panel p-4">Verifying your kitchen session…</div></AppShell>;
  return (
    <AppShell title="Kitchen Command Center">
      <div className="min-h-screen bg-[#F5F5F7] text-[#1D1D1F] font-inter">
        <header className="bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] border-b border-[rgba(255,255,255,0.4)] sticky top-0 z-50 px-4 py-4 flex justify-between items-center">
          <div className="text-xl font-bold font-outfit">Kitchen Command Center</div>
          <div id="queue-dashboard" className={offlineQueueCount > 0 ? "bg-[#FF9500]/20 text-[#FF9500] px-3 py-1 rounded-full text-sm font-medium border border-[#FF9500]/30" : "hidden"}>
            {offlineQueueCount} Pending Sync
          </div>
        </header>

        {queueError && <p role="alert">{queueError}</p>}
        {cacheNotice && <p role="status">{cacheNotice}</p>}
        <main className="p-4 flex flex-col md:flex-row gap-6">
          <section className="flex-1">
            <h2 className="text-lg font-bold font-outfit mb-4">Active Orders</h2>
            {!ordersKnown && <p>Orders are unavailable.</p>}
            <div className="space-y-4">
              {orders.filter(o => o.status !== "ready" && o.status !== "completed").map(order => (
                <div key={order.id} data-testid={`kitchen-order-${order.id}`} className="bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] p-4 shadow-sm">
                  <div className="flex justify-between items-start mb-2">
                    <h3 className="font-bold text-lg">Order #{order.id} - {order.customer_name || 'Guest'}</h3>
                    <span className="bg-[#0071E3]/10 text-[#0071E3] text-xs font-bold px-2 py-1 rounded">NEW</span>
                  </div>
                  <ul className="list-disc list-inside mb-3">
                    {order.items?.map((item, idx) => <li key={idx} className="text-sm">{item.name || item.product_id}</li>)}
                  </ul>
                  {order.notes && (
                    <div className="bg-[#FF9500]/10 border border-[#FF9500]/20 rounded-lg p-3 mb-4">
                      <p className="text-sm font-medium text-[#FF9500] mb-1">Customer Notes:</p>
                      <p className="text-sm italic mb-2">"{order.notes}"</p>
                      {order.translated_notes && (
                        <>
                          <p className="text-sm font-medium text-[#0071E3] mb-1 mt-2">AI Translation:</p>
                          <p className="text-sm font-bold text-lg" dir="rtl">
                            {order.translated_notes}
                          </p>
                        </>
                      )}
                    </div>
                  )}
                  <button
                    onClick={() => handleMarkReady(order.id)}
                    className="w-full h-[44px] min-h-[44px] bg-[#34C759] text-white font-bold text-lg shadow-sm active:scale-95 transition-transform"
                  >
                    Mark Ready & Notify
                  </button>
                </div>
              ))}
              {ordersKnown && orders.filter(o => o.status !== "ready" && o.status !== "completed").length === 0 && (
                <div className="text-center py-8 text-gray-500 italic">No active orders</div>
              )}
            </div>
          </section>

          <section className="w-full md:w-80">
            <h2 className="text-lg font-bold font-outfit mb-4">Daily Menu</h2>
            {!menuKnown && <p>Menu data is unavailable.</p>}
            <div className="space-y-3">
              {menu.map(item => {
                const soldOut = item.is_sold_out || item.available_quantity === 0;
                return (
                <div key={item.id} className="bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] p-4 shadow-sm flex items-center justify-between">
                  <h3 className={`font-bold font-outfit text-lg ${soldOut ? "text-gray-400 line-through" : "text-[#1D1D1F]"}`}>
                    {item.name || item.title}
                  </h3>
                  <button
                    id={`sold-out-toggle-${item.id}`}
                    onClick={() => handleToggleSoldOut(item.id, soldOut)}
                    className={`min-h-[44px] min-w-[44px] h-[44px] px-4 font-bold text-sm transition-colors ${
                      soldOut
                        ? "bg-[#FF3B30]/10 text-[#FF3B30] border border-[#FF3B30]/20"
                        : "bg-[#0071E3]/10 text-[#0071E3] border border-[#0071E3]/20 hover:bg-[#0071E3]/20"
                    }`}
                  >
                    {soldOut ? "Sold Out" : "Mark Sold Out"}
                  </button>
                </div>
              )})}
            </div>
          </section>
        </main>
      </div>
    </AppShell>
  );
}
