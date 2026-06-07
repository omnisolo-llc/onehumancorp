"use client";

import React, { useState, useEffect } from 'react';

// Offline storage helper for KDS data
const OfflineStore = {
  getEvents: () => JSON.parse(localStorage.getItem('ohc_kds_events') || '[]'),
  addEvent: (event: any) => {
    const events = OfflineStore.getEvents();
    events.push(event);
    localStorage.setItem('ohc_kds_events', JSON.stringify(events));
  },
  clearEvents: () => localStorage.setItem('ohc_kds_events', '[]'),
};

export default function KDSPage() {
  const [orders, setOrders] = useState<any[]>([]);
  const [inventory, setInventory] = useState<any[]>([]);
  const [language, setLanguage] = useState<'en' | 'ar'>('en');
  const [isOffline, setIsOffline] = useState(false);
  const [syncing, setSyncing] = useState(false);

  // Agent suggestion state
  const [suggestedSoldOut, setSuggestedSoldOut] = useState<{itemId: string, name: string} | null>(null);

  // Network listener
  useEffect(() => {
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);

    // Set initial state safely
    setIsOffline(!navigator.onLine);

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  // Initial Data Load
  useEffect(() => {
    fetch('/api/pos/orders').then(res => res.json()).then(setOrders).catch(console.error);
    fetch('/api/pos/inventory').then(res => res.json()).then(setInventory).catch(console.error);
  }, []);

  // Background sync & polling for new orders (simulating real-time push)
  useEffect(() => {
    const syncInterval = setInterval(async () => {
      const events = OfflineStore.getEvents();
      if (navigator.onLine) {
        setSyncing(true);
        try {
          // Sync offline events
          if (events.length > 0) {
            const orderEvents = events.filter((e: any) => e.type === 'UPDATE_ORDER_STATUS');
            const inventoryEvents = events.filter((e: any) => e.type === 'TOGGLE_SOLD_OUT');

            if (orderEvents.length > 0) {
              await fetch('/api/pos/orders', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(orderEvents)
              });
            }

            if (inventoryEvents.length > 0) {
              await fetch('/api/pos/inventory', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(inventoryEvents)
              });
            }

            OfflineStore.clearEvents();
          }

          // Poll for new orders & inventory
          const [ordersRes, invRes] = await Promise.all([
            fetch('/api/pos/orders'),
            fetch('/api/pos/inventory')
          ]);

          const newOrders = await ordersRes.json();
          const newInv = await invRes.json();

          // Agentic Logic: Check if we have multiple recent orders for the same item and it's not sold out
          // This is a naive simulation of an anomaly detection agent
          if (newOrders.length > orders.length && !suggestedSoldOut) {
            const latestOrder = newOrders[newOrders.length - 1];
            if (latestOrder.items && latestOrder.items.length > 0) {
              const itemStr = latestOrder.items[0];
              const invMatch = newInv.find((i: any) => itemStr.includes(i.name_en));
              if (invMatch && !invMatch.is_sold_out && latestOrder.items.length >= 3) { // Trigger if they order 3+ items
                setSuggestedSoldOut({ itemId: invMatch.id, name: language === 'en' ? invMatch.name_en : invMatch.name_ar });
              }
            }
          }

          setOrders(newOrders);
          setInventory(newInv);

        } catch (e) {
          console.error("Sync failed", e);
        } finally {
          setSyncing(false);
        }
      }
    }, 5000); // Try syncing every 5 seconds

    return () => clearInterval(syncInterval);
  }, [orders.length, suggestedSoldOut, language]);

  const handleUpdateOrderStatus = (orderId: string, newStatus: string) => {
    // Optimistic UI Update
    setOrders(prev => prev.map(o => o.id === orderId ? { ...o, status: newStatus } : o));

    const event = {
      type: 'UPDATE_ORDER_STATUS',
      payload: { order_id: orderId, status: newStatus },
      timestamp: new Date().toISOString(),
    };

    OfflineStore.addEvent(event);
  };

  const handleToggleSoldOut = (itemId: string, isSoldOut: boolean) => {
    // Optimistic UI Update
    setInventory(prev => prev.map(i => i.id === itemId ? { ...i, is_sold_out: isSoldOut } : i));

    const event = {
      type: 'TOGGLE_SOLD_OUT',
      payload: { item_id: itemId, is_sold_out: isSoldOut },
      timestamp: new Date().toISOString(),
    };

    OfflineStore.addEvent(event);
    if (suggestedSoldOut && suggestedSoldOut.itemId === itemId) {
      setSuggestedSoldOut(null);
    }
  };

  const toggleLanguage = () => {
    setLanguage(prev => prev === 'en' ? 'ar' : 'en');
  };

  const t = {
    en: {
      kds: 'Kitchen Display System',
      orders: 'Active Orders',
      inventory: 'Menu Items',
      soldOut: 'Sold Out',
      available: 'Available',
      pending: 'Pending',
      preparing: 'Preparing',
      ready: 'Ready',
      received: 'Received',
      completed: 'Completed',
      offline: 'Offline Mode',
      syncing: 'Syncing...',
      notes: 'Notes',
      agentSuggestion: 'Operations Agent: High demand detected. Mark sold out?',
      yes: 'Yes',
      dismiss: 'Dismiss'
    },
    ar: {
      kds: 'نظام عرض المطبخ',
      orders: 'الطلبات النشطة',
      inventory: 'عناصر القائمة',
      soldOut: 'نفذ',
      available: 'متوفر',
      pending: 'قيد الانتظار',
      preparing: 'يتم تحضيره',
      ready: 'جاهز',
      received: 'تم الاستلام',
      completed: 'مكتمل',
      offline: 'وضع غير متصل بالشبكة',
      syncing: 'جاري المزامنة...',
      notes: 'ملاحظات',
      agentSuggestion: 'وكيل العمليات: طلب عالٍ مكتشف. هل تريد تعليمه كنفذ؟',
      yes: 'نعم',
      dismiss: 'تجاهل'
    }
  };

  const texts = t[language];

  return (
    <div dir={language === 'ar' ? 'rtl' : 'ltr'} className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10 px-4">
      <div className="w-full max-w-[375px] h-[812px] bg-gradient-to-br from-white/40 to-white/10 backdrop-blur-xl shadow-2xl overflow-hidden flex flex-col relative border border-gray-200 rounded-3xl">

        {/* Header */}
        <div className="pt-12 pb-4 px-6 bg-white/65 backdrop-blur-[30px] shadow-sm border-b border-gray-200 sticky top-0 z-10 flex justify-between items-center">
          <div>
            <h1 className="text-xl font-bold text-gray-900">{texts.kds}</h1>
            {isOffline && <span className="text-red-500 font-bold text-sm bg-red-100 px-2 py-1 rounded-md mt-1 inline-block">{texts.offline}</span>}
          </div>
          <button
            onClick={toggleLanguage}
            className="text-blue-600 font-bold px-3 py-1 bg-blue-50 rounded-lg hover:bg-blue-100 transition"
            data-testid="lang-toggle"
          >
            {language === 'en' ? 'عربي' : 'EN'}
          </button>
        </div>

        {/* Agent Suggestion Overlay */}
        {suggestedSoldOut && (
          <div className="mx-4 mt-4 bg-blue-600 text-white p-4 rounded-2xl shadow-lg flex flex-col gap-3 animate-fade-in z-20" data-testid="agent-suggestion">
            <div className="flex items-start gap-3">
              <div className="bg-white/20 p-2 rounded-full">
                <svg className="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>
              </div>
              <div>
                <p className="font-bold text-sm">{texts.agentSuggestion}</p>
                <p className="text-blue-100 text-xs mt-1 font-medium">{suggestedSoldOut.name}</p>
              </div>
            </div>
            <div className="flex gap-2">
              <button
                onClick={() => handleToggleSoldOut(suggestedSoldOut.itemId, true)}
                className="flex-1 bg-white text-blue-600 font-bold py-2 rounded-xl text-sm"
                data-testid="agent-accept"
              >
                {texts.yes}
              </button>
              <button
                onClick={() => setSuggestedSoldOut(null)}
                className="flex-1 bg-blue-700 text-white font-bold py-2 rounded-xl text-sm"
              >
                {texts.dismiss}
              </button>
            </div>
          </div>
        )}

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-4 pb-20">

          <h2 className="text-lg font-bold text-gray-800 mb-3">{texts.orders}</h2>
          <div className="flex flex-col gap-4 mb-6">
            {orders.filter(o => o.status !== 'completed').map(order => (
              <div key={order.id} className="bg-white/65 backdrop-blur-[30px] rounded-2xl p-4 shadow-sm border border-gray-100">
                <div className="flex justify-between items-start mb-2">
                  <h3 className="font-bold text-lg text-gray-900">#{order.id.replace('ord_', '').slice(-4)} - {order.customer_name}</h3>
                  <span className={`px-2 py-1 rounded text-xs font-bold ${
                    order.status === 'Ready' || order.status === 'ready_for_pickup' ? 'bg-green-100 text-green-700' :
                    order.status === 'Preparing' || order.status === 'preparing' ? 'bg-yellow-100 text-yellow-700' :
                    'bg-blue-100 text-blue-700'
                  }`}>
                    {order.status === 'Ready' || order.status === 'ready_for_pickup' ? texts.ready :
                     order.status === 'Preparing' || order.status === 'preparing' ? texts.preparing :
                     order.status === 'pending' ? texts.pending : texts.received}
                  </span>
                </div>

                {order.customer_note && (
                  <div className="mb-3 bg-amber-50 border border-amber-100 rounded-lg p-2 text-sm text-amber-800 font-medium flex gap-2">
                    <svg className="w-4 h-4 mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"/></svg>
                    <span data-testid={`note-${order.id}`}>{order.customer_note}</span>
                  </div>
                )}

                <ul className="mb-4 text-gray-700 font-medium text-sm">
                  {order.items.map((item: string, idx: number) => <li key={idx}>• {item}</li>)}
                </ul>
                <div className="grid grid-cols-2 gap-2">
                   {(order.status === 'Received' || order.status === 'pending') && (
                      <button
                        onClick={() => handleUpdateOrderStatus(order.id, 'preparing')}
                        className="col-span-2 w-full py-4 bg-yellow-500 text-white font-bold text-lg rounded-xl shadow active:scale-95 transition"
                        data-testid={`btn-prepare-${order.id}`}
                      >
                        {texts.preparing}
                      </button>
                   )}
                   {(order.status === 'Preparing' || order.status === 'preparing') && (
                      <button
                        onClick={() => handleUpdateOrderStatus(order.id, 'ready_for_pickup')}
                        className="col-span-2 w-full py-4 bg-green-500 text-white font-bold text-lg rounded-xl shadow active:scale-95 transition"
                        data-testid={`btn-ready-${order.id}`}
                      >
                        {texts.ready}
                      </button>
                   )}
                   {(order.status === 'Ready' || order.status === 'ready_for_pickup') && (
                      <button
                         onClick={() => handleUpdateOrderStatus(order.id, 'completed')}
                         className="col-span-2 w-full py-4 bg-gray-800 text-white font-bold text-lg rounded-xl shadow active:scale-95 transition"
                         data-testid={`btn-complete-${order.id}`}
                      >
                         {texts.completed}
                      </button>
                   )}
                </div>
              </div>
            ))}
            {orders.filter(o => o.status !== 'completed').length === 0 && (
              <div className="text-center text-gray-500 py-8 bg-white/40 rounded-2xl border border-gray-100 border-dashed">
                No active orders
              </div>
            )}
          </div>

          <h2 className="text-lg font-bold text-gray-800 mb-3">{texts.inventory}</h2>
          <div className="flex flex-col gap-3">
             {inventory.map(item => (
                <div key={item.id} className="bg-white/65 backdrop-blur-[30px] rounded-xl p-4 shadow-sm border border-gray-100 flex justify-between items-center">
                   <span className="font-bold text-gray-800">{language === 'en' ? item.name_en : item.name_ar}</span>
                   <button
                     onClick={() => handleToggleSoldOut(item.id, !item.is_sold_out)}
                     className={`px-6 py-3 rounded-lg font-bold shadow active:scale-95 transition ${item.is_sold_out ? 'bg-red-500 text-white' : 'bg-green-100 text-green-700'}`}
                     data-testid={`toggle-soldout-${item.id}`}
                   >
                     {item.is_sold_out ? texts.soldOut : texts.available}
                   </button>
                </div>
             ))}
          </div>

        </div>

        {/* Sync Indicator */}
        {syncing && (
          <div className="absolute bottom-0 w-full bg-blue-500 text-white text-center py-2 text-sm font-bold z-20 flex items-center justify-center gap-2">
            <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
            {texts.syncing}
          </div>
        )}
      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .animate-fade-in { animation: fadeIn 0.3s ease-out forwards; }
        @keyframes fadeIn { from { opacity: 0; transform: translateY(-10px); } to { opacity: 1; transform: translateY(0); } }
      `}} />
    </div>
  );
}
