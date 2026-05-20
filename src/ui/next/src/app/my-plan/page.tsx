"use client";

import { useState, useEffect } from "react";

export default function MyPlan() {
  const [planData, setPlanData] = useState<any>(null);
  const [costData, setCostData] = useState<any>(null);
  const [showCostDashboard, setShowCostDashboard] = useState(false);

  useEffect(() => {
    fetch('/api/billing/my-plan')
      .then(res => res.json())
      .then(data => setPlanData(data))
      .catch(console.error);

    fetch('/api/billing/cost-dashboard')
      .then(res => res.json())
      .then(data => setCostData(data))
      .catch(console.error);
  }, []);

  if (!planData) return <div className="p-8 text-center">Loading plan details...</div>;

  const storageUsedMB = Math.round((planData.storage_used_bytes || 0) / (1024 * 1024));
  const storageLimitText = planData.storage_limit_bytes ? Math.round(planData.storage_limit_bytes / (1024 * 1024)) + 'MB' : 'Unlimited';
  const aiLimitText = planData.ai_actions_limit || 'Unlimited';

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gray-50">
      <header className="px-6 py-4 flex items-center justify-between border-b bg-white">
         <h1 className="text-2xl font-bold">My Current Plan</h1>
      </header>

      <main className="p-6 md:p-8 max-w-3xl mx-auto w-full">
        {!showCostDashboard ? (
          <div className="bg-white p-8 rounded-xl shadow-sm border border-gray-100">
            <div className="mb-8 pb-8 border-b border-gray-100">
                <h2 className="text-xl font-semibold mb-4 text-gray-800">Plan: {planData.current_plan}</h2>
                <div className="text-sm text-gray-500 mb-2">Status: <span className="text-green-600 font-medium">Active</span></div>
                <div className="text-sm text-gray-500">Estimated Next Bill: <span className="font-medium text-gray-900">${planData.next_bill_estimated}.00</span></div>
            </div>

            <div className="mb-8">
                <h3 className="text-lg font-semibold mb-4 text-gray-800">Your Current Usage</h3>
                <div className="space-y-4">
                    <div className="bg-gray-50 p-4 rounded-lg">
                        <div className="flex justify-between mb-1">
                            <span className="text-sm font-medium text-gray-700">AI Actions Used</span>
                            <span className="text-sm font-medium text-gray-900">{planData.ai_actions_used} / {aiLimitText}</span>
                        </div>
                        {planData.ai_actions_limit && (
                            <div className="w-full bg-gray-200 rounded-full h-2">
                                <div className="bg-blue-600 h-2 rounded-full" style={{ width: `${Math.min(100, (planData.ai_actions_used / planData.ai_actions_limit) * 100)}%` }}></div>
                            </div>
                        )}
                    </div>

                    <div className="bg-gray-50 p-4 rounded-lg">
                        <div className="flex justify-between mb-1">
                            <span className="text-sm font-medium text-gray-700">Storage Used</span>
                            <span className="text-sm font-medium text-gray-900">{storageUsedMB}MB / {storageLimitText}</span>
                        </div>
                        {planData.storage_limit_bytes && (
                            <div className="w-full bg-gray-200 rounded-full h-2">
                                <div className="bg-blue-600 h-2 rounded-full" style={{ width: `${Math.min(100, ((planData.storage_used_bytes || 0) / planData.storage_limit_bytes) * 100)}%` }}></div>
                            </div>
                        )}
                    </div>
                </div>
            </div>

            <div className="flex flex-col sm:flex-row gap-3">
                <a href="/pricing" className="px-6 py-2 bg-blue-600 text-white rounded-lg font-medium text-center hover:bg-blue-700 transition-colors">Upgrade via Stripe</a>
                <button onClick={() => setShowCostDashboard(true)} className="px-6 py-2 bg-white border border-gray-300 text-gray-700 rounded-lg font-medium hover:bg-gray-50 transition-colors">View Cost Details</button>
            </div>
          </div>
        ) : (
          <div className="bg-white p-8 rounded-xl shadow-sm border border-gray-100">
            <div className="flex items-center justify-between mb-6">
                <h2 className="text-xl font-semibold text-gray-800">Cost & AI Usage</h2>
                <button onClick={() => setShowCostDashboard(false)} className="text-sm text-blue-600 hover:text-blue-800 font-medium">&larr; Back to My Plan</button>
            </div>

            {costData ? (
                <div className="space-y-6">
                    <div className="p-6 bg-blue-50 border border-blue-100 rounded-xl">
                        <div className="text-sm text-blue-600 font-medium mb-1">Total Costs</div>
                        <div className="text-4xl font-bold text-blue-900">${(costData.total_costs / 100).toFixed(2)}</div>
                        <div className="text-xs text-blue-500 mt-2">Period: {costData.period_start} to {costData.period_end}</div>
                    </div>

                    <div className="grid grid-cols-2 gap-4">
                        <div className="p-4 bg-gray-50 border border-gray-200 rounded-lg">
                            <div className="text-xs text-gray-500 font-medium mb-1">LLM Usage</div>
                            <div className="text-xl font-semibold text-gray-900">${(costData.llm_cost / 100).toFixed(2)}</div>
                        </div>
                        <div className="p-4 bg-gray-50 border border-gray-200 rounded-lg">
                            <div className="text-xs text-gray-500 font-medium mb-1">Storage Cost</div>
                            <div className="text-xl font-semibold text-gray-900">${(costData.storage_cost / 100).toFixed(2)}</div>
                        </div>
                    </div>
                </div>
            ) : (
                <div className="text-gray-500 text-center py-8">Loading cost details...</div>
            )}
          </div>
        )}
      </main>
    </div>
  );
}
