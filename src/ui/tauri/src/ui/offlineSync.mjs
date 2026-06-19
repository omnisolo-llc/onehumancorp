const DB_NAME = "OHC_Offline_Queue";
const STORE_NAME = "actions";
const DB_VERSION = 1;

let syncInProgress = false;

function getDB() {
    return new Promise((resolve, reject) => {
        const request = window.indexedDB.open(DB_NAME, DB_VERSION);
        request.onerror = (event) => {
            console.error("IndexedDB error", event);
            reject(request.error);
        };
        request.onsuccess = (event) => {
            resolve(event.target.result);
        };
        request.onupgradeneeded = (event) => {
            const db = event.target.result;
            if (!db.objectStoreNames.contains(STORE_NAME)) {
                db.createObjectStore(STORE_NAME, { keyPath: "id" });
            }
        };
    });
}

export async function getQueue() {
    if (!window.indexedDB) return [];
    try {
        const db = await getDB();
        return new Promise((resolve, reject) => {
            const transaction = db.transaction([STORE_NAME], "readonly");
            const store = transaction.objectStore(STORE_NAME);
            const request = store.getAll();
            request.onsuccess = () => resolve(request.result);
            request.onerror = () => reject(request.error);
        });
    } catch (e) {
        console.error("Failed to get actions", e);
        return [];
    }
}

export async function enqueueOfflineMutation(mutation) {
    if (!mutation.id) {
        mutation.id = crypto.randomUUID ? crypto.randomUUID() : Date.now().toString() + Math.random().toString().substring(2);
    }
    if (!mutation.timestamp) {
        mutation.timestamp = Date.now();
    }
    try {
        const db = await getDB();
        await new Promise((resolve, reject) => {
            const transaction = db.transaction([STORE_NAME], "readwrite");
            const store = transaction.objectStore(STORE_NAME);
            const request = store.put(mutation);
            request.onsuccess = () => resolve();
            request.onerror = () => reject(request.error);
        });
        window.dispatchEvent(new Event("ohc_queue_updated"));
    } catch (e) {
        console.error("Failed to enqueue offline mutation", e);
    }

    if (navigator.onLine) {
        syncOfflineQueue();
    }
}

export async function updateOfflineStatus() {
    const networkIndicator = document.getElementById("network-status-indicator");
    const queueIndicator = document.getElementById("queue-dashboard");
    if (!networkIndicator || !queueIndicator) return;

    const queue = await getQueue();

    const dot = document.getElementById("network-status-dot");
    const text = document.getElementById("network-status-text");

    if (!navigator.onLine) {
        networkIndicator.style.display = "flex";
        networkIndicator.classList.remove("hidden");
        if (dot) dot.className = "w-2 h-2 rounded-full bg-orange-500";
        if (text) text.innerText = "Working Offline. Changes saved.";
    } else if (queue.length > 0) {
        networkIndicator.style.display = "flex";
        networkIndicator.classList.remove("hidden");
        if (dot) dot.className = "w-2 h-2 rounded-full bg-blue-500 animate-pulse";
        if (text) text.innerText = `Syncing ${queue.length} action${queue.length !== 1 ? 's' : ''}...`;
    } else {
        networkIndicator.style.display = "none";
        networkIndicator.classList.add("hidden");
    }

    if (queue.length > 0) {
        queueIndicator.style.display = "block";
        queueIndicator.classList.remove("hidden");
        queueIndicator.innerText = queue.length + " Items Pending Sync";
    } else {
        queueIndicator.style.display = "none";
        queueIndicator.classList.add("hidden");
    }
}

export async function syncOfflineQueue(retryCount = 0) {
    if (!navigator.onLine || syncInProgress) return;
    const queue = await getQueue();
    if (queue.length === 0) return;

    syncInProgress = true;

    try {
        const posTransactions = [];
        const generalMutations = [];
        queue.forEach(m => {
            if (m.type === "tap_to_pay") {
                posTransactions.push({
                    id: m.id,
                    client_id: "terminal_client",
                    amount_cents: Math.round(m.amount),
                    currency: m.currency || "usd",
                    payload: JSON.stringify([{ product_id: m.product_id, quantity: m.quantity || 1 }]),
                    timestamp: new Date(m.timestamp || Date.now()).toISOString()
                });
            } else if (m.type === "inventory_toggle") {
                generalMutations.push({
                    transaction_id: m.id,
                    product_id: m.id.replace("e2e-product-", ""),
                    quantity_deducted: 1,
                    amount: null,
                    payment_method: null,
                    payment_intent_id: null,
                    currency: null
                });
            } else {
                generalMutations.push(m);
            }
        });

        const tenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
        const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;
        let allOk = true;

        if (posTransactions.length > 0) {
            const res = await fetch("/api/v1/payments/terminal/sync_offline", {
                method: "POST",
                headers: { "Content-Type": "application/json", "x-spiffe-id": spiffeId },
                body: JSON.stringify({ transactions: posTransactions })
            });
            if (!res.ok) allOk = false;
        }

        if (generalMutations.length > 0) {
            let res;
            if (window.offlineSyncMode === "mcp-deltas") {
                const deltas = generalMutations.map(m => ({
                    id: crypto.randomUUID(),
                    entity_id: m.product_id || m.transaction_id || "general",
                    data: JSON.stringify(m),
                    updated_at: new Date().toISOString()
                }));
                res = await fetch("/api/v1/sync/mcp-deltas", {
                    method: "POST",
                    headers: { "Content-Type": "application/json", "x-spiffe-id": spiffeId },
                    body: JSON.stringify({ tenant_id: tenantId, deltas: deltas })
                });
            } else {
                res = await fetch("/api/v1/sync/offline", {
                    method: "POST",
                    headers: { "Content-Type": "application/json", "x-spiffe-id": spiffeId },
                    body: JSON.stringify({ mutations: generalMutations })
                });
            }
            if (!res.ok) allOk = false;
        }

        if (allOk) {
            const db = await getDB();
            await new Promise((resolve, reject) => {
                const transaction = db.transaction([STORE_NAME], "readwrite");
                const store = transaction.objectStore(STORE_NAME);
                const request = store.clear();
                request.onsuccess = () => resolve();
                request.onerror = () => reject(request.error);
            });
            syncInProgress = false;
            updateOfflineStatus();
        } else {
            throw new Error("Sync API returned non-OK status.");
        }
    } catch(e) {
        console.error("Failed to sync offline queue", e);
        const maxRetries = 5;
        if (retryCount < maxRetries) {
            const delay = 1000 * Math.pow(2, retryCount);
            console.log(`Retrying sync in ${delay}ms... (Attempt ${retryCount + 1})`);
            setTimeout(() => {
                syncInProgress = false;
                syncOfflineQueue(retryCount + 1);
            }, delay);
            return;
        } else {
            console.error("Max retries reached for offline sync.");
            syncInProgress = false;
        }
    }
}

export function initializeOfflineSync() {
    window.addEventListener("online", () => {
        updateOfflineStatus();
        syncOfflineQueue();
    });
    window.addEventListener("offline", updateOfflineStatus);
    window.addEventListener("ohc_queue_updated", updateOfflineStatus);
    updateOfflineStatus();
}

window.enqueueOfflineMutation = enqueueOfflineMutation;
window.getQueue = getQueue;
window.updateOfflineStatus = updateOfflineStatus;
window.syncOfflineQueue = syncOfflineQueue;
window.initializeOfflineSync = initializeOfflineSync;

initializeOfflineSync();
