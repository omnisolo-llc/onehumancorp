/**
 * OptimisticMutationEngine
 * Unifies offline intent queue and sync logic across the OHC mobile-first frontend.
 * Enforces idempotency via UUID generation and maps payloads to the generic OfflineMutation structure.
 */

window.OptimisticMutationEngine = {
    QUEUE_KEY: "ohc_offline_queue",

    enqueueOfflineMutation(mutation) {
        if (!mutation.id) {
            mutation.id = crypto.randomUUID ? crypto.randomUUID() : 'intent_' + Date.now() + Math.random().toString();
        }
        if (!mutation.timestamp) {
            mutation.timestamp = Date.now();
        }

        let queue = [];
        try {
            queue = JSON.parse(localStorage.getItem(this.QUEUE_KEY) || "[]");
        } catch(e) {
            console.error("Failed to parse existing queue, resetting.", e);
        }

        queue.push(mutation);
        localStorage.setItem(this.QUEUE_KEY, JSON.stringify(queue));
        window.dispatchEvent(new Event("ohc_queue_updated"));

        if (navigator.onLine) {
            this.syncOfflineQueue();
        }
    },

    updateOfflineStatus() {
        const networkIndicator = document.getElementById("network-status-indicator");
        const queueIndicator = document.getElementById("queue-dashboard");

        if (networkIndicator) {
            if (!navigator.onLine) {
                networkIndicator.style.display = "block";
                networkIndicator.classList.remove("hidden");
                // Muted amber color style as requested
                networkIndicator.style.backgroundColor = "rgba(245, 158, 11, 0.9)";
                networkIndicator.style.color = "#fff";
                networkIndicator.innerText = "Offline Mode";
            } else {
                networkIndicator.style.display = "none";
                networkIndicator.classList.add("hidden");
            }
        }

        if (queueIndicator) {
            let queue = [];
            try {
                queue = JSON.parse(localStorage.getItem(this.QUEUE_KEY) || "[]");
            } catch(e) {}

            if (queue.length > 0) {
                queueIndicator.style.display = "block";
                queueIndicator.classList.remove("hidden");
                queueIndicator.innerHTML = `
                  <div style="display:flex; align-items:center; justify-content:center; gap: 8px;">
                    <div style="width: 16px; height: 16px; border: 2px dashed currentColor; border-radius: 50%; animation: spin 2s linear infinite;"></div>
                    <span>${queue.length} Pending Sync</span>
                  </div>
                  <style>
                    @keyframes spin { 100% { transform: rotate(360deg); } }
                  </style>
                `;
            } else {
                queueIndicator.style.display = "none";
                queueIndicator.classList.add("hidden");
            }
        }
    },

    async syncOfflineQueue() {
        if (!navigator.onLine) return;

        let queue = [];
        try {
            queue = JSON.parse(localStorage.getItem(this.QUEUE_KEY) || "[]");
        } catch(e) {}

        if (queue.length === 0) return;

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
                    timestamp: m.timestamp ? new Date(m.timestamp).toISOString() : new Date().toISOString()
                });
            } else if (m.type === "inventory_toggle") {
                generalMutations.push({
                    transaction_id: m.id,
                    product_id: m.product_id || m.id.replace("e2e-product-", ""),
                    quantity_deducted: m.quantity || 1,
                    amount: null,
                    payment_method: null,
                    payment_intent_id: null,
                    currency: null,
                    mutation_type: "inventory_toggle"
                });
            } else {
                 generalMutations.push({
                    transaction_id: m.id,
                    product_id: m.product_id || "unknown",
                    quantity_deducted: m.quantity || 0,
                    amount: m.amount || null,
                    payment_method: null,
                    payment_intent_id: null,
                    currency: m.currency || "usd",
                    mutation_type: m.type,
                    payload: m.payload || null
                });
            }
        });

        const tenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
        const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;

        let allOk = true;

        if (posTransactions.length > 0) {
            const sessionId = localStorage.getItem("ohc_active_terminal_session_id");
            try {
                const res = await fetch("/api/v1/payments/terminal/sync_offline", {
                    method: "POST",
                    headers: { "Content-Type": "application/json", "x-spiffe-id": spiffeId },
                    body: JSON.stringify({ session_id: sessionId || undefined, transactions: posTransactions })
                });
                if (!res.ok) allOk = false;
            } catch(e) {
                console.error("POS Offline Sync error:", e);
                allOk = false;
            }
        }

        if (generalMutations.length > 0) {
            try {
                const res = await fetch("/api/v1/sync/offline", {
                    method: "POST",
                    headers: { "Content-Type": "application/json", "x-spiffe-id": spiffeId },
                    body: JSON.stringify({ mutations: generalMutations })
                });
                if (!res.ok) allOk = false;
            } catch(e) {
                console.error("General Offline Sync error:", e);
                allOk = false;
            }
        }

        if (allOk) {
            localStorage.setItem(this.QUEUE_KEY, "[]");
            this.updateOfflineStatus();
        }
    },

    init() {
        window.addEventListener("online", () => {
            this.updateOfflineStatus();
            this.syncOfflineQueue();
        });
        window.addEventListener("offline", () => this.updateOfflineStatus());
        window.addEventListener("ohc_queue_updated", () => this.updateOfflineStatus());
        this.updateOfflineStatus();
    }
};

// Auto-initialize when the script loads
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => window.OptimisticMutationEngine.init());
} else {
    window.OptimisticMutationEngine.init();
}

// Global aliases for backward compatibility with existing inline scripts
window.enqueueOfflineMutation = (mutation) => window.OptimisticMutationEngine.enqueueOfflineMutation(mutation);
window.updateOfflineStatus = () => window.OptimisticMutationEngine.updateOfflineStatus();
window.syncOfflineQueue = () => window.OptimisticMutationEngine.syncOfflineQueue();
