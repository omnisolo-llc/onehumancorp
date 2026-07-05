"use client";

import React, { useState, useEffect } from 'react';
import { SyncManager } from '../../../lib/sync/SyncManager';

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
    fetch('/api/pos/orders').then(res => res.json()).then(setOrders).catch(console.error);
    fetch('/api/pos/inventory').then(res => res.json()).then(setInventory).catch(console.error);
  }, []);

  const handleUpdateOrderStatus = async (orderId: string, newStatus: string) => {
    // Optimistic UI Update
    setOrders(prev => prev.map(o => o.id === orderId ? { ...o, status: newStatus } : o));

    const event = {
      type: 'UPDATE_ORDER_STATUS',
      payload: { order_id: orderId, status: newStatus },
      timestamp: new Date().toISOString(),
    };

    await SyncManager.getInstance().enqueue(event);
  };

  const handleToggleSoldOut = async (itemId: string, isSoldOut: boolean) => {
    // Optimistic UI Update
    setInventory(prev => prev.map(i => i.id === itemId ? { ...i, is_sold_out: isSoldOut } : i));

    const event = {
      type: 'TOGGLE_SOLD_OUT',
      payload: { item_id: itemId, is_sold_out: isSoldOut },
      timestamp: new Date().toISOString(),
    };

    await SyncManager.getInstance().enqueue(event);
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
      <div className="w-[375px] max-w-[375px] mx-auto h-[812px] bg-gradient-to-br from-white/40 to-white/10 backdrop-blur-[30px] saturate-[210%] shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-4 px-6 bg-white backdrop-blur-[30px] shadow-sm border-b border-gray-200 sticky top-0 z-10 flex justify-between items-center">
          <div>
            <h1 className="text-xl font-bold text-gray-900">{texts.kds}</h1>
            {isOffline && <span className="text-[#FF3B30] font-bold text-sm bg-red-100 px-2 py-1 rounded-md">{texts.offline}</span>}
          </div>
          <button
            onClick={toggleLanguage}
            className="text-[#0071E3] font-bold px-3 py-1 bg-blue-50 rounded-lg hover:bg-blue-100 transition"
            data-testid="lang-toggle"
          >
            {language === 'en' ? 'عربي' : 'EN'}
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-4 pb-20 flex flex-col md:flex-row gap-6">

          <div className="flex-1">
          <h2 className="text-lg font-bold text-gray-800 mb-3">{texts.orders}</h2>
          <div className="flex flex-col gap-4 mb-6">
            {orders.map(order => (
              <div key={order.id} className="app-card backdrop-blur-[30px] rounded-2xl p-4 shadow-sm border border-gray-100">
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
                  {order.items.map((item: string, idx: number) => <li key={idx}>• {item}</li>)}
                </ul>
                {order.translated_notes && (
                  <div className="mb-4 p-2 bg-blue-50 text-blue-800 text-sm rounded-lg border border-blue-100">
                    <strong>{language === 'ar' ? 'ملاحظات:' : 'Notes:'}</strong> {order.translated_notes}
                  </div>
                )}
                <div className="grid grid-cols-2 gap-2">
                   {order.status === 'Received' && (
                      <button
                        onClick={() => handleUpdateOrderStatus(order.id, 'Preparing')}
                        className="col-span-2 w-full py-4 bg-yellow-500 text-white font-bold text-lg rounded-xl shadow active:scale-95 transition"
                        data-testid={`btn-prepare-${order.id}`}
                      >
                        {texts.preparing}
                      </button>
                   )}
                   {order.status === 'Preparing' && (
                      <button
                        onClick={() => handleUpdateOrderStatus(order.id, 'Ready')}
                        className="col-span-2 w-full py-4 bg-[#34C759] text-white font-bold text-lg rounded-xl shadow active:scale-95 transition"
                        data-testid={`btn-ready-${order.id}`}
                      >
                        {texts.ready}
                      </button>
                   )}
                   {order.status === 'Ready' && (
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
          </div>

          <div className="flex-1 md:border-l md:pl-6 border-gray-200">
          <h2 className="text-lg font-bold text-gray-800 mb-3">{texts.inventory}</h2>
          <div className="flex flex-col gap-3">
             {inventory.map(item => (
                <div key={item.id} className="app-card backdrop-blur-[30px] rounded-xl p-4 shadow-sm border border-gray-100 flex justify-between items-center">
                   <span className="font-bold text-gray-800 text-lg">{language === 'en' ? item.name_en : item.name_ar}</span>
                   <button
                     id={`sold-out-toggle-${item.id}`}
                     onClick={() => handleToggleSoldOut(item.id, !item.is_sold_out)}
                     className={`px-6 py-4 rounded-xl font-bold text-lg shadow active:scale-95 transition min-w-[120px] ${item.is_sold_out ? 'bg-[#FF3B30] text-white' : 'bg-green-100 text-green-700'}`}
                     data-testid={`toggle-soldout-${item.id}`}
                   >
                     {item.is_sold_out ? texts.soldOut : texts.available}
                   </button>
                </div>
             ))}
          </div>
          </div>

        </div>

        {/* Sync Indicator */}
        {syncing && (
          <div className="absolute bottom-0 w-full bg-[#0066FF] text-white text-center py-2 text-sm font-bold animate-pulse z-20">
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
