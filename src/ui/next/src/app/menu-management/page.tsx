"use client";

import React, { useState, useEffect } from "react";
import { AppShell } from "../components/AppShell";
import { SyncManager } from "../../lib/sync/SyncManager";

// IndexedDB helper for menu cache
const DB_NAME = "OHC_Menu_Cache";
const STORE_NAME = "menu";
const DB_VERSION = 1;

function getMenuDB(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = window.indexedDB.open(DB_NAME, DB_VERSION);
    request.onerror = () => reject(request.error);
    request.onsuccess = (event) => resolve((event.target as IDBOpenDBRequest).result);
    request.onupgradeneeded = (event) => {
      const db = (event.target as IDBOpenDBRequest).result;
      if (!db.objectStoreNames.contains(STORE_NAME)) {
        db.createObjectStore(STORE_NAME, { keyPath: "id" });
      }
    };
  });
}

async function saveMenuToCache(items: any[]) {
  if (typeof window === "undefined" || !window.indexedDB) return;
  try {
    const db = await getMenuDB();
    const transaction = db.transaction([STORE_NAME], "readwrite");
    const store = transaction.objectStore(STORE_NAME);
    store.clear();
    items.forEach(item => store.put(item));
  } catch (err) {
    console.warn("Failed to cache menu", err);
  }
}

async function loadMenuFromCache(): Promise<any[]> {
  if (typeof window === "undefined" || !window.indexedDB) return [];
  try {
    const db = await getMenuDB();
    return new Promise((resolve) => {
      const transaction = db.transaction([STORE_NAME], "readonly");
      const store = transaction.objectStore(STORE_NAME);
      const request = store.getAll();
      request.onsuccess = () => resolve(request.result);
      request.onerror = () => resolve([]);
    });
  } catch (err) {
    return [];
  }
}

export default function MenuManagementView() {
  const [menu, setMenu] = useState<any[]>([]);
  const [offlineQueueCount, setOfflineQueueCount] = useState(0);
  const [isOffline, setIsOffline] = useState(false);
  const [showSyncSuccess, setShowSyncSuccess] = useState(false);

  useEffect(() => {
    SyncManager.getInstance();

    const fetchMenu = async () => {
      try {
        const tenantId = localStorage.getItem("tenant_id") || "default";

        if (!navigator.onLine) {
           const cached = await loadMenuFromCache();
           if (cached.length > 0) {
              setMenu(cached);
              return;
           }
        }

        const menuRes = await fetch("/api/pos/inventory", {
           headers: { "x-tenant-id": tenantId }
        });
        if (menuRes.ok) {
           const data = await menuRes.json();
           const items = data.items || data.inventory || data || [];
           setMenu(items);
           await saveMenuToCache(items);
        } else {
           const cached = await loadMenuFromCache();
           setMenu(cached);
        }
      } catch (err) {
        console.error("Failed to fetch menu data", err);
        const cached = await loadMenuFromCache();
        setMenu(cached);
      }
    };

    fetchMenu();

    let previousQueueCount = 0;

    const updateCount = async () => {
      const count = await SyncManager.getInstance().getQueueLength();
      if (previousQueueCount > 0 && count === 0 && navigator.onLine) {
         setShowSyncSuccess(true);
         setTimeout(() => setShowSyncSuccess(false), 3000);
      }
      previousQueueCount = count;
      setOfflineQueueCount(count);
    };

    const handleOffline = () => setIsOffline(true);
    const handleOnline = () => {
      setIsOffline(false);
      SyncManager.getInstance().sync();
    };

    setIsOffline(!navigator.onLine);

    SyncManager.getInstance().getQueueLength().then(c => {
       previousQueueCount = c;
       setOfflineQueueCount(c);
    });

    window.addEventListener("ohc_queue_updated", updateCount);
    window.addEventListener("offline", handleOffline);
    window.addEventListener("online", handleOnline);

    return () => {
      window.removeEventListener("ohc_queue_updated", updateCount);
      window.removeEventListener("offline", handleOffline);
      window.removeEventListener("online", handleOnline);
    };
  }, []);

  const handleToggleSoldOut = async (itemId: string, currentStatus: boolean) => {
    const updatedMenu = menu.map(m => m.id === itemId ? { ...m, is_sold_out: !currentStatus } : m);
    setMenu(updatedMenu);
    saveMenuToCache(updatedMenu);

    await SyncManager.getInstance().enqueue({
      id: `e2e-product-${itemId}-${Date.now()}`,
      type: "TOGGLE_SOLD_OUT",
      payload: { item_id: itemId, is_sold_out: !currentStatus },
      timestamp: Date.now()
    });
    setShowSyncSuccess(false);
  };

  return (
    <AppShell title="Menu Management">
      <div className="min-h-screen bg-[#F5F5F7] text-[#1D1D1F] font-inter">
        {isOffline && (
          <div className="bg-[#FF9500] text-white font-bold text-center py-2 px-4 shadow-md sticky top-0 z-50">
             Offline - Changes saved locally
          </div>
        )}
        <header className="bg-white/65 backdrop-blur-[30px] saturate-[210%] border-b border-white/40 sticky top-0 z-40 px-4 py-4 flex justify-between items-center shadow-sm">
          <h1 className="text-xl font-bold font-outfit">Today's Active Menu</h1>
          {showSyncSuccess && (
            <div id="queue-dashboard" className="bg-[#34C759]/20 text-[#34C759] px-3 py-1 rounded-full text-sm font-bold border border-[#34C759]/30">
              Menu updated online
            </div>
          )}
          {offlineQueueCount > 0 && (
            <div id="queue-dashboard" className="bg-[#FF9500]/20 text-[#FF9500] px-3 py-1 rounded-full text-sm font-medium border border-[#FF9500]/30">
              {offlineQueueCount} Pending Sync
            </div>
          )}
        </header>

        <main className="p-4 flex flex-col gap-6 max-w-[375px] mx-auto md:max-w-3xl">
          <section className="w-full">
            <div className="space-y-4">
              {menu.map(item => {
                const soldOut = item.is_sold_out || item.stock === 0;
                return (
                <div key={item.id} className="bg-white/65 backdrop-blur-[30px] saturate-[210%] rounded-xl border border-white/40 p-5 shadow-md flex items-center justify-between transition-all">
                  <div className="flex flex-col">
                    <h3 className={`font-bold font-outfit text-xl ${soldOut ? "text-gray-400 line-through" : "text-[#1D1D1F]"}`}>
                      {item.name || item.title}
                    </h3>
                    {item.price_cents && (
                       <span className="text-sm font-medium text-gray-500 mt-1">
                          {(item.price_cents / 100).toLocaleString('en-US', { style: 'currency', currency: item.currency || 'USD' })}
                       </span>
                    )}
                  </div>
                  <button
                    id={`sold-out-toggle-${item.id}`}
                    onClick={() => handleToggleSoldOut(item.id, soldOut)}
                    className={`min-h-[44px] min-w-[120px] h-[44px] px-6 rounded-full font-bold text-sm shadow-sm active:scale-95 transition-all ${
                      soldOut
                        ? "bg-white text-[#FF3B30] border-2 border-[#FF3B30]"
                        : "bg-[#34C759] text-white border border-[#34C759]"
                    }`}
                  >
                    {soldOut ? "Sold Out" : "Available"}
                  </button>
                </div>
              )})}

              {menu.length === 0 && (
                <div className="text-center py-12 text-gray-500 font-medium">No items found in your menu.</div>
              )}
            </div>
          </section>
        </main>
      </div>
    </AppShell>
  );
}
