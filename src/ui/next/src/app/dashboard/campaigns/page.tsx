"use client";

import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { AppShell } from "../../components/AppShell";

type DashboardMetrics = {
  active_customers?: number;
  pending_orders?: number;
  total_sales?: number;
  total_campaigns_sent?: number;
};

type Order = {
  id: string;
  customer_name?: string;
  customer_email?: string;
  email?: string;
  total_amount?: number;
  status?: string;
  product_name?: string;
};

type InboxMessage = {
  id: string;
  source?: string;
  content?: string;
  status?: string;
};

type SupplyPayload = {
  vendors?: unknown[];
  raw_materials?: Array<{
    id: string;
    name: string;
    current_quantity: number;
    reorder_threshold: number;
  }>;
  bom_items?: unknown[];
};

type CampaignKey = "review" | "receipt" | "referral";

type CampaignWorkflow = {
  key: CampaignKey;
  title: string;
  endpoint: string;
  methodLabel: string;
  description: string;
  href: string;
  linkLabel: string;
  ready: boolean;
  emptyReason: string;
  payload: Record<string, string | number | undefined>;
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

function money(value?: number) {
  return new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" }).format(value || 0);
}

function customerEmail(order?: Order) {
  return order?.customer_email || order?.email;
}

function firstEligibleOrder(orders: Order[]) {
  return orders.find((order) => order.customer_name || customerEmail(order)) || orders[0];
}

export default function CampaignOrchestrationPage() {
  const [metrics, setMetrics] = useState<DashboardMetrics>(emptyMetrics);
  const [orders, setOrders] = useState<Order[]>([]);
  const [messages, setMessages] = useState<InboxMessage[]>([]);
  const [supply, setSupply] = useState<SupplyPayload>({});
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [selected, setSelected] = useState<CampaignKey>("review");
  const [draft, setDraft] = useState("");
  const [actionStatus, setActionStatus] = useState("");
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    async function loadCampaignContext() {
      const tenant = encodeURIComponent(tenantId());
      setLoading(true);
      setError("");

      try {
        const unifiedRes = await fetch(`/api/ui/dashboard/unified-feed?tenant_id=${tenant}`);

        if (!unifiedRes.ok) {
          throw new Error("Campaign context could not be loaded from the backend UI endpoints.");
        }

        const unifiedData = await unifiedRes.json();

        const metricsData = unifiedData.metrics || {};
        const ordersData = unifiedData.orders || [];
        const inboxData = unifiedData.inbox || [];
        const supplyData = unifiedData.supply || {};

        setMetrics({ ...emptyMetrics, ...metricsData });
        setOrders(Array.isArray(ordersData) ? ordersData : []);
        setMessages(Array.isArray(inboxData) ? inboxData : []);
        setSupply(supplyData && typeof supplyData === "object" ? supplyData : {});
      } catch (err: any) {
        setError(err?.message || "Campaign context could not be loaded.");
      } finally {
        setLoading(false);
      }
    }

    loadCampaignContext();
  }, []);

  const primaryOrder = useMemo(() => firstEligibleOrder(orders), [orders]);
  const openMessages = useMemo(
    () => messages.filter((message) => (message.status || "open").toLowerCase() !== "closed").length,
    [messages],
  );
  const lowStockCount = useMemo(
    () => (supply.raw_materials || []).filter((item) => item.current_quantity <= item.reorder_threshold).length,
    [supply.raw_materials],
  );
  const tenant = tenantId();
  const workflows: CampaignWorkflow[] = [
    {
      key: "review",
      title: "Review request",
      endpoint: "/api/v1/growth/campaign/generate-review",
      methodLabel: "Generate review draft",
      description: "Use the latest customer order returned by the orders endpoint to prepare a review request.",
      href: "/review-campaigns",
      linkLabel: "Open review workflow",
      ready: Boolean(primaryOrder?.id),
      emptyReason: "No order rows are available for review targeting.",
      payload: {
        customer_name: primaryOrder?.customer_name,
        customer_email: customerEmail(primaryOrder),
        order_id: primaryOrder?.id,
        product_name: primaryOrder?.product_name || "recent purchase",
        tenant_id: tenant,
      },
    },
    {
      key: "receipt",
      title: "Receipt follow-up",
      endpoint: "/api/v1/growth/campaign/send-receipt",
      methodLabel: "Send receipt campaign",
      description: "Send or preview the post-purchase receipt campaign against the selected order context.",
      href: primaryOrder?.id ? `/orders/${primaryOrder.id}` : "/orders",
      linkLabel: "Open receipt workflow",
      ready: Boolean(primaryOrder?.id),
      emptyReason: "No order rows are available for receipt follow-up.",
      payload: {
        customer_email: customerEmail(primaryOrder),
        order_id: primaryOrder?.id,
        amount: money(primaryOrder?.total_amount),
        tenant_id: tenant,
      },
    },
    {
      key: "referral",
      title: "Customer referral",
      endpoint: "/api/v1/growth/campaign/generate-customer-referral",
      methodLabel: "Generate referral draft",
      description: "Create a referral invitation only when the dashboard has customer records to target.",
      href: "/referrals",
      linkLabel: "Open referral workflow",
      ready: (metrics.active_customers || 0) > 0,
      emptyReason: "No active customer count has been returned yet.",
      payload: {
        store_name: tenant,
        tenant_id: tenant,
      },
    },
  ];
  const activeWorkflow = workflows.find((workflow) => workflow.key === selected) || workflows[0];

  async function runWorkflow(workflow: CampaignWorkflow) {
    setSelected(workflow.key);
    setDraft("");
    setActionStatus("");

    if (!workflow.ready) {
      setActionStatus(workflow.emptyReason);
      return;
    }

    setSubmitting(true);
    try {
      const response = await fetch(workflow.endpoint, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(workflow.payload),
      });

      if (!response.ok) {
        throw new Error(`${workflow.title} request failed.`);
      }

      const data = await response.json();
      setDraft(data.message || data.draft || "Campaign request completed.");
      setActionStatus(`${workflow.title} is ready for review.`);
    } catch (err: any) {
      setActionStatus(err?.message || "Campaign request failed.");
    } finally {
      setSubmitting(false);
    }
  }

  const statusItems = [
    { label: "API", value: error ? "Degraded" : "Online", tone: error ? "bad" as const : "good" as const },
    { label: "Orders", value: String(orders.length), tone: orders.length > 0 ? "good" as const : "warn" as const },
    { label: "Inbox", value: String(openMessages), tone: openMessages > 0 ? "warn" as const : "neutral" as const },
    { label: "Sent", value: String(metrics.total_campaigns_sent || 0), tone: "good" as const },
  ];

  return (
    <AppShell
      title="Campaign Orchestration"
      subtitle="Coordinate campaign generation, review, and launch from live dashboard context."
      statusItems={statusItems}
      actions={[{ label: "Dashboard", href: "/dashboard", icon: "dashboard" }]}
    >
      <section className="app-grid metrics !grid-cols-2 lg:!grid-cols-4 mb-6">
        <div className="app-card">
          <div className="app-metric-label">Customers</div>
          <div className="app-metric-value">{metrics.active_customers || 0}</div>
          <div className="app-metric-note">From dashboard metrics</div>
        </div>
        <div className="app-card">
          <div className="app-metric-label">Orders</div>
          <div className="app-metric-value">{orders.length}</div>
          <div className="app-metric-note">{loading ? "Loading order rows" : "Available targets"}</div>
        </div>
        <div className="app-card">
          <div className="app-metric-label">Open Inbox</div>
          <div className="app-metric-value">{openMessages}</div>
          <div className="app-metric-note">Customer context</div>
        </div>
        <div className="app-card">
          <div className="app-metric-label">Low Stock</div>
          <div className="app-metric-value">{lowStockCount}</div>
          <div className="app-metric-note">Supply risk before promotions</div>
        </div>
      </section>

      {error && <div className="app-badge bad mb-4">{error}</div>}

      <section className="app-grid two">
        <div className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Campaign Command Queue</div>
              <div className="app-list-subtitle">Each action uses existing campaign API routes and current tenant data.</div>
            </div>
          </div>
          <div className="app-list">
            {workflows.map((workflow) => (
              <div key={workflow.key} className="app-list-item items-start">
                <div className="min-w-0">
                  <div className="app-list-title">{workflow.title}</div>
                  <div className="app-list-subtitle">{workflow.description}</div>
                  {!workflow.ready && <div className="app-list-subtitle mt-1 text-amber-700">{workflow.emptyReason}</div>}
                </div>
                <div className="flex shrink-0 flex-col gap-2 sm:flex-row">
                  <button
                    type="button"
                    className={selected === workflow.key ? "app-button primary min-h-[44px]" : "app-button min-h-[44px]"}
                    disabled={submitting}
                    onClick={() => runWorkflow(workflow)}
                  >
                    {submitting && selected === workflow.key ? "Working..." : workflow.methodLabel}
                  </button>
                  <Link href={workflow.href} className="app-button min-h-[44px]">
                    {workflow.linkLabel}
                  </Link>
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Review & Launch</div>
              <div className="app-list-subtitle">{activeWorkflow.title} output and next step.</div>
            </div>
            <span className={activeWorkflow.ready ? "app-badge good" : "app-badge warn"}>
              {activeWorkflow.ready ? "Ready" : "Needs data"}
            </span>
          </div>
          <div className="app-panel-body">
            {actionStatus && <p className="mb-3 app-list-subtitle" role="status">{actionStatus}</p>}
            {draft ? (
              <pre className="min-h-[220px] whitespace-pre-wrap border border-gray-200 bg-white p-4 text-sm text-gray-800 shadow-sm dark:border-white/10 dark:bg-black/30 dark:text-gray-100">
                {draft}
              </pre>
            ) : (
              <div className="app-empty">
                {loading ? "Loading campaign context..." : "Choose a campaign action to generate a draft or send workflow request."}
              </div>
            )}
            <div className="mt-4 flex flex-wrap gap-2">
              <Link href={activeWorkflow.href} className="app-button primary min-h-[44px]">
                Continue in workflow
              </Link>
              <Link href="/dashboard" className="app-button min-h-[44px]">
                Back to Dashboard
              </Link>
            </div>
          </div>
        </div>
      </section>
    </AppShell>
  );
}
