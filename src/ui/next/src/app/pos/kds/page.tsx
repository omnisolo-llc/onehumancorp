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
    fetch('/api/v1/food-pre-order/list').then(res => res.json()).then(setOrders).catch(console.error);
    fetch('/api/pos/inventory').then(res => res.json()).then(setInventory).catch(console.error);
  }, []);

  // Background sync
  useEffect(() => {
    const syncInterval = setInterval(async () => {
      const events = OfflineStore.getEvents();
      if (events.length > 0 && navigator.onLine) {
        setSyncing(true);
        try {
          const orderEvents = events.filter((e: any) => e.type === 'UPDATE_ORDER_STATUS');
          const inventoryEvents = events.filter((e: any) => e.type === 'TOGGLE_SOLD_OUT');

          if (orderEvents.length > 0) {
            for (const orderEvent of orderEvents) {
               await fetch(`/api/v1/food-pre-order/${orderEvent.payload.order_id}/status`, {
                 method: 'POST',
                 headers: { 'Content-Type': 'application/json' },
                 body: JSON.stringify({ status: orderEvent.payload.status })
               });
            }
          }

          if (inventoryEvents.length > 0) {
            await fetch('/api/pos/inventory', {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify(inventoryEvents)
            });
          }

          OfflineStore.clearEvents();
        } catch (e) {
          console.error("Sync failed", e);
        } finally {
          setSyncing(false);
        }
      }
    }, 5000); // Try syncing every 5 seconds

    return () => clearInterval(syncInterval);
  }, []);

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
      preparing: 'Preparing',
      ready: 'Ready',
      received: 'Received',
      offline: 'Offline Mode',
      syncing: 'Syncing...'
    },
    ar: {
      kds: 'نظام عرض المطبخ',
      orders: 'الطلبات النشطة',
      inventory: 'عناصر القائمة',
      soldOut: 'نفذ',
      available: 'متوفر',
      preparing: 'يتم تحضيره',
      ready: 'جاهز',
      received: 'تم الاستلام',
      offline: 'وضع غير متصل بالشبكة',
      syncing: 'جاري المزامنة...'
    }
  };

  const texts = t[language];

  return (
    <div dir={language === 'ar' ? 'rtl' : 'ltr'} className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] h-[812px] bg-gradient-to-br from-white/40 to-white/10 backdrop-blur-xl shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-4 px-6 bg-white/65 backdrop-blur-[30px] shadow-sm border-b border-gray-200 sticky top-0 z-10 flex justify-between items-center">
          <div>
            <h1 className="text-xl font-bold text-gray-900">{texts.kds}</h1>
            {isOffline && <span className="text-red-500 font-bold text-sm bg-red-100 px-2 py-1 rounded-md">{texts.offline}</span>}
          </div>
          <button
            onClick={toggleLanguage}
            className="text-blue-600 font-bold px-3 py-1 bg-blue-50 rounded-lg hover:bg-blue-100 transition"
            data-testid="lang-toggle"
          >
            {language === 'en' ? 'عربي' : 'EN'}
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-4 pb-20">

          <h2 className="text-lg font-bold text-gray-800 mb-3">{texts.orders}</h2>
          <div className="flex flex-col gap-4 mb-6">
            {orders.map(order => (
              <div key={order.id} className="bg-white/65 backdrop-blur-[30px] rounded-2xl p-4 shadow-sm border border-gray-100">
                <div className="flex justify-between items-start mb-2">
                  <h3 className="font-bold text-lg text-gray-900">#{order.id} - {order.customer_name}</h3>
                  <span className={`px-2 py-1 rounded text-xs font-bold ${
                    order.status === 'Ready' ? 'bg-green-100 text-green-700' :
                    order.status === 'Preparing' ? 'bg-yellow-100 text-yellow-700' :
                    'bg-blue-100 text-blue-700'
                  }`}>
                    {order.status === 'Ready' ? texts.ready : order.status === 'Preparing' ? texts.preparing : texts.received}
                  </span>
                </div>
                <ul className="mb-4 text-gray-700 font-medium">
                  {(order.items || []).map((item: string, idx: number) => <li key={idx}>• {item}</li>)}
                </ul>
                {(order.notes || order.translated_notes) && (
                   <div className="mb-4 p-3 bg-blue-50 text-blue-800 rounded-lg text-sm font-semibold border border-blue-100">
                     <p className="opacity-70 text-xs mb-1 uppercase tracking-wider">{texts.customerNotes || "Customer Notes"}:</p>
                     <p className="mb-1">{order.notes}</p>
                     {order.translated_notes && <p className="text-blue-600 font-bold border-t border-blue-100/50 pt-1 mt-1">{order.translated_notes}</p>}
                   </div>
                )}
                <div className="grid grid-cols-2 gap-2">
                   {(!order.status || order.status.toLowerCase() === 'received' || order.status.toLowerCase() === 'pending') && (
                      <button
                        onClick={() => handleUpdateOrderStatus(order.id, 'Preparing')}
                        className="col-span-2 w-full py-4 bg-yellow-500 text-white font-bold text-lg rounded-xl shadow active:scale-95 transition"
                        data-testid={`btn-prepare-${order.id}`}
                      >
                        {texts.preparing}
                      </button>
                   )}
                   {order.status?.toLowerCase() === 'preparing' && (
                      <button
                        onClick={() => handleUpdateOrderStatus(order.id, 'Ready')}
                        className="col-span-2 w-full py-4 bg-green-500 text-white font-bold text-lg rounded-xl shadow active:scale-95 transition"
                        data-testid={`btn-ready-${order.id}`}
                      >
                        {texts.ready}
                      </button>
                   )}
                   {order.status?.toLowerCase() === 'ready' && (
                      <button
                         className="col-span-2 w-full py-4 bg-gray-300 text-gray-600 font-bold text-lg rounded-xl"
                         disabled
                      >
                         {texts.ready}
                      </button>
                   )}
                </div>
              </div>
            ))}
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
          <div className="absolute bottom-0 w-full bg-blue-500 text-white text-center py-2 text-sm font-bold animate-pulse z-20">
            {texts.syncing}
          </div>
        )}
      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
