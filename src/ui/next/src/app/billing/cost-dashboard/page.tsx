"use client";

import { useState, useEffect } from "react";

export default function CostDashboard() {
  const [costData, setCostData] = useState<any>(null);

  useEffect(() => {
    async function fetchCost() {
      try {
        const res = await fetch('/api/billing/cost-dashboard');
        const data = await res.json();
        setCostData(data);
      } catch (e) {
        console.error("Failed to fetch cost info", e);
      }
    }
    fetchCost();
  }, []);

  if (!costData) {
    return <div className="p-8">Loading...</div>;
  }

  const formatCurrency = (cents: number) => {
    return '$' + (cents / 100).toFixed(2);
  };

  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4">
      <main className="max-w-4xl w-full p-8 shadow-sm flex flex-col gap-8" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
        <div className="flex justify-between items-end border-b pb-4 border-gray-200">
            <div>
                <h1 className="text-3xl font-bold font-outfit text-gray-900">Cost Transparency Dashboard</h1>
                <p className="text-gray-600 mt-2">See exactly what drives your infrastructure costs.</p>
            </div>
            <div className="text-right">
                <span className="text-sm text-gray-500 font-medium bg-gray-100 px-3 py-1 rounded-full">
                    {costData.period_start} to {costData.period_end}
                </span>
            </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="p-6 bg-white rounded-xl border border-gray-100 shadow-sm flex flex-col gap-2">
                <span className="text-gray-500 font-medium">Total Costs</span>
                <span className="text-4xl font-bold text-gray-900">{formatCurrency(costData.total_costs)}</span>
            </div>
            <div className="p-6 bg-white rounded-xl border border-gray-100 shadow-sm flex flex-col gap-2">
                <span className="text-gray-500 font-medium">Total Revenue</span>
                <span className="text-4xl font-bold text-green-600">{formatCurrency(costData.total_revenue)}</span>
            </div>
        </div>

        <div>
            <h2 className="text-xl font-semibold mb-4 text-gray-800">Cost Breakdown</h2>
            <div className="space-y-4">
                <div className="flex justify-between items-center p-4 bg-gray-50 rounded-lg">
                    <div className="flex items-center gap-3">
                        <div className="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center text-blue-600 text-xl">🤖</div>
                        <span className="font-medium text-gray-700">LLM Usage (Tokens)</span>
                    </div>
                    <span className="font-semibold text-lg">{formatCurrency(costData.llm_cost)}</span>
                </div>

                <div className="flex justify-between items-center p-4 bg-gray-50 rounded-lg">
                    <div className="flex items-center gap-3">
                        <div className="w-10 h-10 rounded-full bg-purple-100 flex items-center justify-center text-purple-600 text-xl">💾</div>
                        <span className="font-medium text-gray-700">Storage & CDN</span>
                    </div>
                    <span className="font-semibold text-lg">{formatCurrency(costData.storage_cost)}</span>
                </div>

                <div className="flex justify-between items-center p-4 bg-gray-50 rounded-lg">
                    <div className="flex items-center gap-3">
                        <div className="w-10 h-10 rounded-full bg-green-100 flex items-center justify-center text-green-600 text-xl">💳</div>
                        <span className="font-medium text-gray-700">Payment Processing Fees</span>
                    </div>
                    <span className="font-semibold text-lg">{formatCurrency(costData.payment_fees)}</span>
                </div>
            </div>
        </div>
      </main>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
