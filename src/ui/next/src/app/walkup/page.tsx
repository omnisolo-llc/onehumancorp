"use client";

import { useState } from "react";
import Link from "next/link";

function tenantId() {
  if (typeof window === "undefined") return "default";
  const urlParams = new URLSearchParams(window.location.search);
  const urlTenant = urlParams.get("tenant_id");
  if (urlTenant) return urlTenant;
  return (
    localStorage.getItem("tenant_id") ||
    localStorage.getItem("tenant") ||
    "default"
  );
}

export default function WalkupPage() {
  const [transcript, setTranscript] = useState("");
  const [isProcessing, setIsProcessing] = useState(false);
  const [result, setResult] = useState<any>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!transcript) return;

    setIsProcessing(true);
    setResult(null);

    try {
      const res = await fetch("/api/v1/webhooks/omni_intake", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tenant_id: tenantId(),
          transcript,
        }),
      });

      if (!res.ok) throw new Error("Failed to process intake");
      const data = await res.json();
      setResult(data.parsed);
    } catch (e: any) {
      console.error(e);
      alert("Error processing transcript");
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="relative min-h-screen bg-gray-50 flex items-center justify-center p-4">
      {/* Processing Overlay */}
      {isProcessing && (
        <div className="absolute inset-0 z-50 flex items-center justify-center" style={{
          background: 'rgba(255, 255, 255, 0.65)',
          backdropFilter: 'blur(30px) saturate(210%)',
        }}>
          <div className="animate-pulse flex flex-col items-center">
            <div className="w-12 h-12 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
            <p className="mt-4 text-gray-700 font-medium tracking-wide">Processing Order...</p>
          </div>
        </div>
      )}

      <div className="w-full max-w-md">
        {!result ? (
          <div className="bg-white rounded-[16px] shadow-sm p-6" style={{
            background: 'rgba(255, 255, 255, 0.65)',
            backdropFilter: 'blur(30px) saturate(210%)',
            border: '1px solid rgba(255, 255, 255, 0.4)',
          }}>
            <div className="text-center mb-8">
              <div className="w-16 h-16 bg-blue-50 rounded-full flex items-center justify-center mx-auto mb-4 text-3xl">
                🎙️
              </div>
              <h1 className="text-2xl font-semibold text-gray-900 font-outfit">Walk-up Order</h1>
              <p className="text-gray-500 mt-2">Enter or speak the customer's order</p>
            </div>

            <form onSubmit={handleSubmit} className="space-y-4">
              <textarea
                className="w-full p-4 border border-gray-200 rounded-[8px] focus:ring-2 focus:ring-blue-500 outline-none resize-none bg-white/50"
                rows={4}
                placeholder="e.g. Quiero 3 tacos de pollo..."
                value={transcript}
                onChange={(e) => setTranscript(e.target.value)}
                disabled={isProcessing}
                autoFocus
              />

              <button
                type="submit"
                disabled={!transcript || isProcessing}
                className="w-full bg-[#0066FF] hover:bg-blue-600 text-white font-medium py-4 rounded-[8px] transition-colors disabled:opacity-50"
              >
                Listen & Translate
              </button>
            </form>
          </div>
        ) : (
          <div className="bg-white rounded-[16px] shadow-sm p-6" style={{
            background: 'rgba(255, 255, 255, 0.65)',
            backdropFilter: 'blur(30px) saturate(210%)',
            border: '1px solid rgba(255, 255, 255, 0.4)',
          }}>
            <div className="flex items-center justify-between mb-6 border-b border-gray-100 pb-4">
              <h2 className="text-xl font-semibold text-gray-900 font-outfit">
                {result.intent === "Order" ? "Order Captured" : "Inquiry Captured"}
              </h2>
              <span className="bg-[#34C759]/10 text-[#34C759] px-3 py-1 rounded-full text-sm font-medium">
                {result.intent}
              </span>
            </div>

            <div className="space-y-3 mb-8">
              {result.items && result.items.map((item: any, i: number) => (
                <div key={i} className="flex items-center justify-between p-3 bg-gray-50/50 rounded-[8px]">
                  <span className="font-medium text-gray-900">{item.name}</span>
                  <span className="text-gray-500 bg-gray-100 px-2 py-1 rounded text-sm">x{item.quantity}</span>
                </div>
              ))}
            </div>

            <div className="flex space-x-3">
              <button
                onClick={() => { setTranscript(""); setResult(null); }}
                className="flex-1 bg-gray-100 hover:bg-gray-200 text-gray-700 font-medium py-3 rounded-[8px] transition-colors"
              >
                New Order
              </button>
              <Link
                href={`/triage?tenant_id=${tenantId()}`}
                className="flex-1 bg-[#0066FF] hover:bg-blue-600 text-white font-medium py-3 rounded-[8px] transition-colors text-center"
              >
                Confirm & Go to List
              </Link>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
