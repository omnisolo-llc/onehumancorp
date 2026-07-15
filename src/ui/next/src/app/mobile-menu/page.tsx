"use client";

import React, { useState, useEffect } from "react";
import { SyncManager } from "../../lib/sync/SyncManager";

export default function MobileMenu() {
  const [menu, setMenu] = useState<any[]>([]);
  const [offlineQueueCount, setOfflineQueueCount] = useState(0);
  const [isOffline, setIsOffline] = useState(false);
  const [showToast, setShowToast] = useState(false);
  const [language, setLanguage] = useState<'en' | 'ar'>('en');

  useEffect(() => {
    // Register Service Worker
    if ('serviceWorker' in navigator) {
      navigator.serviceWorker.register('/sw.js').catch(err => console.error('Service Worker registration failed:', err));
    }

    SyncManager.getInstance();

    const fetchMenu = async () => {
      try {
        const tenantId = localStorage.getItem("tenant_id") || "default";
        const menuRes = await fetch("/api/pos/inventory", {
           headers: { "x-tenant-id": tenantId }
        });
        if (menuRes.ok) {
           const data = await menuRes.json();
           const items = data.items || data.inventory || data || [];
           setMenu(items);
           localStorage.setItem("ohc_mobile_menu_cache", JSON.stringify(items));
        } else {
           throw new Error("Fetch failed");
        }
      } catch (err) {
        console.error("Failed to fetch menu data, trying cache", err);
        const cache = localStorage.getItem("ohc_mobile_menu_cache");
        if (cache) {
            setMenu(JSON.parse(cache));
        }
      }
    };

    fetchMenu();

    const updateCount = async () => {
      setOfflineQueueCount(await SyncManager.getInstance().getQueueLength());
    };

    updateCount();
    window.addEventListener("ohc_queue_updated", updateCount);

    const handleOnline = () => {
        setIsOffline(false);
        setShowToast(true);
        setTimeout(() => setShowToast(false), 3000);
    };
    const handleOffline = () => setIsOffline(true);

    if (typeof navigator !== 'undefined') {
        setIsOffline(!navigator.onLine);
    }
    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);

    return () => {
      window.removeEventListener("ohc_queue_updated", updateCount);
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    };
  }, []);

  const handleToggleSoldOut = async (itemId: string, currentStatus: boolean) => {
    // Optimistic UI update
    const newStatus = !currentStatus;
    const newMenu = menu.map(m => m.id === itemId ? { ...m, is_sold_out: newStatus } : m);
    setMenu(newMenu);
    localStorage.setItem("ohc_mobile_menu_cache", JSON.stringify(newMenu));

    // Add to sync queue for eventual consistency
    await SyncManager.getInstance().enqueue({
      id: `e2e-product-${itemId}-${Date.now()}`,
      type: "TOGGLE_SOLD_OUT",
      payload: { item_id: itemId, is_sold_out: newStatus },
      timestamp: Date.now()
    });
  };

  const t = {
    en: {
        title: "Daily Menu",
        available: "Available",
        soldOut: "Sold Out",
        offlineMsg: "Offline - Changes saved locally",
        onlineMsg: "Menu updated online",
        pendingSync: "Pending Sync"
    },
    ar: {
        title: "القائمة اليومية",
        available: "متوفر",
        soldOut: "نفذ",
        offlineMsg: "غير متصل - تم حفظ التغييرات محلياً",
        onlineMsg: "تم تحديث القائمة عبر الإنترنت",
        pendingSync: "في انتظار المزامنة"
    }
  };

  const strings = t[language];

  return (
    <div dir={language === 'ar' ? 'rtl' : 'ltr'} className="min-h-screen bg-[#F5F5F7] text-[#1D1D1F] font-inter flex flex-col items-center">
      <div className="w-[375px] max-w-[375px] min-h-screen bg-white relative overflow-hidden shadow-2xl flex flex-col">
          {/* Header */}
          <header className="bg-white/65 backdrop-blur-[30px] saturate-[210%] border-b border-white/40 sticky top-0 z-50 px-4 py-4 flex flex-col gap-2">
            {isOffline && (
                <div className="w-full bg-[#FF9500]/10 text-[#FF9500] px-3 py-2 rounded-lg text-sm font-bold text-center border border-[#FF9500]/30 animate-pulse">
                    {strings.offlineMsg}
                </div>
            )}
            {showToast && (
                <div className="w-full bg-[#34C759]/10 text-[#34C759] px-3 py-2 rounded-lg text-sm font-bold text-center border border-[#34C759]/30">
                    {strings.onlineMsg}
                </div>
            )}

            <div className="flex justify-between items-center">
                <h1 className="text-2xl font-bold font-outfit">{strings.title}</h1>
                <div className="flex items-center gap-2">
                    <button onClick={() => setLanguage(l => l === 'en' ? 'ar' : 'en')} className="text-sm font-bold text-[#0071E3]">
                        {language === 'en' ? 'عربي' : 'EN'}
                    </button>
                    <div className={offlineQueueCount > 0 ? "bg-[#FF9500]/20 text-[#FF9500] px-3 py-1 rounded-full text-xs font-bold border border-[#FF9500]/30" : "hidden"}>
                        {offlineQueueCount} {strings.pendingSync}
                    </div>
                </div>
            </div>
          </header>

          <main className="p-4 flex-1 overflow-y-auto pb-20">
            <div className="space-y-4">
              {menu.map(item => {
                const soldOut = item.is_sold_out || item.stock === 0 || item.available_quantity === 0;
                return (
                <div key={item.id} className="bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-gray-200 rounded-2xl p-4 shadow-sm flex flex-col gap-3">
                  <div className="flex justify-between items-center">
                      <h3 className={`font-bold font-outfit text-xl ${soldOut ? "text-gray-400 line-through" : "text-[#1D1D1F]"}`}>
                        {language === 'ar' && item.name_ar ? item.name_ar : item.name || item.title}
                      </h3>
                      {!soldOut && <span className="font-bold text-gray-500">${((item.price_cents || 0) / 100).toFixed(2)}</span>}
                  </div>
                  <button
                    id={`sold-out-toggle-${item.id}`}
                    onClick={() => handleToggleSoldOut(item.id, soldOut)}
                    className={`min-h-[44px] w-full px-4 rounded-xl font-bold text-lg active:scale-95 transition-all ${
                      soldOut
                        ? "bg-[#FF3B30] text-white shadow-md shadow-red-500/20"
                        : "bg-white text-[#0071E3] border-2 border-[#0071E3]/20 hover:bg-blue-50"
                    }`}
                  >
                    {soldOut ? strings.soldOut : strings.available}
                  </button>
                </div>
              )})}
              {menu.length === 0 && (
                  <div className="text-center py-10 text-gray-500 font-medium">
                      No menu items found.
                  </div>
              )}
            </div>
          </main>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800;900&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
