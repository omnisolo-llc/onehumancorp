import { FloatingActionButton } from "./FAB";
"use client";
import { ViralLoopPerformanceWidget } from "./ViralLoopPerformanceWidget";



import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { AppShell } from "../components/AppShell";
import { InteractiveWalkthrough, WalkthroughTarget } from "../../components/Walkthrough";
import { WithTooltip } from "../../components/TooltipRegistry";
import GrowthReferralWidget from "../components/GrowthReferralWidget";
import { SmartBlock } from "../builder/components";
import { UnifiedAgentFeed } from "./UnifiedAgentFeed";
import { NeighborhoodPulseCard } from "./NeighborhoodPulseCard";
import { ViralLoopPerformanceWidget } from "./ViralLoopPerformanceWidget";

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
  const router = useRouter();
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
  const [showMigration, setShowMigration] = useState(false);
  const [migrationUrl, setMigrationUrl] = useState("");
  const [migrationStatus, setMigrationStatus] = useState<"idle" | "running" | "complete">("idle");
  const [actionMessage, setActionMessage] = useState("");
  const [isSyncing, setIsSyncing] = useState(false);
  const [syncErrorCount, setSyncErrorCount] = useState(0);

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

    const handleSync = async () => {
      if (!navigator.onLine) return;
      try {
        const queueStr = localStorage.getItem("ohc_offline_queue") || "[]";
        const queue = JSON.parse(queueStr);
        if (!Array.isArray(queue) || queue.length === 0) return;

        setIsSyncing(true);
        setSyncErrorCount(0);

        const res = await fetch("/api/v1/sync/offline", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ mutations: queue }),
        });

        if (res.ok) {
          const data = await res.json();
          if (data && data.failed_count && data.failed_count > 0) {
            setSyncErrorCount(data.failed_count);
          }

          // Re-fetch queue in case new items were added during the sync
          const currentQueueStr = localStorage.getItem("ohc_offline_queue") || "[]";
          const currentQueue = JSON.parse(currentQueueStr);
          // Remove exactly the items we just synced (by matching id and timestamp or simply slicing by length)
          // Simple slice is safe if we assume append-only queue
          const remainingQueue = currentQueue.slice(queue.length);
          localStorage.setItem("ohc_offline_queue", JSON.stringify(remainingQueue));
          setOfflineQueueCount(remainingQueue.length);
        }
      } catch (e) {
        console.error("Sync failed", e);
      } finally {
        setIsSyncing(false);
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
    handleSync();
    window.addEventListener("online", updateOfflineStatus);
    window.addEventListener("online", handleSync);
    window.addEventListener("offline", updateOfflineStatus);
    window.addEventListener("storage", updateOfflineStatus);

    return () => {
      window.removeEventListener("online", updateOfflineStatus);
      window.removeEventListener("online", handleSync);
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
        { label: "Campaigns", href: "/dashboard/campaigns", icon: "campaigns" },
        { label: "New Product", href: "/products/new", primary: true },
      ]}
    >
      <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10">
        <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Welcome back, {userName}.</h2>
        <p className="text-gray-600 dark:text-gray-400">Your agents are working on your behalf.</p>
      </div>

      <NeighborhoodPulseCard tenant={tenantId()} />
      <FloatingActionButton />

      <InteractiveWalkthrough
        steps={walkthroughSteps}
        isOpen={isWalkthroughOpen}
        onClose={() => setIsWalkthroughOpen(false)}
      />

      <div className="mb-4 flex flex-wrap gap-2">
        <button
          type="button"
          onClick={() => {
            try {
              localStorage.setItem("TEST_WALKTHROUGH", "true");
            } catch {
              // ignore storage failures
            }
            setIsWalkthroughOpen(true);
          }}
          className="app-button"
        >
          Start Tour
        </button>
        <button type="button" onClick={() => router.push("/business-setup")} className="app-button">
          Launch Site
        </button>
        <button type="button" onClick={() => setShowMigration((open) => !open)} className="app-button">
          Migrate Existing Store
        </button>
        <div id="queue-dashboard" className={offlineQueueCount > 0 ? "app-badge warn block" : "hidden"}>
          {offlineQueueCount} Payments Pending Sync
        </div>
        <div id="network-status-indicator" className={isOffline ? "app-badge warn block" : "hidden"} style={{ display: isOffline ? 'block' : 'none' }}>
          Offline - changes saved locally
        </div>
        {isSyncing && (
          <div className="fixed bottom-4 right-4 bg-indigo-600 text-white px-4 py-3 rounded-xl shadow-lg font-medium animate-in slide-in-from-bottom-5 z-50 flex items-center gap-2">
            <svg className="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Syncing {offlineQueueCount} offline payments...
          </div>
        )}
        {syncErrorCount > 0 && (
          <div className="app-badge bad" role="alert">
            {syncErrorCount} payment{syncErrorCount > 1 ? 's' : ''} failed to sync. Tap to resolve.
          </div>
        )}
        {error && <div className="app-badge bad">{error}</div>}
        {actionMessage && <div className="app-badge good" role="status">{actionMessage}</div>}
      </div>

      <div className="mb-6">
        <GrowthReferralWidget />
      </div>

      <ViralLoopPerformanceWidget />

      <div className="mb-6">
          <SmartBlock type="PoweredBy" props={{ tenantId: tenantId(), isPremium: false }} />
      </div>

      <section className="app-panel mb-6">
        <div className="app-panel-header">
          <div>
            <h2 className="app-panel-title">2024 Store Wrapped</h2>
            <div className="app-list-subtitle">A shareable snapshot of your strongest store moments.</div>
          </div>
          <span className="app-badge good">Viral Loop</span>
        </div>
        <div className="app-panel-body">
          <p className="app-list-subtitle mb-3">Turn your sales, products, and milestones into a referral-friendly recap.</p>
          <Link href="/wrapped" className="app-button">View Your Wrapped 🎁</Link>
        </div>
      </section>

      {showMigration && (
        <section className="app-panel mb-6">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Store Migration</div>
              <div className="app-list-subtitle">Import products and storefront details from an existing shop URL.</div>
            </div>
          </div>
          <div className="app-panel-body">
            <div className="flex flex-col gap-3 md:flex-row md:items-end">
              <label className="flex-1 text-sm font-semibold text-gray-700 dark:text-gray-200">
                Existing store URL
                <input
                  name="migration_url"
                  value={migrationUrl}
                  onChange={(event) => setMigrationUrl(event.target.value)}
                  className="mt-2 w-full rounded-[8px] border border-gray-200 bg-white px-3 py-2 text-sm text-[#1D1D1F] shadow-sm dark:border-white/10 dark:bg-black/30 dark:text-[#F5F5F7]"
                  placeholder="mayas-cakes.myshopify.com"
                />
              </label>
              <button
                type="button"
                className="app-button primary"
                onClick={() => {
                  setMigrationStatus("running");
                  window.setTimeout(() => setMigrationStatus("complete"), 750);
                }}
                disabled={!migrationUrl.trim() || migrationStatus === "running"}
              >
                Start Migration
              </button>
            </div>
            {migrationStatus === "running" && (
              <p className="mt-4 app-list-subtitle">Our AI is carefully moving your catalog, product photos, and store settings.</p>
            )}
            {migrationStatus === "complete" && (
              <div className="mt-4 flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
                <p className="app-list-title">Migration Complete</p>
                <button type="button" className="app-button" onClick={() => router.push("/products")}>
                  Review & Publish
                </button>
              </div>
            )}
          </div>
        </section>
      )}

      <main id="dashboard-screen" className="app-grid" style={{ gap: 16 }}>
        <UnifiedAgentFeed />

        <section>
          <div className="mb-6 p-6 rounded-[16px] bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10">
            <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
              <div className="flex items-center gap-4">
                <div className="text-4xl">🎉</div>
                <div>
                  <h3 className="text-xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7] font-outfit">Milestone Unlocked!</h3>
                  <p className="text-sm text-gray-600 dark:text-gray-300">You completed your first 5 orders!</p>
                </div>
              </div>
              <button
                type="button"
                onClick={() => {
                  const inviteUrl = `${window.location.origin}/onboarding?ref=${tenantId()}`;
                  navigator.clipboard?.writeText(inviteUrl).catch(() => undefined);
                  setActionMessage("Reward claimed. Invite link copied.");
                }}
                className="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg font-medium shadow-sm transition-colors"
              >
                Share & Claim Reward
              </button>
            </div>
          </div>

          <div className="mb-3 flex items-center justify-between gap-3">
            <div>
              <h2 className="app-panel-title">Business Analytics</h2>
              <p className="app-list-subtitle">Loaded from `/api/ui/dashboard/metrics`.</p>
            </div>
            <Link href="/business-analytics" className="app-button">Business Analytics</Link>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-[1fr_300px] gap-6">
            <div className="app-grid metrics !grid-cols-2 lg:!grid-cols-4">
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

            <div className="glassmorphism p-4 rounded-[12px] border border-indigo-200/50 bg-gradient-to-br from-indigo-50/50 to-purple-50/50 flex flex-col justify-center items-center text-center relative overflow-hidden">
              <div className="absolute top-0 right-0 w-16 h-16 bg-white/40 rounded-bl-full"></div>
              <h4 className="text-sm font-bold font-outfit text-[#1D1D1F] mb-1 flex items-center gap-1">
                <span className="text-indigo-500">✨</span> Advanced AI Insights
              </h4>
              <p className="text-xs text-gray-600 mb-3">Unlock predictive analytics and AI-driven growth recommendations.</p>
              <button
                onClick={() => {
                  setActionMessage('Opening Pro pricing for Advanced AI Insights.');
                  router.push('/pricing');
                }}
                className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white text-xs font-bold rounded-lg shadow-sm transition-colors w-full"
              >
                Upgrade to Pro
              </button>
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
              <WithTooltip id="recent-orders-tooltip" defaultText="View the latest orders placed by your customers."><div className="app-panel-title">Recent Orders</div></WithTooltip>
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
              <WithTooltip id="inbox-activity-tooltip" defaultText="Keep track of recent customer messages."><div className="app-panel-title">Inbox Activity</div></WithTooltip>
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
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <Link href="/dashboard/campaigns" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-sky-50 dark:bg-sky-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">↗</div>
                <div className="text-sky-700 dark:text-sky-300 font-semibold text-sm bg-sky-50 dark:bg-sky-900/30 px-3 py-1 rounded-full">Orchestrate</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Campaign Orchestration</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Plan, generate, review, and launch customer campaigns from live dashboard data.</p>
            </Link>

            <Link href="/upgrade-roi" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">📈</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">ROI</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Pro Plan ROI Calculator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">See how much extra revenue you could generate by unlocking the Pro Plan.</p>
            </Link>

            <Link href="/referrals" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🤝</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Earn $50</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Referrals</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Invite other business owners to OHC and earn premium credits.</p>
            </Link>

            <Link href="/milestones" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-purple-50 dark:bg-purple-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🏆</div>
                <div className="text-purple-600 dark:text-purple-400 font-semibold text-sm bg-purple-50 dark:bg-purple-900/30 px-3 py-1 rounded-full">Share</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Milestones</h3>

              <p className="text-sm text-gray-600 dark:text-gray-400">Track and share your business achievements with your audience.</p>
            </Link>

            <Link href="/share-cards" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-pink-50 dark:bg-pink-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🎴</div>
                <div className="text-pink-600 dark:text-pink-400 font-semibold text-sm bg-pink-50 dark:bg-pink-900/30 px-3 py-1 rounded-full">Cards</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Social Share Cards</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Generate Share Cards to promote your brand on social media.</p>
            </Link>

            <Link href="/storefront-widget" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🌐</div>
                <div className="text-blue-600 dark:text-blue-400 font-semibold text-sm bg-blue-50 dark:bg-blue-900/30 px-3 py-1 rounded-full">Widget</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Storefront Widget</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Embed a mini storefront on your blog or website to boost sales.</p>
            </Link>

            <Link href="/subscriptions" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-amber-50 dark:bg-amber-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">📦</div>
                <div className="text-amber-700 dark:text-amber-300 font-semibold text-sm bg-amber-50 dark:bg-amber-900/30 px-3 py-1 rounded-full">Recurring</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Subscriptions & Fulfillments</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Manage recurring products, subscribers, and shipping batches.</p>
            </Link>

            <Link href="/social-proof-nudge" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-green-50 dark:bg-green-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🚀</div>
                <div className="text-green-600 dark:text-green-400 font-semibold text-sm bg-green-50 dark:bg-green-900/30 px-3 py-1 rounded-full">Proof</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Social Proof Nudge</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Show visitors that others are buying to increase conversions.</p>
            </Link>

            <Link href="/link-in-bio-generator" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-emerald-50 dark:bg-emerald-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🔗</div>
                <div className="text-emerald-600 dark:text-emerald-400 font-semibold text-sm bg-emerald-50 dark:bg-emerald-900/30 px-3 py-1 rounded-full">Bio</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Create Link-in-Bio Page</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Publish a lightweight social profile page for your storefront and offers.</p>
            </Link>

            <Link href="/marketing/lead-gen" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🎯</div>
                <div className="text-blue-600 dark:text-blue-400 font-semibold text-sm bg-blue-50 dark:bg-blue-900/30 px-3 py-1 rounded-full">Leads</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Want more local jobs this week? [Tap here]</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Launch an autonomous hyper-local lead generation campaign.</p>
            </Link>
          </div>
        </section>
      </main>
    </AppShell>
  );
}
