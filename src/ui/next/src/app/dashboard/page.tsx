"use client";

import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { AppShell } from "../components/AppShell";
import { InteractiveWalkthrough, WalkthroughTarget } from "../../components/Walkthrough";
import { WithTooltip } from "../../components/TooltipRegistry";
import { OneTapReferral } from "../components/OneTapReferral";
import { UnifiedAgentFeed } from "./UnifiedAgentFeed";

type DashboardMetrics = {
  active_customers: number;
  pending_orders: number;
  total_sales: number;
  total_campaigns_sent?: number;
};

type Order = {
  id: string;
  customer_name?: string;
  total_amount?: number;
  status?: string;
  created_at?: string;
};

type InboxMessage = {
  id: string;
  source?: string;
  content?: string;
  draft_reply?: string;
  status?: string;
  created_at?: string;
};

type SupplyPayload = {
  vendors: unknown[];
  raw_materials: Array<{
    id: string;
    name: string;
    current_quantity: number;
    reorder_threshold: number;
  }>;
  bom_items: unknown[];
};

const emptyMetrics: DashboardMetrics = {
  active_customers: 0,
  pending_orders: 0,
  total_sales: 0,
  total_campaigns_sent: 0,
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

function money(value: number | undefined) {
  return new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" }).format(value || 0);
}

function statusTone(status?: string) {
  const normalized = (status || "").toLowerCase();
  if (["paid", "completed", "shipped", "delivered"].includes(normalized)) return "good";
  if (["pending", "unfulfilled", "open"].includes(normalized)) return "warn";
  if (["failed", "cancelled", "canceled"].includes(normalized)) return "bad";
  return "neutral";
}

export default function Dashboard() {
  const [metrics, setMetrics] = useState<DashboardMetrics>(emptyMetrics);
  const [orders, setOrders] = useState<Order[]>([]);
  const [messages, setMessages] = useState<InboxMessage[]>([]);
  const [supply, setSupply] = useState<SupplyPayload>({ vendors: [], raw_materials: [], bom_items: [] });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [isOffline, setIsOffline] = useState(false);
  const [offlineQueueCount, setOfflineQueueCount] = useState(0);
  const [isWalkthroughOpen, setIsWalkthroughOpen] = useState(false);
  const [userName, setUserName] = useState("Human");

  useEffect(() => {
    try {
      const storedName = localStorage.getItem("user_name");
      if (storedName) {
        setUserName(storedName);
      }
    } catch {
      // ignore
    }

    const updateOfflineStatus = () => {
      setIsOffline(!navigator.onLine);
      try {
        setOfflineQueueCount(JSON.parse(localStorage.getItem("ohc_offline_queue") || "[]").length);
      } catch {
        setOfflineQueueCount(0);
      }
    };

    async function loadDashboard() {
      const tenant = encodeURIComponent(tenantId());
      setLoading(true);
      setError("");

      try {
        const [metricsRes, ordersRes, inboxRes, supplyRes] = await Promise.all([
          fetch(`/api/ui/dashboard/metrics?tenant_id=${tenant}`),
          fetch(`/api/ui/orders?tenant_id=${tenant}`),
          fetch(`/api/ui/inbox/messages?tenant_id=${tenant}`),
          fetch(`/api/ui/supply?tenant_id=${tenant}`),
        ]);

        if (!metricsRes.ok || !ordersRes.ok || !inboxRes.ok || !supplyRes.ok) {
          throw new Error("One or more database-backed UI endpoints failed");
        }

        const [metricsData, ordersData, inboxData, supplyData] = await Promise.all([
          metricsRes.json(),
          ordersRes.json(),
          inboxRes.json(),
          supplyRes.json(),
        ]);

        setMetrics({ ...emptyMetrics, ...metricsData });
        setOrders(Array.isArray(ordersData) ? ordersData : []);
        setMessages(Array.isArray(inboxData) ? inboxData : []);
        setSupply({
          vendors: Array.isArray(supplyData?.vendors) ? supplyData.vendors : [],
          raw_materials: Array.isArray(supplyData?.raw_materials) ? supplyData.raw_materials : [],
          bom_items: Array.isArray(supplyData?.bom_items) ? supplyData.bom_items : [],
        });
      } catch (e: any) {
        setError(e?.message || "Failed to load dashboard data");
      } finally {
        setLoading(false);
      }
    }

    updateOfflineStatus();
    loadDashboard();
    window.addEventListener("online", updateOfflineStatus);
    window.addEventListener("offline", updateOfflineStatus);
    window.addEventListener("storage", updateOfflineStatus);

    return () => {
      window.removeEventListener("online", updateOfflineStatus);
      window.removeEventListener("offline", updateOfflineStatus);
      window.removeEventListener("storage", updateOfflineStatus);
    };
  }, []);

  const lowStockCount = useMemo(
    () => supply.raw_materials.filter((item) => item.current_quantity <= item.reorder_threshold).length,
    [supply.raw_materials],
  );

  const statusItems = [
    { label: "API", value: error ? "Degraded" : "Online", tone: error ? "bad" as const : "good" as const },
    { label: "Orders", value: String(metrics.pending_orders || 0), tone: metrics.pending_orders > 0 ? "warn" as const : "good" as const },
    { label: "Stock", value: String(lowStockCount), tone: lowStockCount > 0 ? "warn" as const : "good" as const },
    { label: "Growth", value: "Active", tone: "good" as const },
  ];

  const walkthroughSteps = [
    {
      targetId: "sales-card-target",
      title: "Business Analytics",
      content: "This panel reads sales and customer counts from the database-backed dashboard endpoint.",
      position: "bottom" as const,
    },
    {
      targetId: "operations-map-target",
      title: "Operations Map",
      content: "Use this area to see the live state of orders, inbox, and inventory from your database.",
      position: "bottom" as const,
    },
  ];

  return (
    <AppShell
      title="Dashboard"
      subtitle="Network-style command center for database-backed store operations."
      statusItems={statusItems}
      actions={[
        { label: "New Product", href: "/products/new", primary: true },
      ]}
    >
      <div className="mb-6 p-6 rounded-[16px] mac-glass-container border border-white/40 dark:border-white/10">
        <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Welcome back, {userName}.</h2>
        <p className="text-gray-600 dark:text-gray-400">Your AI assistants are working on your behalf.</p>
      </div>

      <InteractiveWalkthrough
        steps={walkthroughSteps}
        isOpen={isWalkthroughOpen}
        onClose={() => setIsWalkthroughOpen(false)}
      />

      <div className="mb-4 flex flex-wrap gap-2">
        <Link href="/settings" className="app-button">Settings</Link>
        <Link href="/agents" className="app-button">Manage AI Assistants</Link>
        <Link href="/website-builder" className="app-button">Website Builder</Link>
        <button type="button" onClick={() => setIsWalkthroughOpen(true)} className="app-button">
          Start Tour
        </button>
        <div id="queue-dashboard" className={offlineQueueCount > 0 ? "app-badge warn" : "hidden"}>
          {offlineQueueCount} payments pending sync
        </div>
        <div id="network-status-indicator" className={isOffline ? "app-badge warn" : "hidden"}>
          Offline - changes saved locally
        </div>
        {error && <div className="app-badge bad">{error}</div>}
      </div>

      <div className="mb-6">
        <OneTapReferral tenantId={tenantId()} source="dashboard" />
      </div>

      <main id="dashboard-screen" className="app-grid" style={{ gap: 16 }}>
        <UnifiedAgentFeed />

        <section>
          <div className="mb-3 flex items-center justify-between gap-3">
            <div>
              <h2 className="app-panel-title">Business Analytics</h2>
              <p className="app-list-subtitle">Loaded from `/api/ui/dashboard/metrics`.</p>
            </div>
          </div>
          <div className="app-grid metrics">
            <WalkthroughTarget id="sales-card-target" className="app-card">
              <WithTooltip id="total-sales-tooltip" defaultText="Total revenue generated from database orders.">
                <div className="app-metric-label">Total Sales</div>
              </WithTooltip>
              <div className="app-metric-value">{money(metrics.total_sales)}</div>
              <div className="app-metric-note">{loading ? "Loading database rows" : "All recorded orders"}</div>
            </WalkthroughTarget>
            <div className="app-card">
              <div className="app-metric-label">Customers</div>
              <div className="app-metric-value">{metrics.active_customers}</div>
              <div className="app-metric-note">Database customer records</div>
            </div>
            <div className="app-card">
              <div className="app-metric-label">Pending Orders</div>
              <div className="app-metric-value">{metrics.pending_orders}</div>
              <div className="app-metric-note">Open fulfillment workload</div>
            </div>
            <div className="app-card">
              <div className="app-metric-label">Low Stock</div>
              <div className="app-metric-value">{lowStockCount}</div>
              <div className="app-metric-note">Materials below threshold</div>
            </div>
          </div>
        </section>

        <section className="app-grid two">
          <WalkthroughTarget id="operations-map-target" className="app-panel">
            <div className="app-panel-header">
              <div>
                <div className="app-panel-title">Operations Map</div>
                <div className="app-list-subtitle">Live database state across the store workflow.</div>
              </div>
              <Link href="/orders" className="app-button">Open Orders</Link>
            </div>
            <div className="app-panel-body">
              <div className="grid grid-cols-1 gap-3 md:grid-cols-3">
                <div className="app-card">
                  <div className="app-metric-label">Orders</div>
                  <div className="app-metric-value">{orders.length}</div>
                  <div className="app-metric-note">Rows returned</div>
                </div>
                <div className="app-card">
                  <div className="app-metric-label">Inbox</div>
                  <div className="app-metric-value">{messages.length}</div>
                  <div className="app-metric-note">Messages returned</div>
                </div>
                <div className="app-card">
                  <div className="app-metric-label">Vendors</div>
                  <div className="app-metric-value">{supply.vendors.length}</div>
                  <div className="app-metric-note">Supply partners</div>
                </div>
              </div>
            </div>
          </WalkthroughTarget>

          <div className="app-panel">
            <div className="app-panel-header">
              <div className="app-panel-title">Action Required</div>
              <Link href="/inventory" className="app-button">Inventory</Link>
            </div>
            <div className="app-list">
              {metrics.pending_orders > 0 && (
                <div className="app-list-item">
                  <div>
                    <div className="app-list-title">Pending fulfillment</div>
                    <div className="app-list-subtitle">{metrics.pending_orders} order records need attention.</div>
                  </div>
                  <span className="app-badge warn">Orders</span>
                </div>
              )}
              {lowStockCount > 0 && (
                <div className="app-list-item">
                  <div>
                    <div className="app-list-title">Low stock</div>
                    <div className="app-list-subtitle">{lowStockCount} material records are below reorder threshold.</div>
                  </div>
                  <span className="app-badge warn">Supply</span>
                </div>
              )}
              {messages.some((message) => (message.status || "").toLowerCase() !== "closed") && (
                <div className="app-list-item">
                  <div>
                    <div className="app-list-title">Inbox messages</div>
                    <div className="app-list-subtitle">Open customer conversations are waiting in the database.</div>
                  </div>
                  <span className="app-badge">Inbox</span>
                </div>
              )}
              {!loading && metrics.pending_orders === 0 && lowStockCount === 0 && messages.length === 0 && (
                <div className="app-empty">No database-backed actions are currently open.</div>
              )}
            </div>
          </div>
        </section>

        <section className="app-grid two">
          <div className="app-panel">
            <div className="app-panel-header">
              <div className="app-panel-title">Recent Orders</div>
              <Link href="/orders" className="app-button">View All</Link>
            </div>
            {orders.length === 0 ? (
              <div className="app-empty">{loading ? "Loading orders from the database..." : "No order rows found for this tenant."}</div>
            ) : (
              <div className="app-table-wrap">
                <table className="app-table">
                  <thead>
                    <tr>
                      <th>Order</th>
                      <th>Customer</th>
                      <th>Total</th>
                      <th>Status</th>
                    </tr>
                  </thead>
                  <tbody>
                    {orders.slice(0, 8).map((order) => (
                      <tr key={order.id}>
                        <td><Link href={`/orders/${order.id}`} className="font-semibold text-blue-700">{order.id}</Link></td>
                        <td>{order.customer_name || "Unknown"}</td>
                        <td>{money(order.total_amount)}</td>
                        <td><span className={`app-badge ${statusTone(order.status)}`}>{order.status || "Unknown"}</span></td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>

          <div className="app-panel">
            <div className="app-panel-header">
              <div className="app-panel-title">Inbox Activity</div>
              <Link href="/inbox" className="app-button">Open Inbox</Link>
            </div>
            <div className="app-list">
              {messages.length === 0 ? (
                <div className="app-empty">{loading ? "Loading inbox from the database..." : "No inbox message rows found for this tenant."}</div>
              ) : messages.slice(0, 6).map((message) => (
                <div key={message.id} className="app-list-item">
                  <div>
                    <div className="app-list-title">{message.source || "Unknown source"}</div>
                    <div className="app-list-subtitle">{message.content || "Empty message"}</div>
                  </div>
                  <span className={`app-badge ${statusTone(message.status)}`}>{message.status || "Open"}</span>
                </div>
              ))}
            </div>
          </div>
        </section>

        <section className="mt-4">
          <div className="mb-3 flex items-center justify-between gap-3">
            <div>
              <h2 className="app-panel-title">Growth & Virality</h2>
              <p className="app-list-subtitle">Unlock new customers and track milestones.</p>
            </div>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <Link href="/referrals" className="block mac-glass-container p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🤝</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Earn $50</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Referrals</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Invite other business owners to OHC and earn premium credits.</p>
            </Link>

            <Link href="/milestones" className="block mac-glass-container p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-purple-50 dark:bg-purple-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🏆</div>
                <div className="text-purple-600 dark:text-purple-400 font-semibold text-sm bg-purple-50 dark:bg-purple-900/30 px-3 py-1 rounded-full">Share</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Milestones</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Track and share your business achievements with your audience.</p>
            </Link>

            <Link href="/share-cards" className="block mac-glass-container p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-pink-50 dark:bg-pink-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🎴</div>
                <div className="text-pink-600 dark:text-pink-400 font-semibold text-sm bg-pink-50 dark:bg-pink-900/30 px-3 py-1 rounded-full">Cards</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Social Share Cards</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Generate Share Cards to promote your brand on social media.</p>
            </Link>
          </div>
        </section>
      </main>
    </AppShell>
  );
}
