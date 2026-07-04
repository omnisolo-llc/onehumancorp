"use client";

import React, { useState, useEffect } from 'react';
import { SyncManager } from '../../lib/sync/SyncManager';

type Order = {
  id: string;
  customer_name?: string;
  total_amount?: number;
  status?: string;
  created_at?: string;
};

export default function OfflineOrdersPage() {
  const [orders, setOrders] = useState<Order[]>([]);
  const [isOffline, setIsOffline] = useState(false);
  const [syncing, setSyncing] = useState(false);

  // Network listener
  useEffect(() => {
    const handleOnline = () => {
      setIsOffline(false);
      setSyncing(true);
      SyncManager.getInstance().sync().finally(() => {
        setSyncing(false);
      });
    };
    const handleOffline = () => setIsOffline(true);

    if (typeof window !== 'undefined') {
        setIsOffline(!navigator.onLine);
        window.addEventListener('online', handleOnline);
        window.addEventListener('offline', handleOffline);

        return () => {
          window.removeEventListener('online', handleOnline);
          window.removeEventListener('offline', handleOffline);
        };
    }
  }, []);

  // Initial Data Load
  useEffect(() => {
    fetch('/api/ui/orders?tenant_id=default').then(res => res.json()).then(data => {
      setOrders(Array.isArray(data) ? data : []);
    }).catch(console.error);
  }, []);

  const handleCompleteOrder = async (orderId: string) => {
    // Optimistic UI Update
    setOrders(prev => prev.map(o => o.id === orderId ? { ...o, status: 'Completed' } : o));

    const event = {
      id: crypto.randomUUID ? crypto.randomUUID() : Math.random().toString(36).substring(7),
      type: 'UPDATE_ORDER_STATUS',
      payload: { order_id: orderId, status: 'Completed' },
      timestamp: Date.now(),
    };

    await SyncManager.getInstance().enqueue(event);
  };

  const handleToggleSoldOut = async (itemId: string, isSoldOut: boolean) => {
    const event = {
      id: crypto.randomUUID ? crypto.randomUUID() : Math.random().toString(36).substring(7),
      type: 'TOGGLE_SOLD_OUT',
      payload: { item_id: itemId, is_sold_out: isSoldOut },
      timestamp: Date.now(),
    };

    await SyncManager.getInstance().enqueue(event);
    // Note: Assuming a different list for inventory items, we'll just queue it for now.
    // In a complete implementation, we'd also optimistically update the inventory state.
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] max-w-[375px] mx-auto h-[812px] bg-gradient-to-br from-white/40 to-white/10 backdrop-blur-[30px] saturate-[210%] shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-4 px-6 bg-white/65 backdrop-blur-[30px] shadow-sm border-b border-gray-200 sticky top-0 z-10 flex justify-between items-center">
          <div>
            <h1 className="text-xl font-bold text-gray-900">Orders (Offline-First)</h1>
            {isOffline && <span className="text-[#FF3B30] font-bold text-sm bg-red-100 px-2 py-1 rounded-md" data-testid="offline-indicator">Offline ☁️</span>}
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-4 pb-20 flex flex-col gap-6">
          <div className="flex flex-col gap-4 mb-6">
            {orders.map(order => (
              <div key={order.id} className="app-card backdrop-blur-[30px] rounded-2xl p-4 shadow-sm border border-gray-100">
                <div className="flex justify-between items-start mb-2">
                  <h3 className="font-bold text-lg text-gray-900">#{order.id} - {order.customer_name}</h3>
                  <span className={`px-2 py-1 rounded text-xs font-bold ${
                    order.status === 'Completed' ? 'bg-green-100 text-green-700' : 'bg-blue-100 text-blue-700'
                  }`}>
                    {order.status || 'Pending'}
                  </span>
                </div>

                <div className="grid grid-cols-2 gap-2 mt-4">
                  {order.status !== 'Completed' && (
                    <button
                      onClick={() => handleCompleteOrder(order.id)}
                      className="col-span-2 w-full py-4 bg-[#34C759] text-white font-bold text-lg rounded-xl shadow active:scale-95 transition"
                      data-testid={`btn-complete-${order.id}`}
                    >
                      Complete Order
                    </button>
                  )}
                  {order.status === 'Completed' && (
                    <button
                      className="col-span-2 w-full py-4 bg-gray-300 text-gray-600 font-bold text-lg rounded-xl"
                      disabled
                    >
                      Completed
                    </button>
                  )}
                </div>
              </div>
            ))}
          </div>

          <div className="mt-8 border-t border-gray-200 pt-4">
             <h2 className="text-lg font-bold text-gray-800 mb-3">Inventory Actions</h2>
             <button
               onClick={() => handleToggleSoldOut('item-123', true)}
               className="w-full py-4 bg-[#FF3B30] text-white font-bold text-lg rounded-xl shadow active:scale-95 transition mb-2"
               data-testid="btn-sold-out"
             >
               Mark Item "Sold Out"
             </button>
          </div>
        </div>

        {/* Sync Indicator */}
        {syncing && (
          <div className="absolute bottom-0 w-full bg-[#0066FF] text-white text-center py-2 text-sm font-bold animate-pulse z-20" data-testid="sync-toast">
            Syncing changes...
          </div>
        )}
      </div>
    </div>
  );
}
