const CACHE_NAME = 'ohc-offline-cache-v1';
const STATIC_ASSETS = [
  '/',
  '/dashboard',
  '/storefront-builder',
  '/manifest.json'
];

self.addEventListener('install', (event) => {
  self.skipWaiting();
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => {
      return cache.addAll(STATIC_ASSETS);
    })
  );
});

self.addEventListener('activate', (event) => {
  event.waitUntil(self.clients.claim());
});

self.addEventListener('fetch', (event) => {
  if (event.request.method !== 'GET') return;

  event.respondWith(
    fetch(event.request)
      .then((response) => {
        // Cache successful GET responses
        if (response.status === 200) {
          const responseClone = response.clone();
          caches.open(CACHE_NAME).then((cache) => {
            cache.put(event.request, responseClone);
          });
        }
        return response;
      })
      .catch(() => {
        // Fallback to cache
        return caches.match(event.request);
      })
  );
});

// IndexedDB Utility for Service Worker
function openDB() {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open('ohc_offline_db', 1);
    request.onerror = () => reject(request.error);
    request.onsuccess = () => resolve(request.result);
  });
}

async function getQueue() {
  try {
    const db = await openDB();
    return new Promise((resolve, reject) => {
      const transaction = db.transaction(['offline_queue'], 'readonly');
      const store = transaction.objectStore('offline_queue');
      const request = store.getAll();
      request.onsuccess = () => resolve(request.result);
      request.onerror = () => reject(request.error);
    });
  } catch (err) {
    return [];
  }
}

async function clearQueue() {
  try {
    const db = await openDB();
    return new Promise((resolve, reject) => {
      const transaction = db.transaction(['offline_queue'], 'readwrite');
      const store = transaction.objectStore('offline_queue');
      const request = store.clear();
      request.onsuccess = () => resolve();
      request.onerror = () => reject(request.error);
    });
  } catch (err) {
    console.error('Failed to clear queue', err);
  }
}

self.addEventListener('sync', (event) => {
  if (event.tag === 'ohc-offline-sync') {
    event.waitUntil(
      (async () => {
        const queue = await getQueue();
        if (queue && queue.length > 0) {
           const formattedMutations = queue.map((item) => {
             return {
                 transaction_id: item.idempotency_key || item.id || `txn_${Date.now()}`,
                 product_id: item.product_id || item.id || 'unknown',
                 quantity_deducted: item.quantity_deducted || item.amount || 1
             }
          });

          try {
             const res = await fetch("/api/v1/sync/offline", {
                method: "POST",
                headers: {
                  "Content-Type": "application/json",
                },
                body: JSON.stringify({ mutations: formattedMutations }),
             });

             if (res.ok) {
                await clearQueue();

                // Notify clients
                const clients = await self.clients.matchAll();
                for (const client of clients) {
                  client.postMessage({ type: 'SYNC_COMPLETE' });
                }

                // Show notification
                self.registration.showNotification("Offline Sync Complete", {
                    body: "Your offline orders have been successfully synced.",
                    icon: "/icon-192x192.png",
                });
             } else {
                 throw new Error("Failed to sync");
             }
          } catch(err) {
              console.error("Background sync failed", err);
              throw err; // Re-throw to trigger retry
          }
        }
      })()
    );
  }
});

self.addEventListener('push', (event) => {
  const data = event.data ? event.data.json() : { title: 'Notification', body: 'New update!' };
  const options = {
    body: data.body,
    icon: '/icon-192x192.png',
  };
  event.waitUntil(
    self.registration.showNotification(data.title, options)
  );
});