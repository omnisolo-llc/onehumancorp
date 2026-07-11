"use client";

import React, { useState, useEffect } from "react";
import { AppShell } from "../components/AppShell";
import { SyncManager } from "../../lib/sync/SyncManager";

export default function KitchenView() {
  const [orders, setOrders] = useState<any[]>([]);
  const [menu, setMenu] = useState<any[]>([]);
  const [offlineQueueCount, setOfflineQueueCount] = useState(0);

  useEffect(() => {
    // Ensure SyncManager is initialized so it listens to websocket
    SyncManager.getInstance();

    const fetchOrdersAndMenu = async () => {
      try {
        const tenantId = localStorage.getItem("tenant_id") || "default";

        // Fetch active orders (simulate using pos orders or ui orders if backend supports it)
        // Here we try to fetch orders via POS endpoint which might exist.
        // We'll just try to get anything, if fails, we show empty state correctly without fake data.
        const ordersRes = await fetch("/api/pos/orders", {
          headers: { "x-tenant-id": tenantId }
        });
        if (ordersRes.ok) {
          const data = await ordersRes.json();
          // Filter to 'new' or 'pending' orders if applicable
          setOrders(data.orders || data || []);
        }

        // Fetch products/menu via inventory
        const menuRes = await fetch("/api/pos/inventory", {
           headers: { "x-tenant-id": tenantId }
        });
        if (menuRes.ok) {
           const data = await menuRes.json();
           setMenu(data.items || data || []);
        }
      } catch (err) {
        console.error("Failed to fetch kitchen data", err);
      }
    };

    fetchOrdersAndMenu();

    const updateCount = async () => {
      setOfflineQueueCount(await SyncManager.getInstance().getQueueLength());
    };

    updateCount();
    window.addEventListener("ohc_queue_updated", updateCount);

    return () => {
      window.removeEventListener("ohc_queue_updated", updateCount);
    };
  }, []);

  const handleToggleSoldOut = async (itemId: string, currentStatus: boolean) => {
    // Optimistic UI update
    setMenu(menu.map(m => m.id === itemId ? { ...m, is_sold_out: !currentStatus } : m));

    // Add to sync queue for eventual consistency
    await SyncManager.getInstance().enqueue({
      id: `e2e-product-${itemId}`,
      type: "TOGGLE_SOLD_OUT",
      payload: { item_id: itemId, is_sold_out: !currentStatus },
      timestamp: Date.now()
    });
  };

  const handleMarkReady = async (orderId: string) => {
    // Optimistic UI update
    setOrders(orders.map(o => o.id === orderId ? { ...o, status: "ready" } : o));

    await SyncManager.getInstance().enqueue({
      id: `order-ready-${orderId}`,
      type: "UPDATE_ORDER_STATUS",
      payload: {
        order_id: orderId,
        status: "ready"
      },
      timestamp: Date.now()
    });
  };

  return (
    <AppShell title="Kitchen Command Center">
      <div className="min-h-screen bg-[#F5F5F7] text-[#1D1D1F] font-inter">
        <header className="bg-white/65 backdrop-blur-[30px] saturate-[210%] border-b border-white/40 sticky top-0 z-50 px-4 py-4 flex justify-between items-center">
          <h1 className="text-xl font-bold font-outfit">Kitchen Command Center</h1>
          <div id="queue-dashboard" className={offlineQueueCount > 0 ? "bg-[#FF9500]/20 text-[#FF9500] px-3 py-1 rounded-full text-sm font-medium border border-[#FF9500]/30" : "hidden"}>
            {offlineQueueCount} Pending Sync
          </div>
        </header>

        <main className="p-4 flex flex-col md:flex-row gap-6">
          <section className="flex-1">
            <h2 className="text-lg font-bold font-outfit mb-4">Active Orders</h2>
            <div className="space-y-4">
              {orders.filter(o => o.status !== "ready" && o.status !== "completed").map(order => (
                <div key={order.id} className="bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 p-4 shadow-sm">
                  <div className="flex justify-between items-start mb-2">
                    <h3 className="font-bold text-lg">Order #{order.id} - {order.customer_name || 'Guest'}</h3>
                    <span className="bg-[#0071E3]/10 text-[#0071E3] text-xs font-bold px-2 py-1 rounded">NEW</span>
                  </div>
                  <ul className="list-disc list-inside mb-3">
                    {order.items?.map((item: any, idx: number) => <li key={idx} className="text-sm">{item.name || item.product_id}</li>)}
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
                <div key={item.id} className="bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 p-4 shadow-sm flex items-center justify-between">
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
