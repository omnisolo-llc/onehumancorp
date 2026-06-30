"use client";

import React, { useState, useEffect } from "react";
import { useRouter } from "next/navigation";

export default function SmartPricingPage() {
  const router = useRouter();

  const [enabled, setEnabled] = useState(false);
  const [discountPerishables, setDiscountPerishables] = useState(false);
  const [surgePricing, setSurgePricing] = useState(false);
  const [maxAdjustment, setMaxAdjustment] = useState(20);

  useEffect(() => {
    // Only fetch on client
    if (typeof window !== "undefined") {
      const savedEnabled = localStorage.getItem("smartPricingEnabled");
      if (savedEnabled !== null) setEnabled(JSON.parse(savedEnabled));

      const savedPerishables = localStorage.getItem("smartPricingPerishables");
      if (savedPerishables !== null)
        setDiscountPerishables(JSON.parse(savedPerishables));

      const savedSurge = localStorage.getItem("smartPricingSurge");
      if (savedSurge !== null) setSurgePricing(JSON.parse(savedSurge));

      const savedMax = localStorage.getItem("smartPricingMaxAdjustment");
      if (savedMax !== null) setMaxAdjustment(parseInt(savedMax, 10));
    }
  }, []);

  useEffect(() => {
    if (typeof window !== "undefined") {
      localStorage.setItem("smartPricingEnabled", JSON.stringify(enabled));
      localStorage.setItem(
        "smartPricingPerishables",
        JSON.stringify(discountPerishables),
      );
      localStorage.setItem("smartPricingSurge", JSON.stringify(surgePricing));
      localStorage.setItem(
        "smartPricingMaxAdjustment",
        maxAdjustment.toString(),
      );

      // Simulate real backend mutation loop silently if enabled
      if (enabled) {
        fetch("/api/agents/approvals/simulate-smart-pricing", {
          method: "POST",
          headers: {
            Authorization: `Bearer ${localStorage.getItem("token") || ""}`,
            "Content-Type": "application/json",
          },
        }).catch(() => {
          /* silent fail in local mode */
        });
      }
    }
  }, [enabled, discountPerishables, surgePricing, maxAdjustment]);

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-6 py-4 flex items-center justify-between border-b border-white/40 dark:border-white/10 sticky top-0 z-50 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[2.1]">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-[-0.02em]">
          Smart Pricing
        </h1>
        <div className="flex items-center min-h-[44px]">
          <button
            onClick={() => router.push("/dashboard")}
            className="px-4 py-2 bg-gray-200 text-sm font-medium hover:bg-gray-300 transition-colors"
          >
            Back to Dashboard
          </button>
        </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-3xl mx-auto w-full flex flex-col gap-6">
        <div className="text-center mb-4">
          <p className="text-lg text-[#86868B]">
            Let AI automatically adjust your prices to maximize revenue and
            clear inventory, while staying within your safe limits.
          </p>
        </div>

        <div className="p-6 shadow-sm flex items-center justify-between bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
          <div>
            <h3 className="text-lg font-semibold font-outfit text-[#1D1D1F]">
              Enable Smart Pricing
            </h3>
            <p className="text-sm text-gray-500 mt-1">
              Turn on autonomous hyper-local dynamic pricing.
            </p>
          </div>
          <div className="flex items-center min-h-[44px]">
            <button
              aria-label="Enable Smart Pricing"
              aria-pressed={enabled}
              data-testid="enable-smart-pricing-toggle"
              onClick={() => setEnabled(!enabled)}
              className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${enabled ? "bg-[#34C759]" : "bg-gray-300"}`}
            >
              <span
                className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${enabled ? "translate-x-6" : "translate-x-1"}`}
              />
            </button>
          </div>
        </div>

        {enabled && (
          <div className="p-6 shadow-sm flex flex-col gap-6 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
            <h3 className="text-lg font-semibold font-outfit border-b pb-2 text-[#1D1D1F] border-black/10">
              Configuration
            </h3>

            <div className="flex items-center justify-between gap-4">
              <div className="flex-1">
                <p className="font-medium text-[#1D1D1F]">
                  Auto-discount perishables 2 hours before closing
                </p>
                <p className="text-xs text-gray-500 mt-1">
                  Clear out remaining inventory today.
                </p>
              </div>
              <div className="flex items-center min-h-[44px]">
                <button
                  aria-label="Auto-discount perishables"
                  aria-pressed={discountPerishables}
                  data-testid="discount-perishables-toggle"
                  onClick={() => setDiscountPerishables(!discountPerishables)}
                  className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${discountPerishables ? "bg-[#0066FF]" : "bg-gray-300"}`}
                >
                  <span
                    className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${discountPerishables ? "translate-x-6" : "translate-x-1"}`}
                  />
                </button>
              </div>
            </div>

            <div className="flex items-center justify-between gap-4">
              <div className="flex-1">
                <p className="font-medium text-[#1D1D1F]">
                  Surge pricing during high demand
                </p>
                <p className="text-xs text-gray-500 mt-1">
                  Charge a premium during peak rush hours.
                </p>
              </div>
              <div className="flex items-center min-h-[44px]">
                <button
                  aria-label="Surge pricing"
                  aria-pressed={surgePricing}
                  data-testid="surge-pricing-toggle"
                  onClick={() => setSurgePricing(!surgePricing)}
                  className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${surgePricing ? "bg-[#0066FF]" : "bg-gray-300"}`}
                >
                  <span
                    className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${surgePricing ? "translate-x-6" : "translate-x-1"}`}
                  />
                </button>
              </div>
            </div>

            <div className="mt-4">
              <div className="flex justify-between items-center mb-2">
                <label className="font-medium text-[#1D1D1F]">
                  Maximum price adjustment bounds (+/-)
                </label>
                <span className="font-bold text-[#0071E3]">
                  {maxAdjustment}%
                </span>
              </div>
              <div className="flex items-center min-h-[44px]">
                <input
                  aria-label="Maximum price adjustment bounds"
                  type="range"
                  min="5"
                  max="50"
                  step="5"
                  value={maxAdjustment}
                  onChange={(e) => setMaxAdjustment(parseInt(e.target.value))}
                  className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
                  data-testid="price-bounds-slider"
                />
              </div>
            </div>
          </div>
        )}
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
