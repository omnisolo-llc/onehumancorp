"use client";

import React, { useState, useEffect } from 'react';
import { syncManager } from '../../../lib/syncManager';

export default function KDSPage() {
  const [orders, setOrders] = useState<any[]>([]);
  const [inventory, setInventory] = useState<any[]>([]);
  const [language, setLanguage] = useState<'en' | 'ar'>('en');
  const [isOffline, setIsOffline] = useState(false);
  const [syncStatus, setSyncStatus] = useState(syncManager.getStatus());

  // Network listener & Sync Status
  useEffect(() => {
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    setIsOffline(!navigator.onLine);

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    const unsubscribe = syncManager.subscribe(setSyncStatus);

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
      unsubscribe();
    };
  }, []);

  // Initial Data Load
  useEffect(() => {
    fetch('/api/pos/orders').then(res => res.json()).then(setOrders).catch(console.error);
    fetch('/api/pos/inventory').then(res => res.json()).then(setInventory).catch(console.error);
  }, []);

  const handleUpdateOrderStatus = (orderId: string, newStatus: string) => {
    // Optimistic UI Update
    setOrders(prev => prev.map(o => o.id === orderId ? { ...o, status: newStatus, isPending: true } : o));

    syncManager.enqueue({
      mutation_type: 'UPDATE_ORDER_STATUS',
      order_id: orderId,
      status: newStatus,
    });
  };

  const handleToggleSoldOut = (itemId: string, isSoldOut: boolean) => {
    // Optimistic UI Update
    setInventory(prev => prev.map(i => i.id === itemId ? { ...i, is_sold_out: isSoldOut, isPending: true } : i));

    syncManager.enqueue({
      mutation_type: 'TOGGLE_SOLD_OUT',
      product_id: itemId,
      metadata: { is_sold_out: isSoldOut }
    });
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
      syncing: 'Syncing...',
      pending: 'Sync Pending'
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
      syncing: 'جاري المزامنة...',
      pending: 'في انتظار المزامنة'
    }
  };

  const texts = t[language];

  return (
    <div dir={language === 'ar' ? 'rtl' : 'ltr'} className="flex flex-col items-center justify-center min-h-screen bg-gray-900 font-inter py-10">
      <div className="w-[375px] h-[812px] bg-black text-white shadow-2xl overflow-hidden flex flex-col relative border border-gray-800 rounded-[40px]">

        {/* Status Bar / Sync Indicator */}
        <div className="pt-12 pb-4 px-6 bg-black/40 backdrop-blur-[20px] border-b border-white/10 sticky top-0 z-10 flex justify-between items-center">
          <div className="flex items-center gap-2">
            <h1 className="text-lg font-bold font-outfit">{texts.kds}</h1>
            {isOffline && <div className="w-2 h-2 rounded-full bg-amber-500 animate-pulse" title={texts.offline} />}
          </div>
          <div className="flex items-center gap-3">
             {syncStatus.pendingCount > 0 && (
                <div className="flex items-center gap-2 px-3 py-1 bg-white/5 rounded-full border border-white/10">
                   <span className="text-[10px] font-bold text-gray-400 uppercase tracking-widest">{syncStatus.isSyncing ? texts.syncing : texts.pending}</span>
                   <span className="text-xs font-bold text-white">{syncStatus.pendingCount}</span>
                </div>
             )}
             <button
               onClick={toggleLanguage}
               className="text-xs font-bold text-gray-400 hover:text-white transition"
             >
               {language === 'en' ? 'عربي' : 'EN'}
             </button>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 pb-24">

          <h2 className="text-xs font-bold text-gray-500 uppercase tracking-widest mb-4 px-2">{texts.orders}</h2>
          <div className="flex flex-col gap-4 mb-8">
            {orders.map(order => (
              <div key={order.id} className={`bg-white/5 backdrop-blur-[20px] rounded-3xl p-5 border border-white/10 transition-all ${order.isPending ? 'opacity-60 grayscale-[0.5]' : ''}`}>
                <div className="flex justify-between items-start mb-3">
                  <div>
                    <h3 className="font-bold text-white">#{order.id.slice(-4)}</h3>
                    <p className="text-xs text-gray-400">{order.customer_name}</p>
                  </div>
                  <span className={`px-2 py-1 rounded-full text-[10px] font-bold uppercase tracking-wider ${
                    order.status === 'Ready' ? 'bg-green-500/20 text-green-400' :
                    order.status === 'Preparing' ? 'bg-amber-500/20 text-amber-400' :
                    'bg-blue-500/20 text-blue-400'
                  }`}>
                    {order.status === 'Ready' ? texts.ready : order.status === 'Preparing' ? texts.preparing : texts.received}
                  </span>
                </div>
                <div className="space-y-1 mb-6">
                  {order.items.map((item: string, idx: number) => (
                    <div key={idx} className="text-sm text-gray-300 flex items-center gap-2">
                       <div className="w-1 h-1 rounded-full bg-white/20" />
                       {item}
                    </div>
                  ))}
                </div>
                <div className="grid grid-cols-2 gap-2">
                   {order.status === 'Received' && (
                      <button
                        onClick={() => handleUpdateOrderStatus(order.id, 'Preparing')}
                        className="col-span-2 w-full py-3 bg-white text-black font-bold rounded-xl active:scale-[0.98] transition shadow-lg shadow-white/5"
                      >
                        {texts.preparing}
                      </button>
                   )}
                   {order.status === 'Preparing' && (
                      <button
                        onClick={() => handleUpdateOrderStatus(order.id, 'Ready')}
                        className="col-span-2 w-full py-3 bg-green-500 text-white font-bold rounded-xl active:scale-[0.98] transition shadow-lg shadow-green-500/20"
                      >
                        {texts.ready}
                      </button>
                   )}
                </div>
              </div>
            ))}
          </div>

          <h2 className="text-xs font-bold text-gray-500 uppercase tracking-widest mb-4 px-2">{texts.inventory}</h2>
          <div className="space-y-3">
             {inventory.map(item => (
                <div key={item.id} className={`bg-white/5 backdrop-blur-[20px] rounded-2xl p-4 border border-white/10 flex justify-between items-center transition-all ${item.isPending ? 'opacity-60 blur-[0.5px]' : ''}`}>
                   <span className="font-bold text-gray-200">{language === 'en' ? item.name_en : item.name_ar}</span>
                   <button
                     onClick={() => handleToggleSoldOut(item.id, !item.is_sold_out)}
                     className={`px-4 py-2 rounded-xl text-xs font-bold transition-all ${item.is_sold_out ? 'bg-red-500/20 text-red-400 border border-red-500/30' : 'bg-green-500/20 text-green-400 border border-green-500/30'}`}
                   >
                     {item.is_sold_out ? texts.soldOut : texts.available}
                   </button>
                </div>
             ))}
          </div>

        </div>

      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
