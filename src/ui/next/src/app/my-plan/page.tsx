"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";

export default function MyPlanDashboard() {
  const router = useRouter();
  const [showCostDetails, setShowCostDetails] = useState(false);
  const [planData, setPlanData] = useState<any>(null);
  const [costData, setCostData] = useState<any>(null);

  useEffect(() => {
    fetch('/api/my-plan')
      .then(res => res.json())
      .then(data => setPlanData(data))
      .catch(e => console.error(e));

    fetch('/api/cost-dashboard')
      .then(res => res.json())
      .then(data => setCostData(data))
      .catch(e => console.error(e));
  }, []);

  const planName = planData ? planData.current_plan : "Free";
  const aiUsed = planData ? planData.ai_actions_used : 0;
  const aiLimit = planData && planData.ai_actions_limit ? planData.ai_actions_limit : 100;
  const storageUsedMB = planData ? Math.round(planData.storage_used_bytes / (1024 * 1024)) : 0;
  const storageLimitMB = planData && planData.storage_limit_bytes ? Math.round(planData.storage_limit_bytes / (1024 * 1024)) : 500;

  const estimatedBill = planData ? (planData.next_bill_estimated / 100).toFixed(2) : "0.00";

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 font-inter p-8">
      <div className="max-w-4xl mx-auto w-full">
        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-8">My Current Plan</h1>

        <div className="bg-white p-6 rounded-xl shadow-sm border border-gray-200 mb-8">
          <h2 className="text-xl font-bold text-gray-800 mb-4">Plan: {planName}</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
            <div className="bg-gray-50 p-4 rounded-lg border border-gray-100">
              <p className="text-sm text-gray-500 mb-1">AI Actions Used</p>
              <p className="text-2xl font-bold text-gray-900">{aiUsed} / {aiLimit}</p>
            </div>
            <div className="bg-gray-50 p-4 rounded-lg border border-gray-100">
              <p className="text-sm text-gray-500 mb-1">Storage Used</p>
              <p className="text-2xl font-bold text-gray-900">{storageUsedMB}MB / {storageLimitMB}MB</p>
            </div>
          </div>

          <div className="flex gap-4">
            <button onClick={() => router.push('/pricing')} className="bg-blue-600 text-white px-6 py-2 rounded-lg font-medium hover:bg-blue-700 transition">View Upgrade Plans</button>
            <button onClick={() => setShowCostDetails(!showCostDetails)} className="bg-gray-100 text-gray-800 px-6 py-2 rounded-lg font-medium hover:bg-gray-200 transition">View Cost Details</button>
          </div>
        </div>

        {showCostDetails && (
          <div className="bg-white p-6 rounded-xl shadow-sm border border-gray-200">
            <h2 className="text-xl font-bold text-gray-800 mb-4">Cost & AI Usage</h2>
            <p className="text-gray-600 mb-4">Your current estimated bill is ${estimatedBill}.</p>
            {costData && (
              <div className="text-sm text-gray-700 space-y-2">
                <p>LLM Costs: ${(costData.llm_cost / 100).toFixed(2)}</p>
                <p>Storage Costs: ${(costData.storage_cost / 100).toFixed(2)}</p>
                <p>Payment Fees: ${(costData.payment_fees / 100).toFixed(2)}</p>
                <p className="font-bold border-t pt-2 mt-2">Total Costs: ${(costData.total_costs / 100).toFixed(2)}</p>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
