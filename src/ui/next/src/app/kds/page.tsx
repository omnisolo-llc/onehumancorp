'use client';

import React, { useState, useEffect } from 'react';

// Multilingual Dictionary
const translations: Record<string, Record<string, string>> = {
  en: {
    kds_title: "Kitchen Display System",
    language_toggle: "عربي",
    new_order: "New Pre-Order",
    sold_out: "Mark Sold Out",
    sold_out_status: "Sold Out",
    preparing: "Preparing",
    ready: "Ready",
    items: "Items",
    offline_sync_pending: "Sync Pending",
    online: "Online",
    offline: "Offline",
  },
  ar: {
    kds_title: "نظام عرض المطبخ",
    language_toggle: "English",
    new_order: "طلب مسبق جديد",
    sold_out: "تحديد كنفاد",
    sold_out_status: "نفد",
    preparing: "قيد التحضير",
    ready: "جاهز",
    items: "عناصر",
    offline_sync_pending: "المزامنة معلقة",
    online: "متصل",
    offline: "غير متصل",
  }
};

type Order = {
  id: string;
  items: string[];
  status: 'pending' | 'preparing' | 'ready';
  time: string;
};

type InventoryItem = {
  id: string;
  name: string;
  soldOut: boolean;
};

export default function KDSView() {
  const [lang, setLang] = useState<'en' | 'ar'>('en');
  const t = translations[lang];
  const isRtl = lang === 'ar';

  const [isOnline, setIsOnline] = useState<boolean>(true);
  const [syncQueue, setSyncQueue] = useState<any[]>([]);

  const [orders, setOrders] = useState<Order[]>([
    { id: '101', items: ['2x Chicken Over Rice'], status: 'pending', time: '12:05 PM' },
    { id: '102', items: ['1x Falafel Wrap', '1x Mint Lemonade'], status: 'pending', time: '12:08 PM' }
  ]);

  const [inventory, setInventory] = useState<InventoryItem[]>([
    { id: 'item-chicken', name: 'Chicken Over Rice', soldOut: false },
    { id: 'item-falafel', name: 'Falafel Wrap', soldOut: false },
  ]);

  // Handle Offline/Online Status
  useEffect(() => {
    const handleOnline = () => {
      setIsOnline(true);
      syncWithCloud();
    };
    const handleOffline = () => setIsOnline(false);

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    setIsOnline(navigator.onLine);

    // Load initial queue
    try {
      const q = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
      setSyncQueue(q);
    } catch (e) {}

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  const queueMutation = (mutation: any) => {
    const newQueue = [...syncQueue, mutation];
    setSyncQueue(newQueue);
    localStorage.setItem('ohc_offline_queue', JSON.stringify(newQueue));
    if (isOnline) {
      syncWithCloud();
    }
  };

  const syncWithCloud = async () => {
    // In a real app, send to Hybrid Event Mesh/Gateway
    // fetch('/api/kds/sync', ...
    // For now, just clear queue on successful online status
    try {
      const q = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
      if (q.length > 0) {
        // simulate async network request
        setTimeout(() => {
          setSyncQueue([]);
          localStorage.removeItem('ohc_offline_queue');
        }, 500);
      }
    } catch (e) {}
  };

  const toggleLanguage = () => {
    setLang(lang === 'en' ? 'ar' : 'en');
  };

  const updateOrderStatus = (orderId: string, status: 'preparing' | 'ready') => {
    setOrders(orders.map(o => o.id === orderId ? { ...o, status } : o));
    queueMutation({ type: 'order_status_update', orderId, status, timestamp: Date.now() });
  };

  const toggleSoldOut = (itemId: string) => {
    setInventory(inventory.map(i => i.id === itemId ? { ...i, soldOut: !i.soldOut } : i));
    queueMutation({ type: 'inventory_toggle', itemId, timestamp: Date.now() });
  };

  return (
    <div className={`min-h-screen bg-gray-50 flex flex-col ${isRtl ? 'rtl' : 'ltr'}`} dir={isRtl ? 'rtl' : 'ltr'}>
      {/* Header */}
      <header className="bg-white shadow-sm p-4 flex justify-between items-center sticky top-0 z-10">
        <h1 className="text-xl font-bold font-outfit text-gray-900">{t.kds_title}</h1>
        <div className="flex items-center space-x-4 space-x-reverse">
          <div className="flex items-center text-sm font-medium">
            <span id="network-status-indicator" className={`w-3 h-3 rounded-full mr-2 ${isRtl ? 'ml-2 mr-0' : ''} ${isOnline ? 'bg-green-500' : 'bg-red-500'}`}></span>
            {isOnline ? t.online : t.offline}
          </div>
          <button
            id="lang-toggle-btn"
            onClick={toggleLanguage}
            className="px-3 py-1 bg-gray-100 rounded-lg text-sm hover:bg-gray-200 transition-colors"
          >
            {t.language_toggle}
          </button>
        </div>
      </header>

      {/* Sync Banner */}
      {syncQueue.length > 0 && !isOnline && (
        <div id="queue-dashboard" className="bg-yellow-100 text-yellow-800 text-center py-2 text-sm font-medium">
          {syncQueue.length} {t.offline_sync_pending}
        </div>
      )}

      <main className="flex-1 p-4 overflow-y-auto max-w-md mx-auto w-full">
        {/* Orders List */}
        <div className="mb-8">
          {orders.map(order => (
            <div key={order.id} className="bg-white rounded-xl shadow-sm p-4 mb-4 border border-gray-100">
              <div className="flex justify-between items-start mb-3">
                <div>
                  <span className="text-sm text-gray-500 block">{order.time}</span>
                  <span className="font-bold text-lg text-gray-900">#{order.id}</span>
                </div>
                <div className="bg-blue-50 text-blue-700 px-2 py-1 rounded text-xs font-bold uppercase tracking-wider">
                  {t.new_order}
                </div>
              </div>

              <ul className="mb-4 text-gray-800 font-medium">
                {order.items.map((item, idx) => (
                  <li key={idx} className="flex items-center before:content-['•'] before:mr-2 before:text-gray-400 rtl:before:ml-2 rtl:before:mr-0">
                    {item}
                  </li>
                ))}
              </ul>

              <div className="flex space-x-2 space-x-reverse">
                {order.status === 'pending' && (
                  <button
                    id={`btn-prep-${order.id}`}
                    onClick={() => updateOrderStatus(order.id, 'preparing')}
                    className="flex-1 bg-blue-600 text-white py-3 rounded-lg font-bold hover:bg-blue-700 active:scale-95 transition-all text-lg"
                  >
                    {t.preparing}
                  </button>
                )}
                {order.status === 'preparing' && (
                  <button
                    id={`btn-ready-${order.id}`}
                    onClick={() => updateOrderStatus(order.id, 'ready')}
                    className="flex-1 bg-green-500 text-white py-3 rounded-lg font-bold hover:bg-green-600 active:scale-95 transition-all text-lg"
                  >
                    {t.ready}
                  </button>
                )}
                {order.status === 'ready' && (
                  <button disabled className="flex-1 bg-gray-200 text-gray-500 py-3 rounded-lg font-bold text-lg">
                    {t.ready} ✓
                  </button>
                )}
              </div>
            </div>
          ))}
        </div>

        {/* Inventory Sold Out Toggles */}
        <div className="bg-white rounded-xl shadow-sm p-4 border border-gray-100">
          <h2 className="font-bold text-lg text-gray-900 mb-4 font-outfit">{t.items}</h2>
          <div className="space-y-4">
            {inventory.map(item => (
              <div key={item.id} className="flex justify-between items-center pb-4 border-b border-gray-50 last:border-0 last:pb-0">
                <span className={`font-medium ${item.soldOut ? 'text-gray-400 line-through' : 'text-gray-800'}`}>
                  {item.name}
                </span>
                <button
                  id={`sold-out-toggle-${item.id}`}
                  onClick={() => toggleSoldOut(item.id)}
                  className={`px-4 py-2 rounded-lg text-sm font-bold transition-colors ${
                    item.soldOut
                      ? 'bg-red-100 text-red-700'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  {item.soldOut ? t.sold_out_status : t.sold_out}
                </button>
              </div>
            ))}
          </div>
        </div>
      </main>
    </div>
  );
}
'use client';

import React, { useState, useEffect } from 'react';

// Multilingual Dictionary
const translations: Record<string, Record<string, string>> = {
  en: {
    kds_title: "Kitchen Display System",
    language_toggle: "عربي",
    new_order: "New Pre-Order",
    sold_out: "Mark Sold Out",
    sold_out_status: "Sold Out",
    preparing: "Preparing",
    ready: "Ready",
    items: "Items",
    offline_sync_pending: "Sync Pending",
    online: "Online",
    offline: "Offline",
  },
  ar: {
    kds_title: "نظام عرض المطبخ",
    language_toggle: "English",
    new_order: "طلب مسبق جديد",
    sold_out: "تحديد كنفاد",
    sold_out_status: "نفد",
    preparing: "قيد التحضير",
    ready: "جاهز",
    items: "عناصر",
    offline_sync_pending: "المزامنة معلقة",
    online: "متصل",
    offline: "غير متصل",
  }
};

type Order = {
  id: string;
  items: string[];
  status: 'pending' | 'preparing' | 'ready';
  time: string;
};

type InventoryItem = {
  id: string;
  name: string;
  soldOut: boolean;
};

export default function KDSView() {
  const [lang, setLang] = useState<'en' | 'ar'>('en');
  const t = translations[lang];
  const isRtl = lang === 'ar';

  const [isOnline, setIsOnline] = useState<boolean>(true);
  const [syncQueue, setSyncQueue] = useState<any[]>([]);

  const [orders, setOrders] = useState<Order[]>([
    { id: '101', items: ['2x Chicken Over Rice'], status: 'pending', time: '12:05 PM' },
    { id: '102', items: ['1x Falafel Wrap', '1x Mint Lemonade'], status: 'pending', time: '12:08 PM' }
  ]);

  const [inventory, setInventory] = useState<InventoryItem[]>([
    { id: 'item-chicken', name: 'Chicken Over Rice', soldOut: false },
    { id: 'item-falafel', name: 'Falafel Wrap', soldOut: false },
  ]);

  // Handle Offline/Online Status
  useEffect(() => {
    const handleOnline = () => {
      setIsOnline(true);
      syncWithCloud();
    };
    const handleOffline = () => setIsOnline(false);

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    setIsOnline(navigator.onLine);

    // Load initial queue
    try {
      const q = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
      setSyncQueue(q);
    } catch (e) {}

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  const queueMutation = (mutation: any) => {
    const newQueue = [...syncQueue, mutation];
    setSyncQueue(newQueue);
    localStorage.setItem('ohc_offline_queue', JSON.stringify(newQueue));
    if (isOnline) {
      syncWithCloud();
    }
  };

  const syncWithCloud = async () => {
    // In a real app, send to Hybrid Event Mesh/Gateway
    // fetch('/api/kds/sync', ...
    // For now, just clear queue on successful online status
    try {
      const q = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
      if (q.length > 0) {
        // simulate async network request
        setTimeout(() => {
          setSyncQueue([]);
          localStorage.removeItem('ohc_offline_queue');
        }, 500);
      }
    } catch (e) {}
  };

  const toggleLanguage = () => {
    setLang(lang === 'en' ? 'ar' : 'en');
  };

  const updateOrderStatus = (orderId: string, status: 'preparing' | 'ready') => {
    setOrders(orders.map(o => o.id === orderId ? { ...o, status } : o));
    queueMutation({ type: 'order_status_update', orderId, status, timestamp: Date.now() });
  };

  const toggleSoldOut = (itemId: string) => {
    setInventory(inventory.map(i => i.id === itemId ? { ...i, soldOut: !i.soldOut } : i));
    queueMutation({ type: 'inventory_toggle', itemId, timestamp: Date.now() });
  };

  return (
    <div className={`min-h-screen bg-gray-50 flex flex-col ${isRtl ? 'rtl' : 'ltr'}`} dir={isRtl ? 'rtl' : 'ltr'}>
      {/* Header */}
      <header className="bg-white shadow-sm p-4 flex justify-between items-center sticky top-0 z-10">
        <h1 className="text-xl font-bold font-outfit text-gray-900">{t.kds_title}</h1>
        <div className="flex items-center space-x-4 space-x-reverse">
          <div className="flex items-center text-sm font-medium">
            <span id="network-status-indicator" className={`w-3 h-3 rounded-full mr-2 ${isRtl ? 'ml-2 mr-0' : ''} ${isOnline ? 'bg-green-500' : 'bg-red-500'}`}></span>
            {isOnline ? t.online : t.offline}
          </div>
          <button
            id="lang-toggle-btn"
            onClick={toggleLanguage}
            className="px-3 py-1 bg-gray-100 rounded-lg text-sm hover:bg-gray-200 transition-colors"
          >
            {t.language_toggle}
          </button>
        </div>
      </header>

      {/* Sync Banner */}
      {syncQueue.length > 0 && !isOnline && (
        <div id="queue-dashboard" className="bg-yellow-100 text-yellow-800 text-center py-2 text-sm font-medium">
          {syncQueue.length} {t.offline_sync_pending}
        </div>
      )}

      <main className="flex-1 p-4 overflow-y-auto max-w-md mx-auto w-full">
        {/* Orders List */}
        <div className="mb-8">
          {orders.map(order => (
            <div key={order.id} className="bg-white rounded-xl shadow-sm p-4 mb-4 border border-gray-100">
              <div className="flex justify-between items-start mb-3">
                <div>
                  <span className="text-sm text-gray-500 block">{order.time}</span>
                  <span className="font-bold text-lg text-gray-900">#{order.id}</span>
                </div>
                <div className="bg-blue-50 text-blue-700 px-2 py-1 rounded text-xs font-bold uppercase tracking-wider">
                  {t.new_order}
                </div>
              </div>

              <ul className="mb-4 text-gray-800 font-medium">
                {order.items.map((item, idx) => (
                  <li key={idx} className="flex items-center before:content-['•'] before:mr-2 before:text-gray-400 rtl:before:ml-2 rtl:before:mr-0">
                    {item}
                  </li>
                ))}
              </ul>

              <div className="flex space-x-2 space-x-reverse">
                {order.status === 'pending' && (
                  <button
                    id={`btn-prep-${order.id}`}
                    onClick={() => updateOrderStatus(order.id, 'preparing')}
                    className="flex-1 bg-blue-600 text-white py-3 rounded-lg font-bold hover:bg-blue-700 active:scale-95 transition-all text-lg"
                  >
                    {t.preparing}
                  </button>
                )}
                {order.status === 'preparing' && (
                  <button
                    id={`btn-ready-${order.id}`}
                    onClick={() => updateOrderStatus(order.id, 'ready')}
                    className="flex-1 bg-green-500 text-white py-3 rounded-lg font-bold hover:bg-green-600 active:scale-95 transition-all text-lg"
                  >
                    {t.ready}
                  </button>
                )}
                {order.status === 'ready' && (
                  <button disabled className="flex-1 bg-gray-200 text-gray-500 py-3 rounded-lg font-bold text-lg">
                    {t.ready} ✓
                  </button>
                )}
              </div>
            </div>
          ))}
        </div>

        {/* Inventory Sold Out Toggles */}
        <div className="bg-white rounded-xl shadow-sm p-4 border border-gray-100">
          <h2 className="font-bold text-lg text-gray-900 mb-4 font-outfit">{t.items}</h2>
          <div className="space-y-4">
            {inventory.map(item => (
              <div key={item.id} className="flex justify-between items-center pb-4 border-b border-gray-50 last:border-0 last:pb-0">
                <span className={`font-medium ${item.soldOut ? 'text-gray-400 line-through' : 'text-gray-800'}`}>
                  {item.name}
                </span>
                <button
                  id={`sold-out-toggle-${item.id}`}
                  onClick={() => toggleSoldOut(item.id)}
                  className={`px-4 py-2 rounded-lg text-sm font-bold transition-colors ${
                    item.soldOut
                      ? 'bg-red-100 text-red-700'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  {item.soldOut ? t.sold_out_status : t.sold_out}
                </button>
              </div>
            ))}
          </div>
        </div>
      </main>
    </div>
  );
}
