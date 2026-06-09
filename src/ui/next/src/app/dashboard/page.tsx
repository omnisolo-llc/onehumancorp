"use client";
import { FloatingActionButton } from "./FAB";
import { MorningBriefingCard } from "./MorningBriefingCard";






import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { AppShell } from "../components/AppShell";
import { InteractiveWalkthrough, WalkthroughTarget } from "../../components/Walkthrough";
import { WithTooltip } from "../../components/TooltipRegistry";
import GrowthReferralWidget from "../components/GrowthReferralWidget";
import AiTimeSavingsWidget from "../components/AiTimeSavingsWidget";

import { SmartBlock } from "../builder/components";
import { UnifiedAgentFeed } from "./UnifiedAgentFeed";
import { ReviewFeedCard } from './ReviewFeedCard';

import { NeighborhoodPulseCard } from "./NeighborhoodPulseCard";
import { PromoterCard } from "./PromoterCard";
import { ViralLoopPerformanceWidget } from "./ViralLoopPerformanceWidget";
import { SuccessMilestoneAlert } from "./SuccessMilestoneAlert";
import AffiliateMarketingWidget from "./AffiliateMarketingWidget";

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

type ApprovalRequest = {
  id: string;
  tenant_id: string;
  department: string;
  description: string;
  status: string;
  action_risk: string;
  payload?: any;
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
  const [approvals, setApprovals] = useState<ApprovalRequest[]>([]);
  const [dashboardData, setDashboardData] = useState<any>({ pendingReviews: [] });
  const [loading, setLoading] = useState(true);
  const [ledgerBalance, setLedgerBalance] = useState<number | null>(null);
  const [ledgerCurrency, setLedgerCurrency] = useState<string>("USD");
  const [ledgerLoading, setLedgerLoading] = useState(true);

  useEffect(() => {
    async function fetchLedgerBalance() {
      try {
        const res = await fetch("/api/ledger/accounts");
        if (res.ok) {
          const data = await res.json();
          const mainAccount = data.accounts?.find((a: any) => a.name === "main");
          if (mainAccount) {
            setLedgerBalance(mainAccount.balance);
            setLedgerCurrency(mainAccount.currency);
          }
        }
      } catch (err) {
        console.error("Failed to fetch ledger balance", err);
      } finally {
        setLedgerLoading(false);
      }
    }
    fetchLedgerBalance();
  }, []);
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
  const [activeDepartments, setActiveDepartments] = useState<string[]>([]);

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
        const userId = localStorage.getItem("user_id") || "default";
        const [metricsRes, ordersRes, inboxRes, supplyRes, onboardingRes, approvalsRes] = await Promise.all([
          fetch(`/api/ui/dashboard/metrics?tenant_id=${tenant}`),
          fetch(`/api/ui/orders?tenant_id=${tenant}`),
          fetch(`/api/ui/inbox/messages?tenant_id=${tenant}`),
          fetch(`/api/ui/supply?tenant_id=${tenant}`),
          fetch(`/api/onboarding/state`, { headers: { 'X-Tenant-ID': tenant, 'X-User-ID': userId } }),
          fetch(`/api/agents/approvals?organization_id=${tenant}`),
        ]);

        if (!metricsRes.ok || !ordersRes.ok || !inboxRes.ok || !supplyRes.ok) {
          throw new Error("One or more database-backed UI endpoints failed");
        }

        const [metricsData, ordersData, inboxData, supplyData, onboardingData, approvalsData] = await Promise.all([
          metricsRes.json(),
          ordersRes.json(),
          inboxRes.json(),
          supplyRes.json(),
          onboardingRes.ok ? onboardingRes.json() : Promise.resolve(null),
          approvalsRes.ok ? approvalsRes.json() : Promise.resolve({ tasks: [] }),
        ]);

        if (onboardingData?.wizardState?.aiAgents) {
          setActiveDepartments(onboardingData.wizardState.aiAgents);
        } else {
          setActiveDepartments([]);
        }

        setMetrics({ ...emptyMetrics, ...metricsData });
        setOrders(Array.isArray(ordersData) ? ordersData : []);
        setMessages(Array.isArray(inboxData) ? inboxData : []);
        setApprovals(Array.isArray(approvalsData?.tasks) ? approvalsData.tasks : []);
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

  async function handleApproveDraft(approvalId: string) {
    try {
      const token = localStorage.getItem("token") || "";
      const res = await fetch(`/api/agents/approvals/${approvalId}`, {
        method: "POST",
        headers: { "Content-Type": "application/json", "Authorization": `Bearer ${token}` },
        body: JSON.stringify({ approved: true })
      });
      if (res.ok) {
        setApprovals(prev => prev.filter(a => a.id !== approvalId));
      }
    } catch (e) {
      console.error(e);
    }
  }

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

      <AiTimeSavingsWidget />
      <NeighborhoodPulseCard tenant={tenantId()} />
      <FloatingActionButton />

      <MorningBriefingCard tenant={tenantId()} />

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
          Offline Mode
        </div>
        {isSyncing && <div className="app-badge good animate-pulse">Syncing Payments...</div>}
        {syncErrorCount > 0 && <div className="app-badge bad">{syncErrorCount} Sync Failures</div>}
      </div>

      {showMigration && (
        <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-orange-200">
          <h3 className="text-xl font-bold font-outfit mb-4">Migrate to OHC</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <input
              type="text"
              placeholder="Store URL (e.g. shopify-store.com)"
              value={migrationUrl}
              onChange={(e) => setMigrationUrl(e.target.value)}
              className="px-4 py-2 rounded-lg border border-gray-200"
            />
            <button
              onClick={async () => {
                setMigrationStatus("running");
                // Mocking the agentic migration call
                setTimeout(() => setMigrationStatus("complete"), 3000);
              }}
              className="app-btn-primary"
              disabled={migrationStatus === "running"}
            >
              {migrationStatus === "running" ? "Agent analyzing store..." : "Start Migration"}
            </button>
          </div>
          {migrationStatus === "complete" && (
            <p className="mt-4 text-green-600 font-semibold">✨ Migration plan ready! Check your inbox to approve the import.</p>
          )}
        </div>
      )}

      <div className="app-grid">
        <section className="app-panel span-2">
          <SuccessMilestoneAlert tenant={tenantId()} />
          <div className="app-panel-header">
            <div className="app-panel-title">Business Pulse</div>
          </div>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 p-6">
            <WalkthroughTarget id="sales-card-target">
              <div className="app-card overflow-hidden relative">
                <div className="absolute top-0 right-0 p-4 opacity-10 text-4xl">💰</div>
                <div className="app-metric-label">Total Revenue</div>
                <div className="app-metric-value">{money(metrics.total_sales)}</div>
                <div className="app-metric-note">Across all time</div>
              </div>
            </WalkthroughTarget>
            <div className="app-card overflow-hidden relative">
              <div className="absolute top-0 right-0 p-4 opacity-10 text-4xl">👥</div>
              <div className="app-metric-label">Total Customers</div>
              <div className="app-metric-value">{metrics.active_customers}</div>
              <div className="app-metric-note">Unique user records</div>
            </div>
            <div className="app-card overflow-hidden relative">
              <div className="absolute top-0 right-0 p-4 opacity-10 text-4xl">📧</div>
              <div className="app-metric-label">Campaigns Sent</div>
              <div className="app-metric-value">{metrics.total_campaigns_sent || 0}</div>
              <div className="app-metric-note">Marketing reach</div>
            </div>
            <div className="app-card overflow-hidden relative">
              <div className="absolute top-0 right-0 p-4 opacity-10 text-4xl">🏦</div>
              <div className="app-metric-label">Available Balance</div>
              <div className="app-metric-value">{ledgerLoading ? "..." : (ledgerBalance !== null ? `${ledgerCurrency} ${(ledgerBalance/100).toFixed(2)}` : "N/A")}</div>
              <div className="app-metric-note">Verified ledger</div>
            </div>
          </div>
        </section>

        <section className="app-panel">
          <ReviewFeedCard tenantId={tenantId()} />
        </section>

        <section className="app-panel">
          <ViralLoopPerformanceWidget />
        </section>

        <section className="app-panel">
           <AffiliateMarketingWidget tenantId={tenantId()} />
        </section>

        <section className="app-panel">
          <PromoterCard tenant={tenantId()} />
        </section>

        <div className="span-2 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <WalkthroughTarget id="operations-map-target">
            <div className="app-panel h-full">
              <div className="app-panel-header">
                <div className="app-panel-title">Operations Map</div>
                <Link href="/orders" className="app-button">Orders</Link>
              </div>
              <div className="grid grid-cols-2 gap-3 p-6">
                <div className="app-card">
                  <div className="app-metric-label">Open Orders</div>
                  <div className="app-metric-value text-orange-500">{metrics.pending_orders}</div>
                  <div className="app-metric-note">Needs fulfillment</div>
                </div>
                <div className="app-card">
                  <div className="app-metric-label">Low Stock</div>
                  <div className="app-metric-value text-red-500">{lowStockCount}</div>
                  <div className="app-metric-note">SKUs below threshold</div>
                </div>
                <div className="app-card">
                  <div className="app-metric-label">Inbound DMs</div>
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
                            {approvals.filter(a => a.payload?.feature_type === 'ambassador_reply').map(approval => (
                <div key={approval.id} className="app-list-item flex flex-col items-start gap-3">
                  <div className="w-full">
                    <div className="app-list-title">Action Required: Approve Reply</div>
                    <div className="app-list-subtitle font-semibold text-gray-900 mt-1">1 New Message from {approval.payload?.source || "Instagram DM"}</div>
                    <div className="app-list-subtitle mt-2 bg-gray-50 p-2 rounded border border-gray-100 text-xs italic">"{approval.payload?.original_message || approval.payload?.message || "Customer message"}"</div>
                    <div className="app-list-subtitle mt-2 p-2 rounded bg-blue-50 border border-blue-100 text-blue-900 text-sm">
                      <span className="font-semibold text-blue-800 text-xs uppercase mb-1 block">AI Draft</span>
                      {approval.payload?.generated_response || approval.payload?.draft_reply || "Ready to send."}
                    </div>
                  </div>
                  <div className="flex gap-2 w-full mt-1">
                    <button type="button" className="app-btn-primary flex-1 py-2" onClick={() => handleApproveDraft(approval.id)}>✨ 1-Tap Approve</button>
                    <Link href="/inbox" className="app-button flex-1 py-2 text-center bg-gray-100">Edit</Link>
                  </div>
                </div>
              ))}
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
                    <div className="app-list-title">Inventory replenishment</div>
                    <div className="app-list-subtitle">{lowStockCount} raw materials are running low.</div>
                  </div>
                  <span className="app-badge bad">Stock</span>
                </div>
              )}
              {!loading && metrics.pending_orders === 0 && lowStockCount === 0 && messages.length === 0 && approvals.filter(a => a.payload?.feature_type === "ambassador_reply").length === 0 && (
                <div className="app-empty">
                  ✨ No urgent actions. You're all caught up!
                </div>
              )}
            </div>
          </div>

          <div className="app-panel">
            <div className="app-panel-header">
              <div className="app-panel-title">Recent Activity</div>
              <Link href="/inbox" className="app-button">Inbox</Link>
            </div>
            <div className="app-list">
              {messages.length === 0 ? (
                <div className="app-empty">No recent messages recorded.</div>
              ) : messages.map((msg) => (
                <div key={msg.id} className="app-list-item">
                  <div className="min-w-0">
                    <div className="app-list-title">{msg.source || "Customer"}</div>
                    <div className="app-list-subtitle truncate">{msg.content || "No content"}</div>
                  </div>
                  <span className="app-badge neutral">{msg.created_at ? new Date(msg.created_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) : "Recently"}</span>
                </div>
              ))}
            </div>
          </div>
        </div>

        <section className="app-panel span-2">
           <ReviewFeedCard tenantId={tenantId()} />
        </section>

        <section className="app-panel span-2">
          <div className="app-panel-header">
            <div className="app-panel-title">AI Swarm Workforce</div>
            <Link href="/agents" className="app-button">Manage Agents</Link>
          </div>
          <UnifiedAgentFeed tenantId={tenantId()} />
        </section>
      </div>

      <InteractiveWalkthrough
        steps={walkthroughSteps}
        isOpen={isWalkthroughOpen}
        onClose={() => setIsWalkthroughOpen(false)}
      />

      {actionMessage && (
        <div className="fixed bottom-8 left-1/2 -translate-x-1/2 app-badge good shadow-lg px-6 py-3 text-lg animate-bounce">
          {actionMessage}
        </div>
      )}
    </AppShell>
  );
}
