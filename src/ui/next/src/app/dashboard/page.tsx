"use client";
import { AIPaywallWidget } from "../components/AIPaywallWidget";
import { FloatingActionButton } from "./FAB";
import { VoiceAssistantFAB } from "./VoiceAssistantFAB";
import { MorningBriefingCard } from "./MorningBriefingCard";
import { AIFeaturePaywallWidget } from "./AIFeaturePaywallWidget";




import { CFOAgentCard } from "./CFOAgentCard";


import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { AppShell } from "../components/AppShell";
import { InteractiveWalkthrough, WalkthroughTarget } from "../../components/Walkthrough";
import { WithTooltip } from "../../components/TooltipRegistry";
import { DashboardViralInviteWidget } from "./DashboardViralInviteWidget";
import { AIUsageLimitWidget } from "./AIUsageLimitWidget";
import AiTimeSavingsWidget from "../components/AiTimeSavingsWidget";

import { SmartBlock } from "../builder/components";
import { UnifiedAgentFeed } from "./UnifiedAgentFeed";
import { ReviewFeedCard } from './ReviewFeedCard';

import { NeighborhoodPulseCard } from "./NeighborhoodPulseCard";
import { PromoterCard } from "./PromoterCard";
import { GrowBusinessCard } from "./GrowBusinessCard";
import { ViralLoopPerformanceWidget } from "./ViralLoopPerformanceWidget";
import { SuccessMilestoneWidget } from "./SuccessMilestoneWidget";
import AffiliateMarketingWidget from "./AffiliateMarketingWidget";
import { CartRecoveryWidget } from "./CartRecoveryWidget";
import { WrappedWidget } from "./WrappedWidget";
import ReferralMilestonesWidget from "../components/ReferralMilestonesWidget";

type DashboardMetrics = {
  active_customers: number;
  pending_orders: number;
  total_sales: number;
  total_campaigns_sent?: number;
  auto_replied?: number;
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
  auto_replied: 0,
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
  if (["paid", "completed", "shipped", "delivered", "auto_replied"].includes(normalized)) return "good";
  if (["pending", "unfulfilled", "open"].includes(normalized)) return "warn";
  if (["failed", "cancelled", "canceled"].includes(normalized)) return "bad";
  return "neutral";
}

function formatStatus(status?: string) {
  const normalized = (status || "").toLowerCase();
  if (normalized === "auto_replied") return "✨ AI Handled";
  return status || "Open";
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

  const [error, setError] = useState("");
  const [isOffline, setIsOffline] = useState(false);
  const [offlineQueueCount, setOfflineQueueCount] = useState(0);
  const [isWalkthroughOpen, setIsWalkthroughOpen] = useState(false);
  const [walkthroughSteps, setWalkthroughSteps] = useState<any[]>([]);
  const [pendingApprovals, setPendingApprovals] = useState<any[]>([]);
  const [activities, setActivities] = useState<any[]>([]);
  const [initialTriage, setInitialTriage] = useState<any[]>([]);
  const [userName, setUserName] = useState("Human");
  const [remainingActions, setRemainingActions] = useState<number | null>(null);
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
    fetch("/api/walkthrough/dashboard")
      .then((res) => (res.ok ? res.json() : []))
      .then((data) => {
        if (Array.isArray(data)) {
          setWalkthroughSteps(data);
        }
      })
      .catch((err) => console.error("Walkthrough fetch failed:", err));

    try {
      const storedName = localStorage.getItem("user_name");
      if (storedName) {
        setUserName(storedName);
      }
    } catch {
      // ignore
    }

    const updateOfflineStatus = async () => {
      setIsOffline(!navigator.onLine);
      try {
        const { getActions } = await import("../utils/offlineQueue");
        const actions = await getActions();
        setOfflineQueueCount(actions.length);
      } catch {
        setOfflineQueueCount(0);
      }
    };

    const handleSync = async () => {
      if (!navigator.onLine) return;
      try {
        const { getActions, removeAction } = await import("../utils/offlineQueue");
        const queue = await getActions();
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

          // Remove exactly the items we just synced
          for (const item of queue) {
             await removeAction(item.id);
          }

          const currentQueue = await getActions();
          setOfflineQueueCount(currentQueue.length);
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

        const unifiedPromise = fetch(`/api/ui/dashboard/unified-feed?tenant_id=${tenant}&mobile_optimized=${window.innerWidth < 768}`)
          .then(res => {
            if (!res.ok) throw new Error("Unified UI feed endpoint failed");
            return res.json();
          });

        const onboardingPromise = fetch(`/api/onboarding/state`, { headers: { 'X-Tenant-ID': tenant, 'X-User-ID': userId } })
          .then(res => res.ok ? res.json() : null)
          .catch(() => null);

        const ledgerPromise = fetch("/api/ledger/accounts")
          .then(res => res.ok ? res.json() : null)
          .catch(() => null);

        const usagePromise = fetch(`/api/user/usage?tenant_id=${tenant}`)
          .then(res => res.ok ? res.json() : { remainingActions: 9 })
          .catch(() => ({ remainingActions: 9 }));

        const [unifiedData, onboardingData, ledgerData, usageData] = await Promise.all([
          unifiedPromise,
          onboardingPromise,
          ledgerPromise,
          usagePromise,
        ]);

        if (usageData && usageData.remainingActions !== undefined) {
           setRemainingActions(usageData.remainingActions);
        }

        if (ledgerData && ledgerData.accounts) {
          const mainAccount = ledgerData.accounts.find((a: any) => a.name === "main");
          if (mainAccount) {
            setLedgerBalance(mainAccount.balance);
            setLedgerCurrency(mainAccount.currency);
          }
        }
        setLedgerLoading(false);

        const approvalsData = unifiedData?.pending_approvals || [];
        const agentFeedData = { items: unifiedData?.agent_feed || [] };

        setDashboardData((prev: any) => ({ ...prev, initialAgentFeed: agentFeedData }));

        if (approvalsData && Array.isArray(approvalsData) && approvalsData.length > 0 && !agentFeedData.items?.length) {
            setPendingApprovals(approvalsData.filter((i: any) => i.status !== "APPROVED" && i.status !== "REJECTED" && i.status !== "PAUSED"));
            setActivities(approvalsData.filter((i: any) => i.status === "APPROVED" || i.status === "REJECTED" || i.status === "PAUSED"));
        } else if (agentFeedData && agentFeedData.items) {
            setPendingApprovals(agentFeedData.items.filter((i: any) => i.lifecycle_state !== "APPROVED" && i.lifecycle_state !== "DISMISSED" && i.lifecycle_state !== "PAUSED"));
            setActivities(agentFeedData.items.filter((i: any) => i.lifecycle_state === "APPROVED" || i.lifecycle_state === "DISMISSED" || i.lifecycle_state === "PAUSED").map((a: any) => ({
                id: a.id,
                event_type: a.lifecycle_state,
                department: a.event_source,
                payload: typeof a.context_payload === 'object' ? JSON.stringify({ original_payload: a.context_payload }) : a.context_payload,
                created_at: a.created_at
            })));
        }

        const metricsData = unifiedData.metrics || {};
        const ordersData = unifiedData.orders || [];
        const inboxData = unifiedData.inbox || [];
        const supplyData = unifiedData.supply || {};

        if (onboardingData?.wizardState?.aiAgents) {
          setActiveDepartments(onboardingData.wizardState.aiAgents);
        } else {
          setActiveDepartments([]);
        }

        setMetrics({ ...emptyMetrics, ...metricsData });
        setOrders(Array.isArray(ordersData) ? ordersData : []);
        setMessages(Array.isArray(inboxData) ? inboxData : []);
        setSupply({
          vendors: Array.isArray(supplyData?.vendors) ? supplyData.vendors : [],
          raw_materials: Array.isArray(supplyData?.raw_materials) ? supplyData.raw_materials : [],
          bom_items: Array.isArray(supplyData?.bom_items) ? supplyData.bom_items : [],
        });
        setApprovals(Array.isArray(approvalsData?.approvals) ? approvalsData.approvals : (Array.isArray(approvalsData) ? approvalsData : []));
        if (unifiedData.triage) {
          setInitialTriage(unifiedData.triage);
        }
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


  return (
    <>
    <AIPaywallWidget remainingActions={remainingActions} />
    <AppShell
      title="Dashboard"
      subtitle="Network-style command center for your store operations."
      statusItems={statusItems}
      actions={[
        { label: "Sell In Person", href: "/pos/terminal" },
        { label: "Campaigns", href: "/dashboard/campaigns", icon: "campaigns" },
        { label: "New Product", href: "/products/new", primary: true },
      ]}
    >
      <div className="mb-6 p-6 glassmorphism border border-white/40 dark:border-white/10">
        <WalkthroughTarget id="dashboard-title">
          <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Welcome back, {userName}.</h2>
        </WalkthroughTarget>
        <p className="text-gray-600 dark:text-gray-400">Your agents are working on your behalf.</p>
      </div>

      <div className="mb-6 w-full overflow-hidden">
        {/* Action Feed: prioritized on mobile (top), rendered below metrics on desktop. */}
        <UnifiedAgentFeed initialData={{ items: dashboardData?.initialAgentFeed?.items, proposals: pendingApprovals, activity: activities, orders, inbox: messages, triage: initialTriage, priority_tasks: dashboardData?.priority_tasks || [], pendingReviews: dashboardData?.pendingReviews || [] }} />
      </div>

      <div className="hidden md:block">
      <AIUsageLimitWidget />

      <AiTimeSavingsWidget />
      <NeighborhoodPulseCard tenant={tenantId()} />
      <FloatingActionButton />
      <VoiceAssistantFAB />

      <MorningBriefingCard tenant={tenantId()} />
      <CFOAgentCard />
      <AIFeaturePaywallWidget />

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
          id="dashboard-walkthrough-btn"
          className="app-button min-h-[44px]"
        >
          Start Tour
        </button>
        <button type="button" onClick={() => router.push("/onboarding")} className="app-button min-h-[44px]">
          Launch Site
        </button>
        <button type="button" onClick={() => setShowMigration((open) => !open)} className="app-button min-h-[44px]">
          Migrate Existing Store
        </button>
        <div id="queue-dashboard" className={offlineQueueCount > 0 ? "app-badge warn block" : "hidden"}>
          {offlineQueueCount} Payments Pending Sync
        </div>
        <div id="network-status-indicator" className={isOffline ? "app-badge warn block" : "hidden"} style={{ display: isOffline ? 'block' : 'none' }}>
          Offline - changes saved locally
        </div>
        {isSyncing && (
          <div className="fixed bottom-4 right-4 bg-[#0f766e] text-white px-4 py-3 rounded-xl shadow-lg font-medium animate-in slide-in-from-bottom-5 z-50 flex items-center gap-2">
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

      <div className="flex flex-col md:flex-col">
        <div className="order-last md:order-first">
          <WrappedWidget />
          <SuccessMilestoneWidget />
          <ViralLoopPerformanceWidget />
          <div className="mb-6">
            <div className="mb-4"><CartRecoveryWidget /></div>
            <AffiliateMarketingWidget />
          </div>
        </div>
      </div>

      <div className="mb-6 flex flex-col md:flex-row justify-between items-center gap-4">
          <SmartBlock type="PoweredBy" props={{ tenantId: tenantId(), isPremium: false }} />
          <button
            onClick={() => router.push("/incidents")}
            className="h-[44px] px-6 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 font-medium hover:bg-red-100 dark:hover:bg-red-900/40 transition-colors border border-red-200 dark:border-red-800/50"
            data-testid="report-incident-btn"
          >
            Report Incident
          </button>
      </div>

      <section className="app-panel glassmorphism border border-white/40 dark:border-white/10 mb-6">
        <div className="app-panel-header">
          <div>
            <h2 className="app-panel-title">2024 Store Wrapped</h2>
            <div className="app-list-subtitle">A shareable snapshot of your strongest store moments.</div>
          </div>
          <span className="app-badge good">Viral Loop</span>
        </div>
        <div className="app-panel-body">
          <p className="app-list-subtitle mb-3">Turn your sales, products, and milestones into a referral-friendly recap.</p>
          <Link href="/wrapped" className="app-button min-h-[44px]">View Your Wrapped 🎁</Link>
        </div>
      </section>

      {showMigration && (
        <section className="app-panel glassmorphism border border-white/40 dark:border-white/10 mb-6">
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
                  className="mt-2 w-full border border-gray-200 bg-white px-3 py-2 text-sm text-[#1D1D1F] shadow-sm dark:border-white/10 dark:bg-black/30 dark:text-[#F5F5F7]"
                  placeholder="mayas-cakes.myshopify.com"
                />
              </label>
              <button
                type="button"
                className="app-button primary min-h-[44px]"
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
                <button type="button" className="app-button min-h-[44px]" onClick={() => router.push("/products")}>
                  Review & Publish
                </button>
              </div>
            )}
          </div>
        </section>
      )}

      <main id="dashboard-screen" className="app-grid" style={{ gap: 16 }}>
        {activeDepartments.length > 0 && (
          <section className="mb-6 w-full col-span-full">
            <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Active AI Departments</h2>
            <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
              {activeDepartments.map(dept => (
                <div key={dept} className="glassmorphism p-4 border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-2">
                  <div className="flex items-center justify-between">
                    <span className="font-semibold text-sm text-[#1D1D1F] dark:text-[#F5F5F7]">{dept}</span>
                    <span className="w-2 h-2 rounded-full bg-[#34C759]"></span>
                  </div>
                  <span className="text-xs text-gray-500 dark:text-[#A1A1A6]">Active & Monitoring</span>
                </div>
              ))}
            </div>
          </section>
        )}

        <div className="mb-6 grid grid-cols-1 md:grid-cols-2 gap-4">
          <Link href="/assistant" className="app-card block p-5 min-h-[44px] hover:shadow-md transition-all group">
            <div className="flex items-center gap-4">
              <div className="w-11 h-11 bg-[#0f766e] flex items-center justify-center text-white text-xl shadow-sm">
                <span aria-hidden="true">A</span>
              </div>
              <div className="flex-1">
                <h3 className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Assistant Tasks</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">Open the dashboard task workspace for conversations, artifacts, and assistant actions.</p>
              </div>
              <div className="text-[#0f766e] opacity-0 group-hover:opacity-100 transition-opacity transform group-hover:translate-x-1 duration-200">
                →
              </div>
            </div>
          </Link>

          <Link id="sell-in-person-btn" href="/pos/terminal" className="app-card block p-5 min-h-[44px] hover:shadow-md transition-all group bg-gradient-to-br from-blue-50/50 to-indigo-50/50 dark:from-blue-900/20 dark:to-indigo-900/20 border border-blue-100/50 dark:border-blue-800/30">
            <div className="flex items-center gap-4">
              <div className="w-11 h-11 bg-[#0066FF] flex items-center justify-center text-white shadow-md shadow-blue-500/20">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" /></svg>
              </div>
              <div className="flex-1">
                <h3 className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Sell In Person</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">Tap-to-Pay, inventory sync, and cash sales.</p>
              </div>
              <div className="text-[#0066FF] opacity-0 group-hover:opacity-100 transition-opacity transform group-hover:translate-x-1 duration-200">
                →
              </div>
            </div>
          </Link>
        </div>

        <GrowBusinessCard />
          <PromoterCard />

        <section>
          <div className="mb-6 p-6 bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10">
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
                className="px-4 py-2 min-h-[44px] bg-[#0f766e] hover:bg-[#0d645d] text-white rounded-lg font-medium shadow-sm transition-colors"
              >
                Share & Claim Reward
              </button>
            </div>
          </div>

          <div className="mb-3 flex items-center justify-between gap-3">
            <div>
              <h2 className="app-panel-title">Business Analytics</h2>
              <p className="app-list-subtitle">Live performance, orders, and inbox activity.</p>
            </div>
            <Link href="/business-analytics" className="app-button min-h-[44px]">Business Analytics</Link>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-[1fr_300px] gap-6">
            <div className="app-grid metrics !grid-cols-2 lg:!grid-cols-4">
              <div className="app-card">
                <WithTooltip id="total-sales-tooltip" defaultText="Total revenue generated from your orders.">
                  <div className="app-metric-label">Total Sales</div>
                </WithTooltip>
                <div className="app-metric-value">{money(metrics.total_sales)}</div>
                <div className="app-metric-note">{loading ? "Loading your data..." : "All recorded orders"}</div>
              </div>
              <div className="app-card">
                <div className="app-metric-label">Customers</div>
                <div className="app-metric-value">{metrics.active_customers}</div>
                <div className="app-metric-note">Customer records</div>
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


          </div>
        </section>

        <section className="app-grid two">
          <div className="app-panel glassmorphism border border-white/40 dark:border-white/10">
            <div className="app-panel-header">
              <div>
                <div className="app-panel-title">Operations Map</div>
                <div className="app-list-subtitle">Live overview of your store workflow.</div>
              </div>
              <Link href="/orders" className="app-button min-h-[44px]">Open Orders</Link>
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
          </div>

        </section>


        <section className="app-grid two">
          <div className="app-panel glassmorphism border border-white/40 dark:border-white/10">
            <div className="app-panel-header">
              <WithTooltip id="recent-orders-tooltip" defaultText="View the latest orders placed by your customers."><div className="app-panel-title">Recent Orders</div></WithTooltip>
              <Link href="/orders" className="app-button min-h-[44px]">View All</Link>
            </div>
            {orders.length === 0 ? (
              <div className="app-empty">{loading ? "Loading your orders..." : "No orders found."}</div>
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

          <div className="app-panel glassmorphism border border-white/40 dark:border-white/10">
            <div className="app-panel-header">
              <WithTooltip id="inbox-activity-tooltip" defaultText="Keep track of recent customer messages."><div className="app-panel-title">Inbox Activity</div></WithTooltip>
              <Link href="/inbox" className="app-button min-h-[44px]">Open Inbox</Link>
            </div>
            <div className="app-list">
              {messages.length === 0 ? (
                <div className="app-empty">{loading ? "Loading your messages..." : "No messages found."}</div>
              ) : messages.slice(0, 6).map((message) => (
                <div key={message.id} className="app-list-item">
                  <div>
                    <div className="app-list-title">{message.source || "Unknown source"}</div>
                    <div className="app-list-subtitle">{message.content || "Empty message"}</div>
                  </div>
                  <span className={`app-badge ${statusTone(message.status)}`}>{formatStatus(message.status)}</span>
                </div>
              ))}
            </div>
          </div>
        </section>

        <section className="mt-4">
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
            <ReferralMilestonesWidget />
            <DashboardViralInviteWidget />
          </div>
          <div className="mb-3 flex items-center justify-between gap-3">
            <div>
              <h2 className="app-panel-title">Growth & Virality</h2>
              <p className="app-list-subtitle">Unlock new customers and track milestones.</p>
            </div>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <Link href="/feed" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-sky-50 dark:bg-sky-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">↗</div>
                <div className="text-sky-700 dark:text-sky-300 font-semibold text-sm bg-sky-50 dark:bg-sky-900/30 px-3 py-1 rounded-full">Orchestrate</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Campaign Orchestration</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Plan, generate, review, and launch customer campaigns from live dashboard data.</p>
            </Link>

            <Link href="/upgrade-roi" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">📈</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">ROI</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Pro Plan ROI Calculator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">See how much extra revenue you could generate by unlocking the Pro Plan.</p>
            </Link>

            <Link href="/zero-click-builder" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">⚡</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Growth</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Zero-Click Builder</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Generate a business in 30 seconds to show friends how fast OHC is.</p>
            </Link>

            <Link href="/referrals" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🤝</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Earn $50</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Referrals</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Invite other business owners to OHC and earn premium credits.</p>
            </Link>

            <Link href="/referrals" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-purple-50 dark:bg-purple-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🎁</div>
                <div className="text-purple-600 dark:text-purple-400 font-semibold text-sm bg-purple-50 dark:bg-purple-900/30 px-3 py-1 rounded-full">Referrals</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Referral Program</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Invite your network and earn credits for every business that signs up.</p>
            </Link>
            <Link href="/affiliate-badge-builder" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-orange-50 dark:bg-orange-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🏆</div>
                <div className="text-orange-600 dark:text-orange-400 font-semibold text-sm bg-orange-50 dark:bg-orange-900/30 px-3 py-1 rounded-full">Viral</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Affiliate Badge Builder</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Create an embeddable badge to grow your affiliate network.</p>
            </Link>

            <Link href="/finance" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-green-50 dark:bg-green-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">💰</div>
                <div className="text-green-600 dark:text-green-400 font-semibold text-sm bg-green-50 dark:bg-green-900/30 px-3 py-1 rounded-full">Finance</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Finance & Invoicing</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Manage cash flow, invoices, and automated payment follow-ups.</p>
            </Link>

            <Link href="/invoice-generator" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-cyan-50 dark:bg-cyan-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🧾</div>
                <div className="text-cyan-600 dark:text-cyan-400 font-semibold text-sm bg-cyan-50 dark:bg-cyan-900/30 px-3 py-1 rounded-full">Billing</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">AI Invoice Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Generate professional, shareable invoices that bring new customers to OHC.</p>
            </Link>

            <Link href="/pos/terminal" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">📱</div>
                <div className="text-[#0071E3] dark:text-blue-400 font-semibold text-sm bg-blue-50 dark:bg-blue-900/30 px-3 py-1 rounded-full">Sales</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Sell In Person</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Instantly collect payments with Tap-to-Pay and keep inventory synced.</p>
            </Link>

            <Link href="/proposal-generator" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">📝</div>
                <div className="text-[#0071E3] dark:text-blue-400 font-semibold text-sm bg-blue-50 dark:bg-blue-900/30 px-3 py-1 rounded-full">Sales</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">AI Proposal Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Create smart, shareable proposals with an interactive approval flow to win clients faster.</p>
            </Link>

            <Link href="/milestone-alerts" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-purple-50 dark:bg-purple-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform" aria-hidden="true">🏆</div>
                <div className="text-purple-600 dark:text-purple-400 font-semibold text-sm bg-purple-50 dark:bg-purple-900/30 px-3 py-1 rounded-full">Share</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Milestones</h3>

              <p className="text-sm text-gray-600 dark:text-gray-400">Track and share your business achievements with your audience.</p>
            </Link>

            <Link href="/loyalty-program" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-yellow-50 dark:bg-yellow-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🤝</div>
                <div className="text-yellow-600 dark:text-yellow-400 font-semibold text-sm bg-yellow-50 dark:bg-yellow-900/30 px-3 py-1 rounded-full">Loyalty</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Customer Loyalty</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Set up a 'Give X, Get Y' referral program and generate campaigns.</p>
            </Link>

            <Link href="/customer-referral-program" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-emerald-50 dark:bg-emerald-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">💸</div>
                <div className="text-emerald-600 dark:text-emerald-400 font-semibold text-sm bg-emerald-50 dark:bg-emerald-900/30 px-3 py-1 rounded-full">Referrals</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Customer Referral Program</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Launch a Give $10, Get $10 program to turn your customers into advocates.</p>
            </Link>

            <Link href="/share-cards" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-pink-50 dark:bg-pink-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🎴</div>
                <div className="text-pink-600 dark:text-pink-400 font-semibold text-sm bg-pink-50 dark:bg-pink-900/30 px-3 py-1 rounded-full">Cards</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Social Share Cards</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Generate Share Cards to promote your brand on social media.</p>
            </Link>

            <Link href="/storefront-widget" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🌐</div>
                <div className="text-[#0071E3] dark:text-blue-400 font-semibold text-sm bg-blue-50 dark:bg-blue-900/30 px-3 py-1 rounded-full">Widget</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Storefront Widget</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Embed a mini storefront on your blog or website to boost sales.</p>
            </Link>

            <Link href="/embed-builder" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-emerald-50 dark:bg-emerald-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🔌</div>
                <div className="text-emerald-600 dark:text-emerald-400 font-semibold text-sm bg-emerald-50 dark:bg-emerald-900/30 px-3 py-1 rounded-full">Widget</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Interactive Embed</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Build custom intake, booking, or quote widgets for your site.</p>
            </Link>

            <Link href="/subscriptions" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-amber-50 dark:bg-amber-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">📦</div>
                <div className="text-amber-700 dark:text-amber-300 font-semibold text-sm bg-amber-50 dark:bg-amber-900/30 px-3 py-1 rounded-full">Recurring</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Subscriptions & Fulfillments</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Manage recurring products, subscribers, and shipping batches.</p>
            </Link>

            <Link href="/social-proof-nudge" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-green-50 dark:bg-green-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🚀</div>
                <div className="text-green-600 dark:text-green-400 font-semibold text-sm bg-green-50 dark:bg-green-900/30 px-3 py-1 rounded-full">Proof</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Social Proof Nudge</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Show visitors that others are buying to increase conversions.</p>
            </Link>

            <Link href="/work-intake-widget" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">📋</div>
                <div className="text-[#0071E3] dark:text-blue-400 font-semibold text-sm bg-blue-50 dark:bg-blue-900/30 px-3 py-1 rounded-full">Leads</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Work-Intake Widget</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Embed a smart lead capture form with a viral loop directly on your site.</p>
            </Link>

            <WithTooltip id="link-in-bio-tooltip" defaultText="One link to rule them all. Drive social traffic to your store.">
            <Link href="/link-in-bio-generator" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-emerald-50 dark:bg-emerald-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🔗</div>
                <div className="text-emerald-600 dark:text-emerald-400 font-semibold text-sm bg-emerald-50 dark:bg-emerald-900/30 px-3 py-1 rounded-full">Bio</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Create Link-in-Bio Page</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Publish a lightweight social profile page for your storefront and offers.</p>
            </Link>
            </WithTooltip>

            <WithTooltip id="quiz-generator-tooltip" defaultText="Create AI-powered product recommendation quizzes to capture leads.">
            <Link href="/quiz-generator" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🧠</div>
                <span className="text-xs font-semibold px-2 py-1 bg-blue-100 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300 rounded-full tracking-wider uppercase">Lead Gen</span>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Quiz Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Create viral quizzes that capture leads and spread through social sharing.</p>
            </Link>
            </WithTooltip>

            <Link href="/whatsapp-link-generator" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-green-50 dark:bg-green-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">💬</div>
                <div className="text-green-600 dark:text-green-400 font-semibold text-sm bg-green-50 dark:bg-green-900/30 px-3 py-1 rounded-full">Social</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">WhatsApp Link Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Create shareable WhatsApp links to start conversations instantly.</p>
            </Link>

            <Link href="/calendar" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">📅</div>
                <div className="text-[#0071E3] dark:text-blue-400 font-semibold text-sm bg-blue-50 dark:bg-blue-900/30 px-3 py-1 rounded-full">Schedule</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Calendar</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">View upcoming appointments and sync with your AI operations assistant.</p>
            </Link>

            <Link href="/giveaway" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-pink-50 dark:bg-pink-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🎁</div>
                <div className="text-pink-600 dark:text-pink-400 font-semibold text-sm bg-pink-50 dark:bg-pink-900/30 px-3 py-1 rounded-full">Viral</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Viral Giveaway Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Launch a viral sweepstakes to capture emails and drive social shares.</p>
            </Link>

            <Link href="/viral-leaderboard-generator" id="viral-leaderboard-link" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-yellow-50 dark:bg-yellow-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🏆</div>
                <div className="text-yellow-600 dark:text-yellow-400 font-semibold text-sm bg-yellow-50 dark:bg-yellow-900/30 px-3 py-1 rounded-full">Growth</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Viral Leaderboard</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Embed a gamified leaderboard on your storefront to encourage competition and referrals.</p>
            </Link>

            <Link href="/share-and-save-widget" id="share-and-save-link" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">💸</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Growth</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Share & Save Widget</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">One-Tap share widget to reward your customers with discounts for sharing your storefront.</p>
            </Link>

            <Link href="/share-to-unlock-generator" id="share-to-unlock-link" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🔓</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Growth</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Share-to-Unlock Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Require customers to share your page on social media to reveal a discount code.</p>
            </Link>

            <Link href="/interactive-quote-generator" id="interactive-quote-link" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🧮</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Growth</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Interactive Quote Widget</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Build a pricing calculator to embed on your site to capture leads and drive referrals.</p>
            </Link>

            <Link href="/win-back" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-purple-50 dark:bg-purple-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">💌</div>
                <div className="text-purple-600 dark:text-purple-400 font-semibold text-sm bg-purple-50 dark:bg-purple-900/30 px-3 py-1 rounded-full">Retain</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Customer Win-back</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Re-engage inactive customers with AI-generated email campaigns.</p>
            </Link>

            <Link href="/review-campaigns" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-yellow-50 dark:bg-yellow-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">⭐️</div>
                <div className="text-yellow-600 dark:text-yellow-400 font-semibold text-sm bg-yellow-50 dark:bg-yellow-900/30 px-3 py-1 rounded-full">Reviews</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Automated Reviews</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Generate highly-converting, personalized review request emails.</p>
            </Link>

            <Link href="/seasonal-promo" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-teal-50 dark:bg-teal-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">✨</div>
                <div className="text-teal-600 dark:text-teal-400 font-semibold text-sm bg-teal-50 dark:bg-teal-900/30 px-3 py-1 rounded-full">Promo</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Seasonal Promo Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Create AI campaigns and promo codes for special occasions instantly.</p>
            </Link>

            <WithTooltip id="cart-recovery-tooltip" defaultText="Recover abandoned carts with personalized AI follow-ups.">
            <Link href="/cart-recovery" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-orange-50 dark:bg-orange-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🛒</div>
                <div className="text-orange-600 dark:text-orange-400 font-semibold text-sm bg-orange-50 dark:bg-orange-900/30 px-3 py-1 rounded-full">Recover</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Cart Recovery</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Recover abandoned carts with personalized AI follow-ups.</p>
            </Link>
            </WithTooltip>

            <WithTooltip id="flash-sale-tooltip" defaultText="Create high-converting flash sale countdown widgets.">
            <Link href="/flash-sale-generator" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-red-50 dark:bg-red-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">⚡</div>
                <div className="text-red-600 dark:text-red-400 font-semibold text-sm bg-red-50 dark:bg-red-900/30 px-3 py-1 rounded-full">Urgency</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Flash Sale Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Create high-converting flash sale countdown widgets.</p>
            </Link>
            </WithTooltip>


            <WithTooltip id="pre-order-tooltip" defaultText="Launch an omnichannel pre-order engine with tiered waitlist capabilities.">
            <Link href="/pre-order-widget" id="pre-order-waitlist-link" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">⏳</div>
                <div className="text-[#0071E3] dark:text-blue-400 font-semibold text-sm bg-blue-50 dark:bg-blue-900/30 px-3 py-1 rounded-full">Virality</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Pre-Order Waitlist</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Launch an omnichannel pre-order engine with tiered waitlist capabilities.</p>
            </Link>
            </WithTooltip>

            <WithTooltip id="pos-tooltip" defaultText="Universal Mobile POS & Tap-to-Pay with Agentic Inventory Sync.">
            <Link href="/pos/terminal" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">💳</div>
                <div className="text-[#0071E3] dark:text-blue-400 font-semibold text-sm bg-blue-50 dark:bg-blue-900/30 px-3 py-1 rounded-full">Sales</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Sell In Person</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Universal Mobile POS & Tap-to-Pay.</p>
            </Link>
            </WithTooltip>

            <WithTooltip id="discount-code-tooltip" defaultText="Create discount code widgets for your customers.">
            <Link href="/discount-code-generator" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🎯</div>
                <div className="text-[#0071E3] dark:text-blue-400 font-semibold text-sm bg-blue-50 dark:bg-blue-900/30 px-3 py-1 rounded-full">Leads</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Discount Code Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Create discount code widgets for your customers.</p>
            </Link>
            </WithTooltip>

            <WithTooltip id="lead-magnet-tooltip" defaultText="Capture emails and grow your audience.">
            <Link href="/lead-magnet-generator" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🧲</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Leads</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Lead Magnet Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Capture emails and grow your audience.</p>
            </Link>
            </WithTooltip>

            <WithTooltip id="link-in-bio-tooltip" defaultText="One link to rule them all. Drive social traffic to your store.">
            <Link href="/link-in-bio-generator" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🔗</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Growth</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Link in Bio Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">One link to rule them all. Drive social traffic to your store.</p>
            </Link>
            </WithTooltip>

            <WithTooltip id="testimonial-widget-tooltip" defaultText="Build a custom testimonial widget to increase social proof and conversions.">
            <Link href="/testimonial-widget" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-yellow-50 dark:bg-yellow-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🌟</div>
                <div className="text-yellow-600 dark:text-yellow-400 font-semibold text-sm bg-yellow-50 dark:bg-yellow-900/30 px-3 py-1 rounded-full">Growth</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Testimonial Widget</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Build a custom testimonial widget to increase social proof and conversions.</p>
            </Link>
            </WithTooltip>

            <WithTooltip id="spin-to-win-tooltip" defaultText="Create interactive discount wheels to capture emails.">
            <Link href="/spin-to-win-generator" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-purple-50 dark:bg-purple-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🎡</div>
                <div className="text-purple-600 dark:text-purple-400 font-semibold text-sm bg-purple-50 dark:bg-purple-900/30 px-3 py-1 rounded-full">Gamification</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Spin to Win Generator</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Create interactive discount wheels to capture emails.</p>
            </Link>
            </WithTooltip>
            <WithTooltip id="trial-extension-tooltip" defaultText="Share your setup on X to instantly unlock 7 extra days of Pro.">
            <Link href="/trial-extension" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-emerald-50 dark:bg-emerald-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">🎁</div>
                <div className="text-emerald-600 dark:text-emerald-400 font-semibold text-sm bg-emerald-50 dark:bg-emerald-900/30 px-3 py-1 rounded-full">Extension</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Interactive Trial Extension</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Share your setup on X to instantly unlock 7 extra days of Pro.</p>
            </Link>
            </WithTooltip>

            <WithTooltip id="field-ops-tooltip" defaultText="Offline-first mobile route management for field service workers.">
            <Link href="/field-ops/jobs" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">📍</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Operations</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Field Ops Route</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Offline-first mobile route management for field service workers.</p>
            </Link>
            </WithTooltip>


            <WithTooltip id="my-plan-tooltip" defaultText="Manage your subscription, usage, and billing.">
            <Link href="/plan" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">💳</div>
                <div className="text-[#0071E3] dark:text-blue-400 font-semibold text-sm bg-blue-50 dark:bg-blue-900/30 px-3 py-1 rounded-full">Billing</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">My Plan</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Manage your subscription, usage, and billing.</p>
            </Link>
            </WithTooltip>
            <WithTooltip id="proposal-draft-tooltip" defaultText="Generate complex AI proposals instantly.">
            <Link href="/proposals/new" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">📝</div>
                <div className="text-indigo-600 dark:text-indigo-400 font-semibold text-sm bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 rounded-full">Proposals</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Proposal Draft</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Generate complex AI proposals instantly.</p>
            </Link>
            </WithTooltip>

            <WithTooltip id="settings-widget-tooltip" defaultText="Manage your account and preferences.">
            <Link href="/settings" className="block glassmorphism p-6 min-h-[44px] hover:shadow-lg transition-all hover:-translate-y-0.5 group border border-white/40 dark:border-white/10">
              <div className="flex items-start justify-between mb-4">
                <div className="w-12 h-12 rounded-full bg-gray-50 dark:bg-gray-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">⚙️</div>
                <div className="text-gray-600 dark:text-gray-400 font-semibold text-sm bg-gray-50 dark:bg-gray-900/30 px-3 py-1 rounded-full">Config</div>
              </div>
              <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Settings</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Manage your account and preferences.</p>
            </Link>
            </WithTooltip>
          </div>
        </section>
      </main>
      </div>

    </AppShell>
    </>
  );
}
