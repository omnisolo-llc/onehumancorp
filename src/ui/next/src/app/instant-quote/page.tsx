"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

interface PricingRule {
  id: string;
  rule_name: string;
  base_price_cents: number;
  service_category: string;
  modifiers: { type: string; condition: string; value: number | string }[];
}

export default function InstantQuotePage() {
  const router = useRouter();
  const [rules, setRules] = useState<PricingRule[]>([]);
  const [selectedService, setSelectedService] = useState<string>('');
  const [isRush, setIsRush] = useState(false);
  const [isWeekend, setIsWeekend] = useState(false);
  const [estimatedPrice, setEstimatedPrice] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);

  // Fetch pricing rules on component mount
  useEffect(() => {
    async function fetchRules() {
      try {
        const res = await fetch('/api/pricing/rules', {
          headers: { 'x-tenant-id': 'default' }
        });
        if (res.ok) {
          const data = await res.json();
          setRules(data);
        } else {
          console.error("Failed to fetch pricing rules");
        }
      } catch (err) {
        console.error("Error fetching rules:", err);
      } finally {
        setLoading(false);
      }
    }
    fetchRules();
  }, []);

  // Deterministic Evaluation Engine < 50ms
  useEffect(() => {
    if (!selectedService || rules.length === 0) {
      setEstimatedPrice(null);
      return;
    }

    const rule = rules.find(r => r.service_category === selectedService);
    if (!rule) {
      setEstimatedPrice(null);
      return;
    }

    let price = rule.base_price_cents;

    rule.modifiers.forEach(mod => {
      if (mod.condition === 'rush' && isRush) {
        if (mod.type === 'flat') {
          price += Number(mod.value);
        } else if (mod.type === 'percentage') {
          price += price * (Number(mod.value) / 100);
        }
      }
      if (mod.condition === 'weekend' && isWeekend) {
        if (mod.type === 'flat') {
          price += Number(mod.value);
        } else if (mod.type === 'percentage') {
          price += price * (Number(mod.value) / 100);
        }
      }
    });

    setEstimatedPrice(price);
  }, [selectedService, isRush, isWeekend, rules]);

  const handleSubmit = async () => {
    if (!estimatedPrice) return;

    // Simulate sending to triage/SalesAgent
    await fetch('/api/agents/approvals/simulate-quote-draft', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json', 'x-tenant-id': 'default' },
      body: JSON.stringify({
         inbox_message_id: 'msg-' + Date.now(),
         suggested_price: estimatedPrice / 100
      })
    });
    alert('Quote requested! The owner will review and send it to you shortly.');
    router.push('/');
  };

  return (
    <div className="flex flex-col min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 font-inter text-gray-900 w-full overflow-x-hidden">
      <header className="px-4 py-4 flex items-center justify-between sticky top-0 z-50 bg-white/70 backdrop-blur-xl saturate-200 border-b border-white/40 shadow-sm w-full">
        <h1 className="text-xl md:text-2xl font-bold font-outfit text-gray-900 tracking-tight">Instant Quote</h1>
        <button onClick={() => router.back()} className="min-w-[44px] min-h-[44px] px-3 py-2 bg-gray-100 rounded-xl text-sm font-medium text-gray-800 hover:bg-gray-200 transition-colors">
          Back
        </button>
      </header>

      <main className="p-4 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col gap-6">
        {loading ? (
           <p className="text-center text-gray-500">Loading options...</p>
        ) : (
          <div className="glassmorphism p-6 rounded-2xl border border-white/40 shadow-lg flex flex-col gap-6">
            <div>
              <label className="block text-sm font-semibold text-gray-700 mb-2">Select Service</label>
              <select
                className="w-full p-3 bg-white border border-gray-200 rounded-xl focus:ring-2 focus:ring-indigo-500 outline-none text-gray-900"
                value={selectedService}
                onChange={(e) => setSelectedService(e.target.value)}
                data-testid="service-select"
              >
                <option value="">-- Choose a service --</option>
                {rules.map(r => (
                  <option key={r.id} value={r.service_category}>{r.rule_name}</option>
                ))}
              </select>
            </div>

            <div className="flex items-center justify-between p-4 bg-white/50 rounded-xl border border-gray-100">
              <span className="font-medium text-gray-800">Rush Delivery</span>
              <input
                type="checkbox"
                checked={isRush}
                onChange={(e) => setIsRush(e.target.checked)}
                className="w-6 h-6 text-indigo-600 rounded-lg focus:ring-indigo-500"
                data-testid="rush-checkbox"
              />
            </div>

            <div className="flex items-center justify-between p-4 bg-white/50 rounded-xl border border-gray-100">
              <span className="font-medium text-gray-800">Weekend Service</span>
              <input
                type="checkbox"
                checked={isWeekend}
                onChange={(e) => setIsWeekend(e.target.checked)}
                className="w-6 h-6 text-indigo-600 rounded-lg focus:ring-indigo-500"
                data-testid="weekend-checkbox"
              />
            </div>
          </div>
        )}
      </main>

      <div className="sticky bottom-0 bg-white/80 backdrop-blur-xl border-t border-gray-200 p-4 w-full shadow-[0_-4px_20px_-10px_rgba(0,0,0,0.1)]">
        <div className="max-w-lg mx-auto flex items-center justify-between gap-4">
          <div className="flex flex-col">
            <span className="text-xs text-gray-500 font-medium">Estimated Quote</span>
            <span className="text-2xl font-bold text-gray-900" data-testid="estimated-price">
              {estimatedPrice !== null ? `$${(estimatedPrice / 100).toFixed(2)}` : '--'}
            </span>
          </div>
          <button
            disabled={!estimatedPrice}
            onClick={handleSubmit}
            className="flex-1 bg-indigo-600 text-white font-semibold py-3 px-6 rounded-xl hover:bg-indigo-700 transition-colors disabled:opacity-50 min-h-[44px]"
            data-testid="request-quote-btn"
          >
            Request Final Quote
          </button>
        </div>
      </div>
    </div>
  );
}
