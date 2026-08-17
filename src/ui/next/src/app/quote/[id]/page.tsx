"use client";

import { CheckCircle2, RefreshCw } from "lucide-react";
import { useParams } from "next/navigation";
import { useCallback, useEffect, useState } from "react";

type Quote = Readonly<{
  id: string;
  status: string;
  total_amount_cents: number | null;
  required_deposit_cents: number | null;
}>;

type QuoteLineItem = Readonly<{
  id: string;
  description: string;
  unit_price_cents: number;
  quantity: number;
}>;

type QuoteResponse = Readonly<{ quote: Quote; line_items: QuoteLineItem[] }>;

const QUOTE_ID = /^[A-Za-z0-9._-]{1,128}$/;

function parseQuote(value: unknown): QuoteResponse | null {
  if (value === null || typeof value !== "object") return null;
  const candidate = value as Partial<QuoteResponse>;
  if (candidate.quote === null || typeof candidate.quote !== "object") return null;
  if (!Array.isArray(candidate.line_items)) return null;
  if (typeof candidate.quote.id !== "string" || typeof candidate.quote.status !== "string") return null;
  const validItems = candidate.line_items.every((item) => item !== null
    && typeof item === "object"
    && typeof item.id === "string"
    && typeof item.description === "string"
    && Number.isSafeInteger(item.unit_price_cents)
    && Number.isSafeInteger(item.quantity));
  return validItems ? candidate as QuoteResponse : null;
}

function formatMoney(cents: number | null) {
  return new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" })
    .format((cents ?? 0) / 100);
}

export default function InteractiveQuotePage() {
  const params = useParams();
  const rawId = params.id;
  const quoteId = typeof rawId === "string" && QUOTE_ID.test(rawId) ? rawId : null;
  const [quote, setQuote] = useState<QuoteResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [accepting, setAccepting] = useState(false);
  const [accepted, setAccepted] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadQuote = useCallback(async (signal?: AbortSignal) => {
    setLoading(true);
    setError(null);
    try {
      if (quoteId === null) throw new Error("invalid quote");
      const response = await fetch(`/api/v1/quotes/${encodeURIComponent(quoteId)}`, {
        cache: "no-store",
        signal,
      });
      const parsed = parseQuote(await response.json());
      if (!response.ok || parsed === null) throw new Error("quote unavailable");
      setQuote(parsed);
      setAccepted(parsed.quote.status === "ACCEPTED");
    } catch (loadError) {
      if (loadError instanceof DOMException && loadError.name === "AbortError") return;
      setQuote(null);
      setError("This quote is unavailable.");
    } finally {
      setLoading(false);
    }
  }, [quoteId]);

  useEffect(() => {
    const controller = new AbortController();
    void loadQuote(controller.signal);
    return () => controller.abort();
  }, [loadQuote]);

  async function acceptQuote() {
    if (quoteId === null || accepting) return;
    setAccepting(true);
    setError(null);
    try {
      const response = await fetch(`/api/v1/quotes/${encodeURIComponent(quoteId)}/accept`, {
        method: "POST",
        headers: { "content-type": "application/json" },
      });
      if (!response.ok) throw new Error("acceptance rejected");
      setAccepted(true);
    } catch {
      setError("The quote could not be accepted. Try again.");
    } finally {
      setAccepting(false);
    }
  }

  if (loading) return <p className="py-10 text-sm text-gray-600" role="status">Loading quote...</p>;

  if (quote === null) {
    return (
      <div className="app-panel max-w-lg rounded-lg p-5">
        <p className="text-sm text-red-700 dark:text-red-300" role="alert">{error}</p>
        <button className="app-button mt-4 inline-flex items-center gap-2" onClick={() => void loadQuote()} type="button">
          <RefreshCw aria-hidden="true" className="h-4 w-4" />
          Retry
        </button>
      </div>
    );
  }

  if (accepted) {
    return (
      <section className="app-panel max-w-lg rounded-lg p-6 text-center" role="status">
        <CheckCircle2 aria-hidden="true" className="mx-auto h-10 w-10 text-green-600" />
        <h2 className="mt-3 text-xl font-semibold">Quote accepted</h2>
        <p className="mt-2 text-sm text-gray-600 dark:text-gray-300">
          The business has been notified and can continue scheduling the work.
        </p>
      </section>
    );
  }

  return (
    <div className="max-w-2xl space-y-5">
      <section className="app-panel rounded-lg p-5">
        <div className="flex flex-wrap items-start justify-between gap-4">
          <div>
            <p className="text-xs font-semibold uppercase text-gray-500">Quote</p>
            <p className="mt-1 font-mono text-sm text-gray-600 dark:text-gray-300">{quote.quote.id}</p>
          </div>
          <p className="text-2xl font-semibold">{formatMoney(quote.quote.total_amount_cents)}</p>
        </div>
        <div className="mt-5 divide-y divide-gray-200 dark:divide-gray-700">
          {quote.line_items.map((item) => (
            <div className="flex items-start justify-between gap-4 py-3 text-sm" key={item.id}>
              <span>{item.description} x{item.quantity}</span>
              <span className="font-medium">{formatMoney(item.unit_price_cents * item.quantity)}</span>
            </div>
          ))}
        </div>
        <div className="mt-4 flex items-center justify-between rounded-lg bg-blue-50 p-4 text-sm text-blue-950 dark:bg-blue-950/30 dark:text-blue-100">
          <span>Required deposit</span>
          <strong>{formatMoney(quote.quote.required_deposit_cents)}</strong>
        </div>
      </section>

      {error && <p className="text-sm text-red-700 dark:text-red-300" role="alert">{error}</p>}
      <button
        className="app-button min-h-11 bg-[#0066FF] px-5 py-2.5 font-semibold text-white disabled:opacity-60"
        disabled={accepting}
        onClick={() => void acceptQuote()}
        type="button"
      >
        {accepting ? "Accepting..." : "Accept quote"}
      </button>
    </div>
  );
}
