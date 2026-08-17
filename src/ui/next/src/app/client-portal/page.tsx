"use client";

import {
  CreditCard,
  FileText,
  LifeBuoy,
  MessageSquareText,
  RefreshCw,
  Repeat2,
} from "lucide-react";
import Link from "next/link";
import { useCallback, useEffect, useState } from "react";

type BillingPlan = {
  current_plan?: string;
  ai_actions_used?: number;
  ai_actions_limit?: number;
  next_bill_estimated?: number;
};

type SubscriptionOverview = {
  plans: Array<{ id: string; name: string; active: boolean }>;
  subscribers: Array<{ id: string; status: string }>;
  batches: Array<{ id: string; status: string }>;
};

type PortalData = {
  billing: BillingPlan | null;
  subscriptions: SubscriptionOverview | null;
};

const EMPTY_DATA: PortalData = { billing: null, subscriptions: null };

const portalActions = [
  {
    href: "/quoting",
    label: "Quotes and proposals",
    description: "Review quote details and approval status.",
    icon: FileText,
  },
  {
    href: "/subscriptions",
    label: "Subscriptions",
    description: "View plans, subscribers, and fulfillment batches.",
    icon: Repeat2,
  },
  {
    href: "/cost-dashboard",
    label: "Billing",
    description: "Manage invoices, usage, and billing settings.",
    icon: CreditCard,
  },
  {
    href: "/inbox",
    label: "Support messages",
    description: "Continue customer and support conversations.",
    icon: MessageSquareText,
  },
] as const;

async function readJson<T>(url: string, signal: AbortSignal): Promise<T> {
  const response = await fetch(url, { signal, cache: "no-store" });
  if (!response.ok) throw new Error(`${url} returned HTTP ${response.status}`);
  return response.json() as Promise<T>;
}

function formatLimit(value: number | undefined): string {
  return typeof value === "number" ? value.toLocaleString() : "Not reported";
}

function formatMoney(cents: number | undefined): string {
  if (typeof cents !== "number") return "Not reported";
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
  }).format(cents / 100);
}

export default function ClientPortalPage() {
  const [data, setData] = useState<PortalData>(EMPTY_DATA);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [reloadKey, setReloadKey] = useState(0);

  const reload = useCallback(() => setReloadKey((key) => key + 1), []);

  useEffect(() => {
    const controller = new AbortController();

    async function loadPortal() {
      setLoading(true);
      setError(null);
      const [billing, subscriptions] = await Promise.allSettled([
        readJson<BillingPlan>("/api/v1/billing/my-plan", controller.signal),
        readJson<SubscriptionOverview>("/api/v1/subscriptions", controller.signal),
      ]);
      if (controller.signal.aborted) return;

      const nextData = {
        billing: billing.status === "fulfilled" ? billing.value : null,
        subscriptions:
          subscriptions.status === "fulfilled" ? subscriptions.value : null,
      };
      setData(nextData);
      if (nextData.billing === null && nextData.subscriptions === null) {
        setError("Client data is currently unavailable.");
      }
      setLoading(false);
    }

    void loadPortal();
    return () => controller.abort();
  }, [reloadKey]);

  const activePlans =
    data.subscriptions?.plans.filter((plan) => plan.active).length;

  return (
    <main
      className="space-y-6"
      aria-labelledby="client-portal-heading"
      data-client-portal-state={loading ? "loading" : "settled"}
    >
      <section className="flex flex-col gap-4 border-b border-[#D2D2D7] pb-5 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <p className="app-eyebrow">Customer workspace</p>
          <h2 id="client-portal-heading" className="text-xl font-semibold text-[#1D1D1F]">
            Account overview
          </h2>
          <p className="mt-1 text-sm text-[#6E6E73]">
            Live plan, subscription, billing, and support status.
          </p>
        </div>
        <button
          type="button"
          className="app-button min-h-[44px] self-start"
          onClick={reload}
          disabled={loading}
        >
          <RefreshCw aria-hidden="true" size={16} />
          Refresh
        </button>
      </section>

      {loading ? (
        <div className="app-panel p-5 text-sm text-[#6E6E73]" role="status">
          Loading client data...
        </div>
      ) : null}

      {error ? (
        <div className="app-panel border-[#FF3B30] p-5" role="alert">
          <div className="flex items-start gap-3">
            <LifeBuoy aria-hidden="true" className="mt-0.5 text-[#C9342C]" size={18} />
            <div>
              <h3 className="font-semibold text-[#1D1D1F]">Unable to load account data</h3>
              <p className="mt-1 text-sm text-[#6E6E73]">{error}</p>
            </div>
          </div>
        </div>
      ) : null}

      {!loading && (data.billing || data.subscriptions) ? (
        <section className="grid gap-3 sm:grid-cols-2 lg:grid-cols-4" aria-label="Account status">
          <div className="app-card p-4">
            <p className="text-xs font-semibold text-[#6E6E73]">Current plan</p>
            <p className="mt-2 text-lg font-semibold text-[#1D1D1F]">
              {data.billing?.current_plan ?? "Not reported"}
            </p>
          </div>
          <div className="app-card p-4">
            <p className="text-xs font-semibold text-[#6E6E73]">AI actions</p>
            <p className="mt-2 text-lg font-semibold text-[#1D1D1F]">
              {formatLimit(data.billing?.ai_actions_used)}
              <span className="text-sm font-normal text-[#6E6E73]">
                {" / "}{formatLimit(data.billing?.ai_actions_limit)}
              </span>
            </p>
          </div>
          <div className="app-card p-4">
            <p className="text-xs font-semibold text-[#6E6E73]">Active plans</p>
            <p className="mt-2 text-lg font-semibold text-[#1D1D1F]">
              {activePlans ?? "Not reported"}
            </p>
          </div>
          <div className="app-card p-4">
            <p className="text-xs font-semibold text-[#6E6E73]">Estimated next bill</p>
            <p className="mt-2 text-lg font-semibold text-[#1D1D1F]">
              {formatMoney(data.billing?.next_bill_estimated)}
            </p>
          </div>
        </section>
      ) : null}

      <section aria-labelledby="client-actions-heading">
        <h3 id="client-actions-heading" className="mb-3 text-sm font-semibold text-[#1D1D1F]">
          Account tools
        </h3>
        <div className="grid gap-3 md:grid-cols-2">
          {portalActions.map((action) => {
            const Icon = action.icon;
            return (
              <Link key={action.href} href={action.href} className="app-card p-4 transition-colors hover:border-[#0066FF]">
                <div className="flex items-start gap-3">
                  <span className="flex h-9 w-9 shrink-0 items-center justify-center rounded-md bg-[#EAF2FF] text-[#0066FF]">
                    <Icon aria-hidden="true" size={18} />
                  </span>
                  <span>
                    <span className="block font-semibold text-[#1D1D1F]">{action.label}</span>
                    <span className="mt-1 block text-sm text-[#6E6E73]">{action.description}</span>
                  </span>
                </div>
              </Link>
            );
          })}
        </div>
      </section>
    </main>
  );
}
