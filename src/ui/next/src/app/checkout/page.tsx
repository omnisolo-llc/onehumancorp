"use client";

import { Suspense, useEffect, useMemo, useState } from "react";
import { useRouter, useSearchParams } from "next/navigation";
import { WithTooltip } from "../../components/TooltipRegistry";

type CatalogProduct = {
  id: string;
  title: string;
  price_cents: number;
};

type PaidOrder = { id: string; status: string };

const SAFE_ID = /^[A-Za-z0-9._-]{1,128}$/;
const PAID_STATUSES = new Set(["paid", "completed", "confirmed"]);
const PLAN_TIERS = new Set(["starter", "pro", "business"]);

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function parseProduct(value: unknown): CatalogProduct | null {
  if (!isRecord(value)) return null;
  if (!SAFE_ID.test(String(value.id ?? ""))) return null;
  if (typeof value.title !== "string" || !value.title.trim()) return null;
  if (!Number.isSafeInteger(value.price_cents) || Number(value.price_cents) < 0) return null;
  return { id: String(value.id), title: value.title.trim(), price_cents: Number(value.price_cents) };
}

function parsePaidOrder(value: unknown, orderId: string): PaidOrder | null {
  if (!Array.isArray(value)) return null;
  for (const candidate of value) {
    if (!isRecord(candidate) || candidate.id !== orderId || typeof candidate.status !== "string") continue;
    const status = candidate.status.trim().toLowerCase();
    if (PAID_STATUSES.has(status)) return { id: orderId, status };
  }
  return null;
}

function trustedCheckoutUrl(value: unknown): string | null {
  if (typeof value !== "string") return null;
  try {
    const url = new URL(value);
    if (
      url.protocol !== "https:" ||
      url.hostname !== "checkout.stripe.com" ||
      url.username ||
      url.password
    ) return null;
    return url.toString();
  } catch {
    return null;
  }
}

function CheckoutContent() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const rawTier = searchParams?.get("tier")?.trim() ?? "";
  const tier = PLAN_TIERS.has(rawTier.toLowerCase()) ? rawTier : "";
  const productId = searchParams?.get("product_id")?.trim() ?? "";
  const rawQuantity = searchParams?.get("quantity") ?? "1";
  const quantity = /^\d{1,3}$/.test(rawQuantity) ? Number(rawQuantity) : 0;
  const successRequested = searchParams?.get("success") === "true";
  const orderId = searchParams?.get("orderId")?.trim() ?? "";

  const [product, setProduct] = useState<CatalogProduct | null>(null);
  const [paidOrder, setPaidOrder] = useState<PaidOrder | null>(null);
  const [pageStatus, setPageStatus] = useState<"loading" | "ready" | "error">("loading");
  const [pageError, setPageError] = useState("");
  const [checkoutStatus, setCheckoutStatus] = useState("");
  const [isProcessing, setIsProcessing] = useState(false);

  const validProductRequest = SAFE_ID.test(productId) && quantity >= 1 && quantity <= 100;
  const validOrderRequest = SAFE_ID.test(orderId);

  useEffect(() => {
    let active = true;
    async function load() {
      setPageStatus("loading");
      setPageError("");
      try {
        if (successRequested) {
          if (!validOrderRequest) throw new Error("Payment has not been confirmed for a valid order.");
          const response = await fetch("/api/v1/ui/orders");
          if (!response.ok) throw new Error("Payment confirmation is unavailable.");
          const verified = parsePaidOrder(await response.json(), orderId);
          if (!verified) throw new Error("Payment has not been confirmed for this order.");
          if (active) setPaidOrder(verified);
        } else if (tier) {
          // The authenticated billing endpoint owns plan pricing and eligibility.
        } else {
          if (!validProductRequest) throw new Error("A valid product is required to start checkout.");
          const response = await fetch("/api/v1/catalog/products");
          if (!response.ok) throw new Error("Product details are unavailable.");
          const payload: unknown = await response.json();
          if (!Array.isArray(payload)) throw new Error("Product details are unavailable.");
          const selected = payload.map(parseProduct).find((item) => item?.id === productId) ?? null;
          if (!selected) throw new Error("The selected product is unavailable.");
          if (active) setProduct(selected);
        }
        if (active) setPageStatus("ready");
      } catch (error) {
        if (active) {
          setPageError(error instanceof Error ? error.message : "Checkout is unavailable.");
          setPageStatus("error");
        }
      }
    }
    void load();
    return () => { active = false; };
  }, [orderId, productId, quantity, successRequested, tier, validOrderRequest, validProductRequest]);

  const total = useMemo(() => product ? product.price_cents * quantity : null, [product, quantity]);

  async function handlePayment() {
    if (!tier && !product) return;
    setIsProcessing(true);
    setCheckoutStatus("Preparing checkout…");
    try {
      const response = await fetch("/api/v1/billing/create-checkout-session", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(tier ? {
          tier,
          is_subscription: true,
        } : {
          is_subscription: false,
          product_id: product?.id,
          quantity,
        }),
      });
      if (response.status === 409) throw new Error("The selected product just sold out.");
      const payload: unknown = await response.json();
      const url = isRecord(payload) ? trustedCheckoutUrl(payload.checkout_url) : null;
      if (!response.ok || !url) throw new Error("Checkout is temporarily unavailable.");
      setCheckoutStatus("Redirecting to secure checkout…");
      window.location.assign(url);
    } catch (error) {
      setCheckoutStatus(error instanceof Error ? error.message : "Checkout is temporarily unavailable.");
      setIsProcessing(false);
    }
  }

  const heading = paidOrder ? "Order Successful" : tier ? "Plan Upgrade" : "Secure Checkout";

  return (
    <div className="flex min-h-screen flex-col overflow-x-hidden bg-[#F8F9FA] text-gray-900">
      <header className="app-panel-header sticky top-0 z-50 flex items-center justify-between px-4 py-4 shadow-sm md:px-6 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border-b border-white/40 dark:border-white/10">
        <WithTooltip id="checkout-title-tooltip" defaultText="Review verified checkout details before paying.">
          <h1 className="font-outfit text-2xl font-bold">{heading}</h1>
        </WithTooltip>
      </header>
      <main id="checkout-screen" className="mx-auto flex w-full max-w-lg flex-1 flex-col justify-center p-4 md:p-8">
        {pageStatus === "loading" && <p className="text-sm text-gray-600">Loading verified checkout details…</p>}
        {pageStatus === "error" && <p className="rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-700" role="alert">{pageError}</p>}
        {pageStatus === "ready" && paidOrder && (
          <section className="app-card rounded-3xl border border-green-200/40 dark:border-green-800/40 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] p-8 text-center shadow-lg">
            <h2 className="font-outfit text-2xl font-bold">Payment confirmed</h2>
            <p className="mt-3 text-gray-600">Order {paidOrder.id} has a confirmed payment.</p>
            <button onClick={() => router.push("/orders")} className="mt-6 w-full rounded-lg bg-gray-900 px-4 py-3 font-medium text-white">View Orders</button>
          </section>
        )}
        {pageStatus === "ready" && !paidOrder && (tier || product) && (
          <section className="app-card rounded-3xl border border-white/40 dark:border-white/10 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] p-6 shadow-lg md:p-8">
            <div className="flex items-center justify-between border-b border-gray-200 pb-5">
              <div>
                <h2 className="font-outfit text-xl font-bold">{tier ? `OHC ${tier} Plan` : product?.title}</h2>
                <p className="mt-1 text-sm text-gray-600">{tier ? "Final pricing and eligibility are confirmed by billing." : `Quantity: ${quantity}`}</p>
              </div>
              {total !== null && <span className="font-outfit text-xl font-bold">${(total / 100).toFixed(2)}</span>}
            </div>
            <p className="mt-5 text-sm text-gray-600">Taxes, fees, and any verified discounts are calculated by the payment provider.</p>
            {checkoutStatus && <p className="mt-4 text-sm font-medium text-indigo-700" role="status">{checkoutStatus}</p>}
            <button onClick={handlePayment} disabled={isProcessing} className="mt-6 w-full rounded-lg bg-black px-4 py-3 font-medium text-white disabled:opacity-50">
              {isProcessing ? "Processing…" : tier ? "Upgrade" : "Pay"}
            </button>
            <button onClick={() => router.push(tier ? "/pricing" : "/products")} className="mt-3 w-full rounded-lg bg-gray-200 px-4 py-3 font-medium text-gray-800">Cancel</button>
          </section>
        )}
      </main>
    </div>
  );
}

export default function CheckoutPage() {
  return <Suspense fallback={<div>Loading checkout…</div>}><CheckoutContent /></Suspense>;
}
