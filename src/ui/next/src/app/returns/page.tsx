"use client";

import { useState } from "react";
import { AppShell } from "../components/AppShell";

export default function InitiateReturnPage() {
  const [orderId, setOrderId] = useState("");
  const [productId, setProductId] = useState("");
  const [amountCents, setAmountCents] = useState(0);
  const [status, setStatus] = useState<"idle" | "submitting" | "success" | "error">("idle");
  const [errorMsg, setErrorMsg] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setStatus("submitting");
    setErrorMsg("");

    try {
      const res = await fetch("/api/v1/returns/initiate", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          order_id: orderId,
          product_id: productId,
          amount_cents: amountCents,
        }),
      });

      if (!res.ok) {
        throw new Error("Failed to initiate return");
      }

      setStatus("success");
      setOrderId("");
      setProductId("");
      setAmountCents(0);
    } catch (e: any) {
      setStatus("error");
      setErrorMsg(e.message || "An error occurred");
    }
  };

  return (
    <AppShell>
      <div className="p-4 sm:p-8 max-w-lg mx-auto">
        <div className="glassmorphism p-6 rounded-2xl shadow-sm border border-white/20 relative overflow-hidden">
          <div className="absolute inset-0 bg-gradient-to-br from-white/40 to-white/10 z-0 pointer-events-none"></div>
          <div className="relative z-10">
            <h1 className="text-2xl font-bold tracking-tight text-gray-900 mb-2">Request a Return</h1>
            <p className="text-sm text-gray-600 mb-6">Need to return an item? Start the process here.</p>

            {status === "success" && (
              <div className="mb-6 p-4 rounded-xl bg-green-50/80 backdrop-blur-md border border-green-200 text-green-800 text-sm flex items-start gap-3">
                <span className="text-xl">✅</span>
                <div>
                  <strong className="font-semibold block mb-1">Return Requested!</strong>
                  Our team has been notified and will review your request shortly.
                </div>
              </div>
            )}

            {status === "error" && (
              <div className="mb-6 p-4 rounded-xl bg-red-50/80 backdrop-blur-md border border-red-200 text-red-800 text-sm">
                ⚠️ {errorMsg}
              </div>
            )}

            <form onSubmit={handleSubmit} className="space-y-4">
              <div>
                <label className="block text-sm font-semibold text-gray-800 mb-1">Order ID</label>
                <input
                  type="text"
                  required
                  value={orderId}
                  onChange={(e) => setOrderId(e.target.value)}
                  placeholder="e.g. ORD-12345"
                  className="w-full px-4 py-3 rounded-xl border border-gray-200 bg-white/70 backdrop-blur-sm focus:bg-white focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all outline-none"
                />
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-800 mb-1">Product ID</label>
                <input
                  type="text"
                  required
                  value={productId}
                  onChange={(e) => setProductId(e.target.value)}
                  placeholder="e.g. PROD-987"
                  className="w-full px-4 py-3 rounded-xl border border-gray-200 bg-white/70 backdrop-blur-sm focus:bg-white focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all outline-none"
                />
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-800 mb-1">Refund Amount (cents)</label>
                <input
                  type="number"
                  required
                  min="1"
                  value={amountCents || ""}
                  onChange={(e) => setAmountCents(parseInt(e.target.value) || 0)}
                  placeholder="e.g. 4500 for $45.00"
                  className="w-full px-4 py-3 rounded-xl border border-gray-200 bg-white/70 backdrop-blur-sm focus:bg-white focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all outline-none"
                />
              </div>

              <button
                type="submit"
                disabled={status === "submitting"}
                className="w-full min-h-[44px] mt-2 py-3 px-4 rounded-xl bg-blue-600 hover:bg-blue-700 text-white font-semibold shadow-md shadow-blue-500/20 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {status === "submitting" ? "Submitting..." : "Submit Return Request"}
              </button>
            </form>
          </div>
        </div>
      </div>
    </AppShell>
  );
}
