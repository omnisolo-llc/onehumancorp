'use client';

import React, { useState, useEffect, useRef } from 'react';
import Head from 'next/head';

interface Order {
  id: string;
  customerName: string;
  items: string[];
  status: 'new' | 'preparing' | 'ready';
  timestamp: string;
}

interface MenuItem {
  id: string;
  name: string;
  soldOut: boolean;
}

const translations = {
  en: {
    title: 'Kitchen Display',
    newOrders: 'New Orders',
    preparing: 'Preparing',
    markPreparing: 'Start Preparing',
    soldOut: 'Sold Out',
    markSoldOut: 'Mark Sold Out',
    markAvailable: 'Mark Available',
    language: 'عربي',
    menu: 'Menu Items',
    ready: 'Mark Ready',
    queueEmpty: 'No new orders',
    syncing: 'Syncing...',
    offline: 'Offline Mode',
    items: 'Items:',
    customer: 'Customer:',
    queueStatus: (count: number) => `${count} Mutations Pending Sync`
  },
  ar: {
    title: 'شاشة المطبخ',
    newOrders: 'طلبات جديدة',
    preparing: 'جاري التحضير',
    markPreparing: 'ابدأ التحضير',
    soldOut: 'نفد',
    markSoldOut: 'تحديد كنفد',
    markAvailable: 'تحديد كمتاح',
    language: 'English',
    menu: 'عناصر القائمة',
    ready: 'تحديد كجاهز',
    queueEmpty: 'لا توجد طلبات جديدة',
    syncing: 'جاري المزامنة...',
    offline: 'وضع عدم الاتصال',
    items: 'العناصر:',
    customer: 'الزبون:',
    queueStatus: (count: number) => `${count} تغييرات في انتظار المزامنة`
  }
};

export default function KDS() {
  const [lang, setLang] = useState<'en' | 'ar'>('en');
  const t = translations[lang];
  const [isOnline, setIsOnline] = useState(true);
  const [offlineQueue, setOfflineQueue] = useState<any[]>([]);
  const audioContextRef = useRef<AudioContext | null>(null);

  const [orders, setOrders] = useState<Order[]>([
      { id: '101', customerName: 'John Doe', items: ['2x Chicken Over Rice', '1x Soda'], status: 'new', timestamp: new Date().toISOString() }
  ]);
  const [menu, setMenu] = useState<MenuItem[]>([
    { id: 'falafel', name: 'Falafel', soldOut: false },
    { id: 'chicken-rice', name: 'Chicken Over Rice', soldOut: false },
    { id: 'lamb-gyro', name: 'Lamb Gyro', soldOut: false },
  ]);

  useEffect(() => {
    setIsOnline(navigator.onLine);

    const handleOnline = () => {
        setIsOnline(true);
        flushOfflineQueue();
    };
    const handleOffline = () => setIsOnline(false);

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    // Initialize offline queue from local storage
    try {
        const q = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
        setOfflineQueue(q);
    } catch(e) {}

    // Init Audio Context for notification chime
    audioContextRef.current = new (window.AudioContext || (window as any).webkitAudioContext)();

    return () => {
        window.removeEventListener('online', handleOnline);
        window.removeEventListener('offline', handleOffline);
    };
  }, []);

  const playChime = () => {
    if (audioContextRef.current) {
        if (audioContextRef.current.state === 'suspended') {
             audioContextRef.current.resume();
        }
        const osc = audioContextRef.current.createOscillator();
        const gainNode = audioContextRef.current.createGain();
        osc.connect(gainNode);
        gainNode.connect(audioContextRef.current.destination);
        osc.type = 'sine';
        osc.frequency.setValueAtTime(880, audioContextRef.current.currentTime);
        gainNode.gain.setValueAtTime(1, audioContextRef.current.currentTime);
        gainNode.gain.exponentialRampToValueAtTime(0.001, audioContextRef.current.currentTime + 1);
        osc.start(audioContextRef.current.currentTime);
        osc.stop(audioContextRef.current.currentTime + 1);
    }
  };

  const enqueueMutation = (mutation: any) => {
    const q = [...offlineQueue, mutation];
    setOfflineQueue(q);
    localStorage.setItem('ohc_offline_queue', JSON.stringify(q));
    if (isOnline) {
        flushOfflineQueue();
    }
  };

  const flushOfflineQueue = async () => {
    const q = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
    if (q.length === 0) return;

    try {
        const res = await fetch('/api/mesh/v2/broadcast', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'x-spiffe-id': 'spiffe://ohc/org/example.org/agent/kds'
            },
            body: JSON.stringify({
                topic: 'mesh:kds:sync',
                message: {
                     agent_id: 'kds',
                     action: 'sync_mutations',
                     status: 'ok',
                     payload: btoa(JSON.stringify(q)),
                     msg_id: crypto.randomUUID()
                }
            })
        });

        if (res.ok) {
            setOfflineQueue([]);
            localStorage.setItem('ohc_offline_queue', '[]');
        }
    } catch (e) {
        console.error('Failed to flush offline queue', e);
    }
  };

  const toggleLang = () => {
    setLang(lang === 'en' ? 'ar' : 'en');
  };

  const handleSoldOutToggle = (id: string) => {
    setMenu(prev => prev.map(item => item.id === id ? { ...item, soldOut: !item.soldOut } : item));
    enqueueMutation({
        id: `e2e-product-${id}`,
        type: 'inventory_toggle',
        timestamp: new Date().toISOString()
    });
  };

  const markOrderPreparing = (id: string) => {
      setOrders(prev => prev.map(o => o.id === id ? { ...o, status: 'preparing' } : o));
      enqueueMutation({
          id: id,
          type: 'order_status_change',
          status: 'preparing',
          timestamp: new Date().toISOString()
      });
  };

  // Simulate incoming order
  useEffect(() => {
      const handleCustomPush = (e: any) => {
           playChime();
           const newOrder: Order = {
               id: crypto.randomUUID(),
               customerName: 'New Customer',
               items: ['1x Item'],
               status: 'new',
               timestamp: new Date().toISOString()
           };
           setOrders(prev => [newOrder, ...prev]);
      };
      window.addEventListener('push-notification', handleCustomPush);
      return () => window.removeEventListener('push-notification', handleCustomPush);
  }, []);

  return (
    <div className={`min-h-screen bg-gray-900 text-white ${lang === 'ar' ? 'rtl font-arabic' : 'ltr font-outfit'}`} dir={lang === 'ar' ? 'rtl' : 'ltr'}>
      <Head>
        <title>{t.title} - OHC</title>
        <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=0" />
      </Head>

      <header className="bg-gray-800 p-4 sticky top-0 z-10 shadow-md flex justify-between items-center border-b border-gray-700">
        <h1 className="text-2xl font-bold text-white">{t.title}</h1>
        <div className="flex gap-4 items-center">
          <div id="network-status-indicator" className={`px-3 py-1 rounded-full text-sm font-bold ${!isOnline ? 'bg-red-600 text-white block' : 'hidden'}`}>
             {t.offline}
          </div>
          <button
            id="lang-toggle"
            onClick={toggleLang}
            className="px-6 py-4 bg-blue-600 text-white rounded-xl font-bold text-xl min-w-[120px] hover:bg-blue-500 transition-colors shadow-lg active:scale-95"
          >
            {t.language}
          </button>
        </div>
      </header>

      <main className="p-4 flex flex-col gap-8 max-w-lg mx-auto pb-24">
        {offlineQueue.length > 0 && (
             <div id="queue-dashboard" className="bg-yellow-600 text-white p-3 rounded-xl font-bold text-center block">
                 {t.queueStatus(offlineQueue.length)}
             </div>
        )}

        <section>
          <h2 className="text-2xl font-semibold mb-6 text-gray-300 border-b border-gray-700 pb-2">{t.newOrders}</h2>
          {orders.length === 0 ? (
            <div className="bg-gray-800 p-12 rounded-2xl border border-gray-700 text-center text-gray-400 text-xl font-medium">
              {t.queueEmpty}
            </div>
          ) : (
            <div className="flex flex-col gap-6">
              {orders.map(order => (
                  <div key={order.id} className="bg-gray-800 p-6 rounded-3xl border-2 border-gray-700 flex flex-col gap-4 shadow-xl">
                      <div className="flex justify-between items-start">
                          <div>
                              <p className="text-gray-400 text-lg mb-1">{t.customer}</p>
                              <p className="text-2xl font-bold text-white">{order.customerName}</p>
                          </div>
                          <span className={`px-4 py-2 rounded-lg text-sm font-bold uppercase tracking-wider ${order.status === 'new' ? 'bg-blue-900 text-blue-200' : 'bg-yellow-900 text-yellow-200'}`}>
                              {order.status === 'new' ? t.newOrders : t.preparing}
                          </span>
                      </div>
                      <div className="bg-gray-900 p-4 rounded-xl border border-gray-800">
                          <p className="text-gray-400 text-sm mb-2">{t.items}</p>
                          <ul className="list-disc pl-5 flex flex-col gap-2">
                              {order.items.map((item, i) => (
                                  <li key={i} className="text-xl font-medium text-gray-100">{item}</li>
                              ))}
                          </ul>
                      </div>
                      {order.status === 'new' && (
                          <button
                             onClick={() => markOrderPreparing(order.id)}
                             className="w-full py-6 mt-2 bg-green-600 hover:bg-green-500 text-white text-2xl font-bold rounded-2xl shadow-lg active:scale-95 transition-all"
                          >
                             {t.markPreparing}
                          </button>
                      )}
                  </div>
              ))}
            </div>
          )}
        </section>

        <section className="mt-8">
          <h2 className="text-2xl font-semibold mb-6 text-gray-300 border-b border-gray-700 pb-2">{t.menu}</h2>
          <div className="flex flex-col gap-4">
            {menu.map(item => (
              <div key={item.id} className="bg-gray-800 p-5 rounded-3xl border border-gray-700 flex justify-between items-center shadow-lg">
                <div className="flex-1">
                    <span className={`text-2xl font-bold ${item.soldOut ? 'text-gray-500 line-through' : 'text-gray-100'}`}>
                      {item.name}
                    </span>
                </div>
                <button
                  id={`sold-out-toggle-${item.id}`}
                  onClick={() => handleSoldOutToggle(item.id)}
                  className={`px-8 py-6 rounded-2xl font-bold text-xl min-w-[180px] shadow-md active:scale-95 transition-all ${
                    item.soldOut ? 'bg-gray-700 text-gray-300 border-2 border-gray-600' : 'bg-red-600 text-white hover:bg-red-500'
                  }`}
                >
                  {item.soldOut ? t.markAvailable : t.soldOut}
                </button>
              </div>
            ))}
          </div>
        </section>
      </main>
    </div>
  );
}
