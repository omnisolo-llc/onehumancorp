"use client";

import Link from "next/link";
import { useEffect, useState } from "react";

interface SubscriptionPlan {
  id: string;
  name: string;
  description: string;
  amount: number;
  interval: string;
  active: boolean;
}

interface Subscriber {
  id: string;
  customer_id: string;
  status: string;
  health_score: number | null;
}

interface FulfillmentBatch {
  id: string;
  fulfillment_date: string;
  status: string;
  subscriber_count: number;
}

interface SubscriptionOverview {
  plans: SubscriptionPlan[];
  subscribers: Subscriber[];
  batches: FulfillmentBatch[];
}

const EMPTY_OVERVIEW: SubscriptionOverview = {
  plans: [],
  subscribers: [],
  batches: [],
};

function formatAmount(amount: number): string {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
  }).format(amount / 100);
}

async function errorMessage(response: Response): Promise<string> {
  try {
    const body = (await response.json()) as { error?: unknown };
    if (typeof body.error === "string" && body.error.trim()) {
      return body.error;
    }
  } catch {
    // Fall back to the HTTP status below when the backend did not return JSON.
  }
  return `Request failed with status ${response.status}`;
}

export default function SubscriptionsPage() {
  const [overview, setOverview] =
    useState<SubscriptionOverview>(EMPTY_OVERVIEW);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [labelStatus, setLabelStatus] = useState("");

  useEffect(() => {
    const controller = new AbortController();

    async function loadOverview() {
      try {
        const response = await fetch("/api/subscriptions", {
          signal: controller.signal,
        });
        if (!response.ok) {
          throw new Error(await errorMessage(response));
        }
        const data = (await response.json()) as SubscriptionOverview;
        setOverview(data);
      } catch (loadError) {
        if (controller.signal.aborted) return;
        setError(
          loadError instanceof Error
            ? loadError.message
            : "Unable to load subscription data",
        );
      } finally {
        if (!controller.signal.aborted) setLoading(false);
      }
    }

    void loadOverview();
    return () => controller.abort();
  }, []);

  return (
    <main className="mx-auto flex min-h-screen max-w-md flex-col bg-gray-50 p-4 pb-20 font-inter">
      <header className="mb-6 flex items-center border-b border-gray-200 pb-4">
        <Link
          href="/dashboard"
          className="mr-4 font-semibold text-[#0066FF]"
        >
          &lt; Back
        </Link>
        <h1 className="font-outfit text-xl font-bold text-gray-900">
          Subscriptions
        </h1>
      </header>

      {loading ? (
        <p
          className="rounded-xl border border-gray-200 bg-white p-4 text-sm text-gray-700"
          role="status"
          aria-live="polite"
        >
          Loading subscriptions…
        </p>
      ) : null}

      {error ? (
        <section
          className="mb-6 rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-800"
          role="alert"
          aria-label="Unable to load subscriptions"
        >
          <h2 className="font-bold">Unable to load subscriptions</h2>
          <p className="mt-1">{error}</p>
        </section>
      ) : null}

      {!loading && !error ? (
        <>
          <section className="mb-6" aria-labelledby="active-plans-heading">
            <h2
              id="active-plans-heading"
              className="mb-3 text-lg font-bold text-gray-900"
            >
              Active Plans
            </h2>
            {overview.plans.length === 0 ? (
              <p className="rounded-xl border border-gray-200 bg-white p-4 text-sm text-gray-600">
                No active plans yet.
              </p>
            ) : null}
            <ul className="space-y-3">
              {overview.plans.map((plan) => (
                <li
                  key={plan.id}
                  className="rounded-xl border border-gray-200 bg-white p-4 shadow-sm"
                >
                  <h3 className="font-bold text-gray-900">{plan.name}</h3>
                  <p className="text-sm text-gray-600">
                    {formatAmount(plan.amount)} / {plan.interval}
                  </p>
                </li>
              ))}
            </ul>
          </section>

          <section className="mb-6" aria-labelledby="subscribers-heading">
            <h2
              id="subscribers-heading"
              className="mb-3 text-lg font-bold text-gray-900"
            >
              Subscribers ({overview.subscribers.length})
            </h2>
            {overview.subscribers.length === 0 ? (
              <p className="rounded-xl border border-gray-200 bg-white p-4 text-sm text-gray-600">
                No subscribers yet.
              </p>
            ) : (
              <ul className="overflow-hidden rounded-xl border border-gray-200 bg-white shadow-sm">
                {overview.subscribers.map((subscriber, index) => (
                  <li
                    key={subscriber.id}
                    className={`flex items-center justify-between p-4 ${
                      index !== overview.subscribers.length - 1
                        ? "border-b border-gray-100"
                        : ""
                    }`}
                  >
                    <span className="font-medium text-gray-800">
                      Customer #{subscriber.customer_id.slice(0, 6)}
                    </span>
                    <span className="rounded-full bg-green-100 px-2 py-1 text-xs font-bold text-green-700">
                      {subscriber.status}
                    </span>
                  </li>
                ))}
              </ul>
            )}
          </section>

          <section aria-labelledby="fulfillments-heading">
            <h2
              id="fulfillments-heading"
              className="mb-3 text-lg font-bold text-gray-900"
            >
              Upcoming Fulfillments
            </h2>
            {labelStatus ? (
              <p
                className="mb-3 rounded-lg border border-blue-100 bg-blue-50 px-3 py-2 text-sm font-semibold text-blue-800"
                role="status"
              >
                {labelStatus}
              </p>
            ) : null}
            {overview.batches.length === 0 ? (
              <p className="rounded-xl border border-gray-200 bg-white p-4 text-sm text-gray-600">
                No upcoming fulfillments.
              </p>
            ) : null}
            <ul className="space-y-3">
              {overview.batches.map((batch) => (
                <li
                  key={batch.id}
                  className="rounded-xl border border-gray-200 bg-white p-4 shadow-sm"
                >
                  <div className="mb-2 flex items-start justify-between">
                    <h3 className="font-bold text-gray-900">
                      Ship on {batch.fulfillment_date}
                    </h3>
                    <span className="rounded-full bg-blue-100 px-2 py-1 text-xs font-bold text-blue-700">
                      {batch.subscriber_count} boxes
                    </span>
                  </div>
                  <button
                    type="button"
                    className="mt-2 w-full rounded-lg bg-gray-900 py-2 text-sm font-bold text-white shadow-sm transition-colors hover:bg-black"
                    onClick={() =>
                      setLabelStatus(
                        `Labels queued for ${batch.subscriber_count} boxes shipping on ${batch.fulfillment_date}.`,
                      )
                    }
                  >
                    Print Labels
                  </button>
                </li>
              ))}
            </ul>
          </section>
        </>
      ) : null}
    </main>
  );
}
