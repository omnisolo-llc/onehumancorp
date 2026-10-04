"use client";

import { useEffect, useRef, useState } from "react";
import { useParams } from "next/navigation";
import { AppShell } from "../../components/AppShell";

type Order = { id: string; customer_name?: string; total_amount?: number; status?: string; created_at?: string };
type ShippingRate = { id: string; carrier: string; service: string; amount: number; days?: number };
type ShippingLabel = { url: string; trackingNumber?: string; carrier?: string };

const isRecord = (value: unknown): value is Record<string, unknown> => Boolean(value) && typeof value === "object" && !Array.isArray(value);
const nonEmptyString = (value: unknown) => typeof value === "string" && value.trim() ? value.trim() : undefined;

function parseOrder(value: unknown): Order | null {
  if (!isRecord(value)) return null;
  const id = nonEmptyString(value.id);
  if (!id) return null;
  const total = typeof value.total_amount === "number" && Number.isFinite(value.total_amount) && value.total_amount >= 0
    ? value.total_amount : undefined;
  return {
    id,
    customer_name: nonEmptyString(value.customer_name),
    total_amount: total,
    status: nonEmptyString(value.status),
    created_at: nonEmptyString(value.created_at),
  };
}

function parseRates(value: unknown): ShippingRate[] | null {
  if (!isRecord(value) || !Array.isArray(value.rates) || value.rates.length === 0) return null;
  const rates: ShippingRate[] = [];
  for (const candidate of value.rates) {
    if (!isRecord(candidate)) return null;
    const id = nonEmptyString(candidate.id);
    const carrier = nonEmptyString(candidate.carrier);
    const service = nonEmptyString(candidate.service);
    const rawAmount = nonEmptyString(candidate.amount);
    const amount = rawAmount && /^\d+(?:\.\d{1,2})?$/.test(rawAmount) ? Number(rawAmount) : Number.NaN;
    const days = candidate.days;
    if (!id || !carrier || !service || !Number.isFinite(amount) || amount < 0) return null;
    if (days !== undefined && (typeof days !== "number" || !Number.isInteger(days) || days < 0)) return null;
    rates.push({ id, carrier, service, amount, days: days as number | undefined });
  }
  return rates;
}

function parseLabel(value: unknown): ShippingLabel | null {
  if (!isRecord(value) || value.success !== true) return null;
  const rawUrl = nonEmptyString(value.labelUrl);
  const trackingNumber = nonEmptyString(value.trackingNumber);
  const carrier = nonEmptyString(value.carrier);
  if (!rawUrl) return null;
  try {
    const url = new URL(rawUrl);
    const trustedShippoHost = url.hostname === "goshippo.com"
      || url.hostname.endsWith(".goshippo.com")
      || [
        "shippo-delivery.s3.amazonaws.com",
        "shippo-delivery-east.s3.amazonaws.com",
        "shippo-delivery-west.s3.amazonaws.com",
      ].includes(url.hostname);
    if (url.protocol !== "https:" || !trustedShippoHost || url.username || url.password) return null;
    return { url: url.toString(), trackingNumber, carrier };
  } catch {
    return null;
  }
}

export default function OrderDetailsPage() {
  const params = useParams();
  const orderId = String(params.id || "");
  const [order, setOrder] = useState<Order | null>(null);
  const [status, setStatus] = useState<"loading" | "ready" | "missing" | "error">("loading");
  const [weight, setWeight] = useState("");
  const [dimensions, setDimensions] = useState("");
  const [rates, setRates] = useState<ShippingRate[]>([]);
  const [selectedRate, setSelectedRate] = useState("");
  const [shippingError, setShippingError] = useState("");
  const [shippingPending, setShippingPending] = useState(false);
  const [label, setLabel] = useState<ShippingLabel | null>(null);
  const [reconciliationRequired, setReconciliationRequired] = useState(false);
  const purchaseInFlight = useRef(false);
  const orderGeneration = useRef(0);

  useEffect(() => {
    orderGeneration.current += 1;
    const controller = new AbortController();
    let current = true;
    setOrder(null);
    setStatus("loading");
    setRates([]);
    setSelectedRate("");
    setLabel(null);
    setShippingError("");
    setReconciliationRequired(false);
    setShippingPending(false);
    purchaseInFlight.current = false;
    fetch(`/api/v1/ui/orders/${encodeURIComponent(orderId)}`, { cache: "no-store", signal: controller.signal })
      .then(async (res) => {
        if (res.status === 404) {
          if (current) setStatus("missing");
          return;
        }
        if (!res.ok) throw new Error();
        const record = parseOrder(await res.json());
        if (!record || record.id !== orderId) throw new Error();
        if (current) {
          setOrder(record);
          setStatus("ready");
        }
      })
      .catch(() => { if (current) setStatus("error"); });
    return () => { current = false; controller.abort(); orderGeneration.current += 1; };
  }, [orderId]);

  const fetchRates = async () => {
    if (label || reconciliationRequired || purchaseInFlight.current) return;
    setShippingError("");
    setRates([]);
    setSelectedRate("");

    const weightNumber = Number(weight);
    if (!Number.isFinite(weightNumber) || weightNumber <= 0 || !/^\d+(?:\.\d+)?x\d+(?:\.\d+)?x\d+(?:\.\d+)?$/i.test(dimensions.trim())) {
      setShippingError("Enter a valid positive weight and dimensions such as 10x8x6.");
      return;
    }
    setShippingPending(true);
    const generation = orderGeneration.current;
    try {
      const response = await fetch("/api/v1/shipping/rates", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ orderId, weight: weight.trim(), dimensions: dimensions.trim().toLowerCase() }),
      });
      const body: unknown = await response.json();
      if (response.ok) {
        const parsed = parseRates(body);
        if (generation !== orderGeneration.current) return;
        if (parsed && parsed.length > 0) {
          setRates(parsed);
          setSelectedRate(parsed[0].id);
          return;
        }
      }
      throw new Error();
    } catch {
      if (generation === orderGeneration.current) setShippingError("Shipping rates are unavailable.");
    } finally {
      if (generation === orderGeneration.current) setShippingPending(false);
    }
  };

  const buyLabel = async () => {
    if (!selectedRate || label || reconciliationRequired || purchaseInFlight.current) return;
    purchaseInFlight.current = true;
    setReconciliationRequired(true);
    setShippingError("");
    setShippingPending(true);
    const generation = orderGeneration.current;
    try {
      const response = await fetch("/api/v1/shipping/label", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ orderId, rateId: selectedRate }),
      });
      const body: unknown = await response.json();
      if (generation !== orderGeneration.current) return;
      if (response.ok) {
        const parsed = parseLabel(body);
        if (parsed) {
          setLabel(parsed);
          setReconciliationRequired(false);
          return;
        }
      }
      setShippingError(isRecord(body) && body.reconciliationRequired === true
        ? nonEmptyString(body.error) || "The purchase outcome is unknown. Reconcile before retrying."
        : "The shipping label could not be confirmed.");
    } catch {
      if (generation === orderGeneration.current) setShippingError("The shipping label could not be confirmed.");
    } finally {
      if (generation === orderGeneration.current) {
        setShippingPending(false);
        purchaseInFlight.current = false;
      }
    }
  };

  return (
    <AppShell
      title={order ? `Order ${order.id}` : "Order details"}
      subtitle="Database-backed fulfillment and shipping details."
      statusItems={[
        { label: "Status", value: status === "ready" ? order?.status || "Unknown" : status },
      ]}
      actions={[{ label: "All orders", href: "/orders" }]}
    >
      <div className="mx-auto max-w-3xl space-y-6">
        {status === "loading" && <p className="text-sm text-gray-500">Loading order…</p>}
        {status === "error" && <p className="text-sm text-red-600" role="alert">Order data is unavailable.</p>}
        {status === "missing" && <p className="text-sm text-gray-600">This order was not found.</p>}
        {status === "ready" && order && (
          <>
            <section className="app-card rounded-2xl border border-gray-200 bg-white/70 p-6 shadow-sm">
              <h2 className="text-xl font-bold font-outfit text-gray-900">Order Summary</h2>
              <dl className="mt-5 grid gap-4 sm:grid-cols-2">
                <Field label="Order ID" value={order.id} />
                <Field label="Status" value={order.status || "Unavailable"} />
                <Field label="Customer" value={order.customer_name || "Unavailable"} />
                <Field label="Created" value={order.created_at || "Unavailable"} />
                <Field label="Recorded total" value={typeof order.total_amount === "number" ? order.total_amount.toLocaleString(undefined, { style: "currency", currency: "USD" }) : "Unavailable"} />
              </dl>
            </section>
            <section className="app-card rounded-2xl border border-gray-200 bg-white/70 p-6 shadow-sm">
              <div className="flex items-center justify-between">
                <h2 className="text-xl font-bold font-outfit text-gray-900">Fulfillment</h2>
                <span className="text-xs font-semibold px-2 py-1 rounded bg-indigo-50 text-indigo-700 border border-indigo-200">Powered by Shippo</span>
              </div>
              <div className="mt-4 grid gap-3 sm:grid-cols-2">
                <label className="text-sm font-medium">Weight (oz)<input aria-label="Package weight in ounces" type="number" value={weight} onChange={(event) => setWeight(event.target.value)} className="mt-1 w-full rounded-lg border p-2" /></label>
                <label className="text-sm font-medium">Dimensions<input aria-label="Package dimensions" placeholder="e.g. 10x8x6" value={dimensions} onChange={(event) => setDimensions(event.target.value)} className="mt-1 w-full rounded-lg border p-2" /></label>
              </div>
              <button onClick={fetchRates} disabled={shippingPending || reconciliationRequired || Boolean(label)} className="mt-4 rounded-lg bg-gray-900 px-4 py-2 text-white">Get Shipping Rates</button>
              {shippingError && <p className="mt-3 text-sm text-red-600" role="alert">{shippingError}</p>}
              {reconciliationRequired && !shippingPending && <p className="mt-2 text-sm text-amber-800">Do not purchase another label until this provider outcome is reconciled. Reloading does not establish whether a purchase occurred.</p>}
              {rates.length > 0 && (
                <div className="mt-4 space-y-2">
                  <h3 className="text-sm font-semibold text-gray-700">Select a Service</h3>
                  {rates.map((rate) => (
                    <label key={rate.id} className="flex items-center justify-between rounded-lg border p-3 cursor-pointer">
                      <span><input type="radio" name="shipping_rate" value={rate.id} checked={selectedRate === rate.id} onChange={() => setSelectedRate(rate.id)} /> <span className="ml-2">{rate.carrier} {rate.service}</span>{typeof rate.days === "number" ? ` · ${rate.days} days` : ""}</span>
                      <span>${rate.amount.toFixed(2)}</span>
                    </label>
                  ))}
                  <button onClick={buyLabel} disabled={!selectedRate || shippingPending || reconciliationRequired || Boolean(label)} className="rounded-lg bg-indigo-600 px-4 py-2 text-white disabled:opacity-50">
                    Buy Label
                  </button>
                </div>
              )}
              {label && (
                <div className="mt-4 rounded-lg border border-green-200 bg-green-50 p-4 space-y-2">
                  <p className="font-semibold text-green-800">Label Purchased Successfully</p>
                  <p className="text-sm text-gray-700">{label.carrier ? `${label.carrier} tracking` : "Tracking"}: <strong>{label.trackingNumber || "Not available yet"}</strong></p>
                  <div className="flex items-center gap-3">
                    <a href={label.url} target="_blank" rel="noopener noreferrer" className="text-indigo-700 underline font-medium">
                      Open Shipping Label
                    </a>
                    <span className="text-xs font-semibold px-2 py-0.5 rounded bg-green-100 text-green-800">Label created</span>
                  </div>
                </div>
              )}
            </section>
          </>
        )}
      </div>
    </AppShell>
  );
}

function Field({ label, value }: { label: string; value: string }) {
  return <div><dt className="text-xs font-bold uppercase tracking-wide text-gray-500">{label}</dt><dd className="mt-1 text-sm font-medium text-gray-900">{value}</dd></div>;
}
