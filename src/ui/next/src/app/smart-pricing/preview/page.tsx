"use client";

import React, { useState } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";

export default function SmartPricingPreviewPage() {
  const router = useRouter();
  const [loading, setLoading] = useState(false);
  const [saleActive, setSaleActive] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleApprove = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch("/api/agents/approvals/simulate-smart-pricing", {
        method: "POST",
        headers: {
          Authorization: `Bearer ${localStorage.getItem("token") || ""}`,
          "Content-Type": "application/json",
        },
      });
      if (res.ok) {
        setSaleActive(true);
      } else {
        setError("Failed to apply dynamic pricing");
      }
    } catch (e) {
      setError("Network error applying dynamic pricing");
    } finally {
      setLoading(false);
    }
  };

  if (saleActive) {
    return (
      <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7] p-6">
        <main className="flex-1 max-w-lg mx-auto w-full flex flex-col gap-6 mt-12">
          <div className="bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:border-white/10 shadow-sm p-6 rounded-[12px] text-center">
            <h2 className="text-2xl font-bold text-[#1D1D1F] dark:text-white mb-4">Sale Active! 🎉</h2>
            <p className="text-[#86868B] mb-6">
              The 20% discount on slow-moving items is live. We've notified 150 past customers.
            </p>
            <div className="w-full bg-gray-200 rounded-full h-2.5 mb-6 dark:bg-gray-700">
              <div className="bg-blue-600 h-2.5 rounded-full" style={{ width: "0%" }}></div>
            </div>
            <p className="text-sm text-gray-500 mb-6">0 / 3 items sold so far</p>
            <Link href="/dashboard" className="px-4 py-2 bg-gray-200 text-[#1D1D1F] rounded-[8px] font-medium hover:bg-gray-300 transition-colors">
              Return to Dashboard
            </Link>
          </div>
        </main>
      </div>
    );
  }

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-6 py-4 flex items-center justify-between border-b border-white/40 dark:border-white/10 sticky top-0 z-50 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[2.1]">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-[-0.02em] dark:text-white">
          Review Proposed Sale
        </h1>
        <div className="flex items-center min-h-[44px]">
          <button
            onClick={() => router.push("/dashboard")}
            className="px-4 py-2 bg-gray-200 text-[#1D1D1F] text-sm font-medium hover:bg-gray-300 transition-colors rounded-[8px]"
          >
            Cancel
          </button>
        </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col gap-6">
        {error && (
          <div className="p-4 bg-red-100 text-red-700 rounded-[12px]">
            {error}
          </div>
        )}
        <div className="bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:border-white/10 shadow-sm p-6 rounded-[12px]">
          <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-white mb-4">
            Clear Stagnant Inventory
          </h2>

          <div className="flex flex-col gap-4 mb-6">
            <div className="flex justify-between items-center pb-2 border-b border-gray-100 dark:border-gray-800">
              <span className="font-medium text-[#1D1D1F] dark:text-gray-300">Items</span>
              <span className="text-[#86868B]">Red Summer Dress, Summer Hats</span>
            </div>
            <div className="flex justify-between items-center pb-2 border-b border-gray-100 dark:border-gray-800">
              <span className="font-medium text-[#1D1D1F] dark:text-gray-300">Suggested Discount</span>
              <span className="text-blue-600 font-semibold">20% off for 48 hours</span>
            </div>
            <div className="flex justify-between items-center pb-2 border-b border-gray-100 dark:border-gray-800">
              <span className="font-medium text-[#1D1D1F] dark:text-gray-300">Estimated Revenue</span>
              <span className="text-green-600 font-bold">$450.00</span>
            </div>
          </div>

          <div className="flex flex-col gap-3">
            <button
              onClick={handleApprove}
              disabled={loading}
              className="w-full min-h-[44px] flex items-center justify-center bg-[#0071E3] text-white rounded-[8px] font-medium hover:bg-[#005bb5] transition-colors"
            >
              {loading ? "Applying..." : "Approve & Notify Customers"}
            </button>
            <button
              onClick={() => router.push("/smart-pricing")}
              className="w-full min-h-[44px] flex items-center justify-center bg-gray-200 text-[#1D1D1F] rounded-[8px] font-medium hover:bg-gray-300 transition-colors"
            >
              Adjust Details
            </button>
          </div>
        </div>
      </main>

      <style
        dangerouslySetInnerHTML={{
          __html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `,
        }}
      />
    </div>
  );
}
