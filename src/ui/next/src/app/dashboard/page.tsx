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
import { TriageFeed } from "./TriageFeed";
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
  const [approvals, setApprovals] = useState<any[]>([]);
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

  const handleApproveDraft = async (approvalId: string) => {
    try {
      const token = localStorage.getItem("token") || "";
      const res = await fetch(`/api/agents/approvals/${approvalId}`, {
        method: "POST",
        headers: { "Content-Type": "application/json", "Authorization": `Bearer ${token}` },
        body: JSON.stringify({ approved: true })
      });
      if (res.ok) {
        setDashboardData((prev: any) => ({ ...prev, pendingReviews: prev.pendingReviews.filter((a: any) => a.id !== approvalId) }));
      }
    } catch (e) {
      console.error(e);
    }
  };

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
        console.error("Background sync failed", e);
        setSyncErrorCount((prev) => prev + 1);
      } finally {
        setIsSyncing(false);
      }
    };

    window.addEventListener("online", handleSync);
    window.addEventListener("online", updateOfflineStatus);
    window.addEventListener("offline", updateOfflineStatus);

    updateOfflineStatus();
    if (navigator.onLine) {
       handleSync();
    }

    return () => {
      window.removeEventListener("online", handleSync);
      window.removeEventListener("online", updateOfflineStatus);
      window.removeEventListener("offline", updateOfflineStatus);
    };
  }, []);

  const [hasStartedInteractiveTrial, setHasStartedInteractiveTrial] = useState(false);

  useEffect(() => {
    async function load() {
      setLoading(true);
      try {
        const query = `?tenant_id=${encodeURIComponent(tenantId())}`;

        const settingsRes = await fetch(`/api/ui/settings${query}`);
        if (settingsRes.ok) {
           const settingsData = await settingsRes.json();
           const businessType = settingsData.business_type || '';
           if (businessType.includes('interactive_trial_started')) {
               setHasStartedInteractiveTrial(true);
           }
        }


        const checkMigration = async () => {
          try {
            const settingsRes = await fetch(`/api/ui/settings${query}`);
            if (settingsRes.ok) {
              const settingsData = await settingsRes.json();
              if (settingsData.business_type === 'legacy_platform' && settingsData.migration_status !== 'complete') {
                setShowMigration(true);
              }
            }
          } catch (e) {
             // ignore
          }
        };
        checkMigration();
        const depsRes = await fetch(`/api/settings/departments?tenant_id=${tenantId()}`);
        if (depsRes.ok) {
           const depsData = await depsRes.json();
           setActiveDepartments(depsData.departments || []);
        }

        const [metRes, ordRes, msgRes, supRes] = await Promise.all([
          fetch(`/api/ui/dashboard/metrics${query}`),
          fetch(`/api/ui/orders${query}`),
          fetch(`/api/ui/inbox/messages${query}`),
          fetch(`/api/ui/supply${query}`)
        ]);

        if (metRes.ok) setMetrics(await metRes.json());
        if (ordRes.ok) setOrders(await ordRes.json());
        if (msgRes.ok) setMessages(await msgRes.json());
        if (supRes.ok) setSupply(await supRes.json());
      } catch (err: any) {
        setError(err.message || "Failed to load dashboard data.");
      } finally {
        setLoading(false);
      }
    }
    load();
  }, []);

  const metricItems = [
    { label: "Active Customers", value: String(metrics.active_customers), tone: metrics.active_customers > 0 ? "good" : "neutral" },
    { label: "Total Sales", value: money(metrics.total_sales), tone: metrics.total_sales > 0 ? "good" : "neutral" },
  ];

  if (metrics.pending_orders > 0) {
    metricItems.push({ label: "Pending Orders", value: String(metrics.pending_orders), tone: "warn" });
  }

  return (
    <AppShell
      title={`Good morning, ${userName}.`}
      subtitle="Here is what is happening across your work today."
      statusItems={metricItems}
    >
      <main className="w-full max-w-[1400px] mx-auto space-y-6">
        <FloatingActionButton />

        <InteractiveWalkthrough
          isOpen={isWalkthroughOpen}
          onClose={() => setIsWalkthroughOpen(false)}
          tenantId={tenantId()}
        />

        {error && (
          <div className="p-4 mb-6 rounded-md bg-red-50 text-red-700 text-sm">
            {error}
          </div>
        )}

        {isOffline && (
          <div className="p-4 mb-6 rounded-[16px] bg-yellow-50 dark:bg-yellow-900/30 border border-yellow-200 dark:border-yellow-700/50 flex flex-col sm:flex-row sm:items-center justify-between gap-4">
            <div className="flex items-center gap-3">
              <span className="text-xl">⚠️</span>
              <div>
                <h3 className="font-semibold text-yellow-800 dark:text-yellow-200">You are operating offline.</h3>
                <p className="text-sm text-yellow-700 dark:text-yellow-300">
                  {offlineQueueCount > 0
                    ? `You have ${offlineQueueCount} action(s) saved locally.`
                    : 'OHC is running in local-first mode.'}
                  {" "}They will sync automatically when your connection is restored.
                </p>
              </div>
            </div>
            {offlineQueueCount > 0 && (
              <button
                onClick={() => router.push('/sync')}
                className="px-4 py-2 bg-yellow-100 dark:bg-yellow-800/50 hover:bg-yellow-200 dark:hover:bg-yellow-800 text-yellow-800 dark:text-yellow-200 text-sm font-medium rounded-lg transition-colors"
              >
                View Sync Queue
              </button>
            )}
          </div>
        )}

        {!isOffline && isSyncing && (
          <div className="p-4 mb-6 rounded-[16px] bg-blue-50 dark:bg-blue-900/30 border border-blue-200 dark:border-blue-700/50 flex items-center gap-3">
            <span className="text-xl animate-spin">🔄</span>
            <p className="text-sm text-blue-700 dark:text-blue-300">
              Syncing offline changes to the cloud...
            </p>
          </div>
        )}

        {!isOffline && syncErrorCount > 0 && (
          <div className="p-4 mb-6 rounded-[16px] bg-red-50 dark:bg-red-900/30 border border-red-200 dark:border-red-700/50 flex items-center gap-3">
             <span className="text-xl">❌</span>
             <p className="text-sm text-red-700 dark:text-red-300">
                {syncErrorCount} offline action(s) failed to sync. Please check the sync queue.
             </p>
             <button onClick={() => router.push('/sync')} className="ml-auto px-4 py-2 bg-red-100 dark:bg-red-800/50 hover:bg-red-200 dark:hover:bg-red-800 text-red-800 dark:text-red-200 text-sm font-medium rounded-lg transition-colors">
                Resolve
             </button>
          </div>
        )}

        {actionMessage && (
           <div className="p-4 mb-6 rounded-[16px] bg-green-50 dark:bg-green-900/30 border border-green-200 dark:border-green-700/50 text-green-800 dark:text-green-200">
             {actionMessage}
           </div>
        )}

        {showMigration && (
           <div className="p-6 mb-6 rounded-[16px] bg-blue-50 dark:bg-blue-900/30 border border-blue-200 dark:border-blue-700/50">
             <h3 className="text-lg font-bold text-blue-900 dark:text-blue-100 mb-2">Platform Migration Available</h3>
             <p className="text-sm text-blue-800 dark:text-blue-200 mb-4">
               We detected you are running on a legacy commerce platform. The Autonomous Ops Agent can migrate your catalog, customers, and order history into OHC automatically.
             </p>
             <div className="flex flex-col sm:flex-row gap-4 items-center">
                <input
                  type="text"
                  placeholder="https://your-legacy-store.com"
                  className="flex-1 w-full p-2 rounded-md border border-blue-200 dark:border-blue-700 dark:bg-blue-900/50 dark:text-white"
                  value={migrationUrl}
                  onChange={(e) => setMigrationUrl(e.target.value)}
                />
                <button
                  disabled={migrationStatus === 'running'}
                  onClick={async () => {
                     setMigrationStatus('running');
                     try {
                        await fetch(`/api/agents/autonomous-ops?tenant_id=${tenantId()}`, {
                           method: 'POST',
                           headers: { 'Content-Type': 'application/json' },
                           body: JSON.stringify({ action: 'trigger_migration', url: migrationUrl })
                        });
                        setMigrationStatus('complete');
                        setActionMessage("Migration started! The Autonomous Ops agent is moving your data in the background. We will notify you when it's ready.");
                        setShowMigration(false);
                     } catch(e) {
                        setMigrationStatus('idle');
                        setError("Migration failed to start.");
                     }
                  }}
                  className="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg disabled:opacity-50 whitespace-nowrap"
                >
                  {migrationStatus === 'running' ? 'Migrating...' : 'Start Migration'}
                </button>
             </div>
           </div>
        )}

        <WalkthroughTarget id="metrics-overview">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
            <SmartBlock
              type="metric"
              title="Ledger Balance"
              value={ledgerLoading ? "..." : (ledgerBalance !== null ? `${ledgerBalance.toFixed(2)} ${ledgerCurrency}` : "0.00 USD")}
              trend={ledgerBalance !== null && ledgerBalance > 0 ? "+ Active" : "Neutral"}
              status={ledgerBalance !== null && ledgerBalance > 0 ? "good" : "neutral"}
            />
            <SmartBlock type="metric" title="Active Customers" value={String(metrics.active_customers)} />
            <SmartBlock type="metric" title="Pending Orders" value={String(metrics.pending_orders)} status={metrics.pending_orders > 0 ? "warn" : "neutral"} />
            <SmartBlock type="metric" title="Total Sales" value={money(metrics.total_sales)} trend={metrics.total_sales > 0 ? "+ Up" : undefined} status={metrics.total_sales > 0 ? "good" : "neutral"} />
          </div>
        </WalkthroughTarget>

        <WalkthroughTarget id="morning-briefing">
          <MorningBriefingCard />
        </WalkthroughTarget>

        {activeDepartments.includes('marketing') && (
           <WalkthroughTarget id="marketing-widgets">
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
                 <ViralLoopPerformanceWidget />
                 <GrowthReferralWidget />
              </div>
              <div className="mb-6">
                <AffiliateMarketingWidget />
              </div>
           </WalkthroughTarget>
        )}

        {activeDepartments.includes('customer_service') && (
          <WalkthroughTarget id="customer-insights">
            <NeighborhoodPulseCard />
          </WalkthroughTarget>
        )}

        {dashboardData.pendingReviews && dashboardData.pendingReviews.map((approval: any) => (
             <ReviewFeedCard
               key={approval.id}
               tenantId={tenantId()}
               approval={approval}
               onDecision={async (id: string, approved: boolean) => {
                 try {
                   const token = localStorage.getItem("token") || "";
                   const res = await fetch(`/api/agents/approvals/${id}`, {
                     method: "POST",
                     headers: { "Content-Type": "application/json", "Authorization": `Bearer ${token}` },
                     body: JSON.stringify({ approved })
                   });
                   if (res.ok) {
                     setDashboardData((prev: any) => ({ ...prev, pendingReviews: prev.pendingReviews.filter((a: any) => a.id !== id) }));
                   }
                 } catch (e) { console.error(e); }
               }}
             />
        ))}

        <TriageFeed />
        <UnifiedAgentFeed />

        <section>
          <div className="mb-6 p-6 rounded-[16px] bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10">
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
                Claim Reward & Share
              </button>
            </div>
          </div>
          {metrics.total_sales > 1000 && (
             <SuccessMilestoneAlert
                milestoneType="revenue"
                milestoneValue="1k"
             />
          )}
        </section>

        <section className="mb-6">
          <AiTimeSavingsWidget />
        </section>

        <section className="mb-6">
          <PromoterCard />
        </section>

        <section>
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Recent Work</h2>
            <div className="flex items-center gap-3">
              <button
                onClick={() => setIsWalkthroughOpen(true)}
                className="text-sm font-medium text-blue-600 hover:text-blue-700 dark:text-blue-400"
              >
                Show Tour
              </button>
              <Link href="/orders" className="text-sm font-medium text-indigo-600 dark:text-indigo-400 hover:text-indigo-700">View All</Link>
            </div>
          </div>
          <div className="app-grid">
            {orders.length === 0 && !loading ? (
              <div className="app-panel col-span-full">
                <div className="app-empty">No recent orders found.</div>
              </div>
            ) : (
              orders.slice(0, 3).map((order) => (
                <div key={order.id} className="app-panel p-4">
                  <div className="flex justify-between items-start mb-2">
                    <div className="font-semibold text-gray-900 dark:text-white truncate pr-2">{order.customer_name || "Guest"}</div>
                    <span className={`app-badge ${statusTone(order.status)}`}>{order.status || "Unknown"}</span>
                  </div>
                  <div className="text-sm text-gray-500 mb-3">{new Date(order.created_at || Date.now()).toLocaleDateString()}</div>
                  <div className="font-bold text-lg text-gray-900 dark:text-white">{money(order.total_amount)}</div>
                </div>
              ))
            )}
          </div>
        </section>


        <section>
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Growth & Tools</h2>
          </div>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">

            <Link href="/store-wrap" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🎧</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Share</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Store Wrap 2024</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Generate a beautiful, shareable recap of your business year.</p>
            </Link>

            <Link href="/storefront-builder" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🛍️</div>
                <div className="text-blue-600 dark:text-blue-400 font-semibold text-sm bg-blue-50 dark:bg-blue-900/30 px-3 py-1 rounded-full">Sales</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Storefront Builder</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Drag and drop to build your modern e-commerce storefront.</p>
            </Link>

            <Link href="/bio" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-emerald-50 dark:bg-emerald-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🔗</div>
                <div className="text-emerald-600 dark:text-emerald-400 font-semibold text-sm bg-emerald-50 dark:bg-emerald-900/30 px-3 py-1 rounded-full">Bio</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Create Link-in-Bio Page</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Publish a lightweight social profile page for your storefront and offers.</p>
            </Link>

            <Link href="/giveaway" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-pink-50 dark:bg-pink-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🎁</div>
                <div className="text-pink-600 dark:text-pink-400 font-semibold text-sm bg-pink-50 dark:bg-pink-900/30 px-3 py-1 rounded-full">Viral</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Viral Giveaway Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Launch a viral sweepstakes to capture emails and drive social shares.</p>
            </Link>

            <Link href="/win-back" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-purple-50 dark:bg-purple-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">💌</div>
                <div className="text-purple-600 dark:text-purple-400 font-semibold text-sm bg-purple-50 dark:bg-purple-900/30 px-3 py-1 rounded-full">Retain</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Customer Win-back</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Re-engage inactive customers with AI-generated email campaigns.</p>
            </Link>

            <Link href="/review-campaigns" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-yellow-50 dark:bg-yellow-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">⭐️</div>
                <div className="text-yellow-600 dark:text-yellow-400 font-semibold text-sm bg-yellow-50 dark:bg-yellow-900/30 px-3 py-1 rounded-full">Reviews</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Automated Reviews</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Generate highly-converting, personalized review request emails.</p>
            </Link>

            <Link href="/seasonal-promo" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-teal-50 dark:bg-teal-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">✨</div>
                <div className="text-teal-600 dark:text-teal-400 font-semibold text-sm bg-teal-50 dark:bg-teal-900/30 px-3 py-1 rounded-full">Promo</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Seasonal Promo Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Create AI campaigns and promo codes for special occasions instantly.</p>
            </Link>

            <Link href="/cart-recovery" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-orange-50 dark:bg-orange-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🛒</div>
                <div className="text-orange-600 dark:text-orange-400 font-semibold text-sm bg-orange-50 dark:bg-orange-900/30 px-3 py-1 rounded-full">Recover</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Cart Recovery</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Recover abandoned carts with personalized AI follow-ups.</p>
            </Link>

            <Link href="/flash-sale-generator" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-red-50 dark:bg-red-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">⚡</div>
                <div className="text-red-600 dark:text-red-400 font-semibold text-sm bg-red-50 dark:bg-red-900/30 px-3 py-1 rounded-full">Urgency</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Flash Sale Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Create high-converting flash sale countdown widgets.</p>
            </Link>


            <Link href="/triage" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">📥</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Inbox</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Triage Inbox</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Review AI-prioritized tasks, unread messages, and system alerts.</p>
            </Link>

            <Link href="/marketing/lead-gen" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🎯</div>
                <div className="text-blue-600 dark:text-blue-400 font-semibold text-sm bg-blue-50 dark:bg-blue-900/30 px-3 py-1 rounded-full">Leads</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Want more local jobs this week? [Tap here]</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Launch an autonomous hyper-local lead generation campaign.</p>
            </Link>

            <Link href="/trial-extension" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-emerald-50 dark:bg-emerald-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🎁</div>
                <div className="text-emerald-600 dark:text-emerald-400 font-semibold text-sm bg-emerald-50 dark:bg-emerald-900/30 px-3 py-1 rounded-full">Extension</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Interactive Trial Extension</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Share your setup on X to instantly unlock 7 extra days of Pro.</p>
            </Link>

            <Link href="/field-ops/jobs" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">📍</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Operations</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Field Ops Route</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Offline-first mobile route management for field service workers.</p>
            </Link>

            <Link href="/settings" className="block glassmorphism p-6 rounded-[16px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-gray-50 dark:bg-gray-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">⚙️</div>
                <div className="text-gray-600 dark:text-gray-400 font-semibold text-sm bg-gray-50 dark:bg-gray-900/30 px-3 py-1 rounded-full">Config</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Settings</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Manage your account and preferences.</p>
            </Link>
          </div>
        </section>
      </main>

    </AppShell>
  );
}
