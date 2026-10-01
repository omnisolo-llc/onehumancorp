"use client";
import type { OrderRecord, SaleProduct } from '@/lib/business-records';

import React, { useState, useEffect, useRef } from "react";
import { AppShell } from "../components/AppShell";
import { SyncManager } from "../../lib/sync/SyncManager";

export default function KitchenView() {
  const [orders, setOrders] = useState<OrderRecord[]>([]);
  const [menu, setMenu] = useState<SaleProduct[]>([]);
  const [offlineQueueCount, setOfflineQueueCount] = useState(0);
  const [queueError, setQueueError] = useState<string | null>(null);
  const pending = useRef(new Set<string>());

  useEffect(() => {
    // Ensure SyncManager is initialized so it listens to websocket
    SyncManager.getInstance();

    const fetchOrdersAndMenu = async () => {
      try {
        // Fetch recorded orders; an authoritative empty response stays empty.
        let ordersData: OrderRecord[] = [];
        let ordersLoaded = false;
        try {
          const ordersRes = await fetch("/api/v1/pos/orders");
          if (ordersRes.ok) {
            const data = await ordersRes.json();
            ordersData = data.orders || (Array.isArray(data) ? data : []);
            ordersLoaded = true;
          }
        } catch {
          // ignore
        }
        if (!ordersLoaded) {
          const cachedOrders = localStorage.getItem('kds_orders_cache');
          if (cachedOrders) {
            try { ordersData = JSON.parse(cachedOrders); } catch { ordersData = []; }
          }
        }
        setOrders(ordersData);
        try { localStorage.setItem('kds_orders_cache', JSON.stringify(ordersData)); } catch {
          // ignore cache errors
        }

        let menuData: SaleProduct[] = [];
        let menuLoaded = false;
        try {
          const menuRes = await fetch("/api/v1/pos/inventory");
          if (menuRes.ok) {
            const data = await menuRes.json();
            menuData = data.inventory || data.items || (Array.isArray(data) ? data : []);
            menuLoaded = true;
          }
        } catch {
          // ignore
        }
        if (!menuLoaded) {
          const cachedMenu = localStorage.getItem('kds_menu_cache');
          if (cachedMenu) {
            try { menuData = JSON.parse(cachedMenu); } catch { menuData = []; }
          }
        }
        setMenu(menuData);
        try { localStorage.setItem('kds_menu_cache', JSON.stringify(menuData)); } catch {
          // ignore cache errors
        }
      } catch (err) {
        console.error("Failed to fetch kitchen data", err);
      }
    };

    fetchOrdersAndMenu();

    const updateCount = async () => {
      try { setOfflineQueueCount(await SyncManager.getInstance().getQueueLength()); }
      catch { setQueueError('The offline queue could not be read.'); }
    };

    updateCount();
    window.addEventListener("omnisolo_queue_updated", updateCount);

    return () => {
      window.removeEventListener("omnisolo_queue_updated", updateCount);
    };
  }, []);

  const handleToggleSoldOut = async (itemId: string, currentStatus: boolean) => {
    const key = `product:${itemId}`;
    const original = menu.find(item => item.id === itemId);
    if (pending.current.has(key)) return;
    pending.current.add(key); setQueueError(null);
    setMenu(current => current.map(item => item.id === itemId ? { ...item, is_sold_out: !currentStatus } : item));
    try {
      await SyncManager.getInstance().enqueue({ id: crypto.randomUUID(), type: "TOGGLE_SOLD_OUT", payload: { item_id: itemId, is_sold_out: !currentStatus, expected_is_sold_out: currentStatus, ...(original?.updated_at ? { expected_updated_at: original.updated_at } : {}), ...(original?.base_version !== undefined || original?.version !== undefined ? { base_version: original.base_version ?? original.version } : {}) }, timestamp: Date.now() });
    } catch {
      setMenu(current => current.map(item => item.id === itemId ? { ...item, is_sold_out: currentStatus } : item));
      setQueueError('This change could not be saved. Check your connection and local storage.');
    } finally { pending.current.delete(key); }
  };

  const handleMarkReady = async (orderId: string) => {
    const key = `order:${orderId}`;
    const original = orders.find(order => order.id === orderId);
    if (!original || pending.current.has(key)) return;
    pending.current.add(key); setQueueError(null);
    setOrders(current => current.map(order => order.id === orderId ? { ...order, status: "ready" } : order));
    try {
      await SyncManager.getInstance().enqueue({ id: crypto.randomUUID(), type: "UPDATE_ORDER_STATUS", payload: { order_id: orderId, status: "ready", expected_status: original.status, ...(original.updated_at ? { expected_updated_at: original.updated_at } : {}), ...(original.base_version !== undefined || original.version !== undefined ? { base_version: original.base_version ?? original.version } : {}) }, timestamp: Date.now() });
    } catch {
      setOrders(current => current.map(order => order.id === orderId ? { ...order, status: original.status } : order));
      setQueueError('This change could not be saved. Check your connection and local storage.');
    } finally { pending.current.delete(key); }
  };

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
        <main className="p-4 flex flex-col md:flex-row gap-6">
          <section className="flex-1">
            <h2 className="text-lg font-bold font-outfit mb-4">Active Orders</h2>
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
              {orders.filter(o => o.status !== "ready" && o.status !== "completed").length === 0 && (
                <div className="text-center py-8 text-gray-500 italic">No active orders</div>
              )}
            </div>
          </section>

          <section className="w-full md:w-80">
            <h2 className="text-lg font-bold font-outfit mb-4">Daily Menu</h2>
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
