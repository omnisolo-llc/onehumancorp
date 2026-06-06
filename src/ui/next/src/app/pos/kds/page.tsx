"use client";

import React, { useState, useEffect } from 'react';
import { useSyncManager } from '../../../lib/useSyncManager';

export default function KDSPage() {
  const [orders, setOrders] = useState<any[]>([]);
  const [inventory, setInventory] = useState<any[]>([]);
  const [language, setLanguage] = useState<'en' | 'ar'>('en');

  const { queueMutation, isOffline, syncing } = useSyncManager('ohc_kds_events', '/api/v1/sync/kds');

  // Initial Data Load
  useEffect(() => {
    fetch('/api/v1/pos/orders').then(res => res.json()).then(setOrders).catch(console.error);
    fetch('/api/v1/pos/inventory').then(res => res.json()).then(setInventory).catch(console.error);
  }, []);

  const handleUpdateOrderStatus = (orderId: string, newStatus: string) => {
    // Optimistic UI Update
    setOrders(prev => prev.map(o => o.id === orderId ? { ...o, status: newStatus } : o));
    queueMutation('UPDATE_ORDER_STATUS', { order_id: orderId, status: newStatus });
  };

  const handleToggleSoldOut = (itemId: string, isSoldOut: boolean) => {
    // Optimistic UI Update
    setInventory(prev => prev.map(i => i.id === itemId ? { ...i, is_sold_out: isSoldOut } : i));
    queueMutation('TOGGLE_SOLD_OUT', { item_id: itemId, is_sold_out: isSoldOut });
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
                  <h3 className="font-bold text-lg text-gray-900">#{order.id.slice(0, 8)} - {order.customer_name}</h3>
                  <span className={`px-2 py-1 rounded text-xs font-bold ${
                    order.status === 'Ready' || order.status === 'ready' ? 'bg-green-100 text-green-700' :
                    order.status === 'Preparing' || order.status === 'preparing' ? 'bg-yellow-100 text-yellow-700' :
                    'bg-blue-100 text-blue-700'
                  }`}>
                    {order.status === 'Ready' || order.status === 'ready' ? texts.ready : order.status === 'Preparing' || order.status === 'preparing' ? texts.preparing : texts.received}
                  </span>
                </div>
                <ul className="mb-4 text-gray-700 font-medium">
                  {order.items.map((item: string, idx: number) => <li key={idx}>• {item}</li>)}
                </ul>
                <div className="grid grid-cols-2 gap-2">
                   {(order.status === 'Received' || order.status === 'pending' || order.status === 'received') && (
                      <button
                        onClick={() => handleUpdateOrderStatus(order.id, 'Preparing')}
                        className="col-span-2 w-full py-4 bg-yellow-500 text-white font-bold text-lg rounded-xl shadow active:scale-95 transition"
                        data-testid={`btn-prepare-${order.id}`}
                      >
                        {texts.preparing}
                      </button>
                   )}
                   {(order.status === 'Preparing' || order.status === 'preparing') && (
                      <button
                        onClick={() => handleUpdateOrderStatus(order.id, 'Ready')}
                        className="col-span-2 w-full py-4 bg-green-500 text-white font-bold text-lg rounded-xl shadow active:scale-95 transition"
                        data-testid={`btn-ready-${order.id}`}
                      >
                        {texts.ready}
                      </button>
                   )}
                   {(order.status === 'Ready' || order.status === 'ready') && (
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
