"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";

export default function WalkupOrderPage() {
  const router = useRouter();
  const [inputText, setInputText] = useState("");
  const [isProcessing, setIsProcessing] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [structuredOrder, setStructuredOrder] = useState("");

  const tenantId = typeof window !== "undefined" ? localStorage.getItem("tenant_id") || "default" : "default";

  const handleListenOrSubmit = async () => {
    if (!inputText.trim()) return;

    setIsProcessing(true);
    try {
      const response = await fetch("/api/ui/walkup", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-tenant-id": tenantId,
        },
        body: JSON.stringify({
          tenant_id: tenantId,
          message: inputText
        }),
      });

      if (response.ok) {
        const data = await response.json();
        if (data.structured_order) {
            setStructuredOrder(data.structured_order);
        }
        setIsSuccess(true);
        setTimeout(() => {
          setIsSuccess(false);
          setInputText("");
          router.push("/pos/kds");
        }, 3000);
      } else {
        console.error("Submission failed");
      }
    } catch (e) {
      console.error(e);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10 relative">
      <div className="w-[375px] max-w-[375px] mx-auto h-[812px] bg-gradient-to-br from-white/40 to-white/10 backdrop-blur-[30px] saturate-[210%] shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-4 px-6 bg-white/65 backdrop-blur-[30px] shadow-sm border-b border-gray-200 sticky top-0 z-10 flex justify-center items-center">
          <h1 className="text-xl font-bold text-gray-900">Walk-up Order</h1>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-6 py-8 flex flex-col">
          <p className="text-gray-600 mb-6 text-center font-medium">
            Enter or say the customer's request in their language. OHC will translate it and structure the order automatically.
          </p>

          <textarea
            value={inputText}
            onChange={(e) => setInputText(e.target.value)}
            className="w-full h-40 p-4 border border-gray-300 rounded-2xl shadow-sm resize-none focus:ring-2 focus:ring-blue-500 focus:outline-none text-lg bg-white/80"
            placeholder="e.g., Quiero 3 tacos de pollo..."
            disabled={isProcessing}
            data-testid="input-walkup-text"
          />

          <button
            onClick={handleListenOrSubmit}
            disabled={!inputText.trim() || isProcessing}
            className={`mt-6 w-full py-4 text-white font-bold text-lg rounded-2xl shadow-lg transition transform ${
              !inputText.trim() || isProcessing ? "bg-gray-400 cursor-not-allowed" : "bg-blue-600 active:scale-95"
            }`}
            data-testid="btn-submit-walkup"
          >
            {isProcessing ? "Processing..." : "Process Request"}
          </button>
        </div>

        {/* Processing Overlay */}
        {isProcessing && (
          <div className="absolute inset-0 bg-white/50 backdrop-blur-md z-20 flex flex-col items-center justify-center">
            <div className="w-12 h-12 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
            <p className="mt-4 text-blue-800 font-bold text-lg" data-testid="processing-overlay">Listening & Translating...</p>
          </div>
        )}

        {/* Success Card Overlay */}
        {isSuccess && (
          <div className="absolute inset-0 bg-black/40 backdrop-blur-md z-30 flex flex-col items-center justify-center p-6">
            <div className="bg-white rounded-3xl p-8 w-full shadow-2xl flex flex-col items-center text-center">
              <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mb-4">
                <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="3" d="M5 13l4 4L19 7"></path></svg>
              </div>
              <h2 className="text-2xl font-bold text-gray-900 mb-2">Order Structured!</h2>
              <p className="text-gray-600 font-medium mb-4" data-testid="success-card">Added to list.</p>
              {structuredOrder && (
                  <div className="p-3 bg-gray-50 border border-gray-200 rounded-xl w-full text-left">
                      <p className="text-sm text-gray-500 mb-1">Translated & Structured:</p>
                      <p className="text-lg font-bold text-gray-900" data-testid="structured-order-text">{structuredOrder}</p>
                  </div>
              )}
            </div>
          </div>
        )}

      </div>
    </div>
  );
}
