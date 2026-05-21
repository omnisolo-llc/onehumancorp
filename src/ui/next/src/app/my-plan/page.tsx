"use client";

import { useState, useEffect } from "react";

export default function MyPlan() {
  const [planData, setPlanData] = useState<any>(null);
  const [costData, setCostData] = useState<any>(null);
  const [showCostDetails, setShowCostDetails] = useState(false);

  useEffect(() => {
    async function fetchData() {
      try {
        const token = localStorage.getItem('token') || '';

        const planRes = await fetch('/api/billing/my-plan', {
          headers: { 'Authorization': `Bearer ${token}` }
        });
        if (planRes.ok) {
          setPlanData(await planRes.json());
        } else {
            setPlanData({ current_plan: 'Free' });
        }

        const costRes = await fetch('/api/billing/cost-dashboard', {
          headers: { 'Authorization': `Bearer ${token}` }
        });
        if (costRes.ok) {
          setCostData(await costRes.json());
        } else {
            setCostData({
                total_revenue: 0,
                total_costs: 0,
                llm_cost: 0,
                storage_cost: 0
            });
        }
      } catch (err) {
        console.error("Failed to fetch billing data", err);
        setPlanData({ current_plan: 'Free' });
        setCostData({
            total_revenue: 0,
            total_costs: 0,
            llm_cost: 0,
            storage_cost: 0
        });
      }
    }
    fetchData();
  }, []);

  return (
    <div className="flex flex-col min-h-screen font-inter dark:bg-[#16161A] bg-[#F5F5F7] transition-colors duration-300">
      <header className="px-6 py-4 flex items-center justify-between border-b dark:border-white/10 border-black/10 dark:bg-[#16161A]/70 bg-white/65 backdrop-blur-[30px] saturate-[210%] sticky top-0 z-50">
         <h1 className="text-2xl font-bold font-outfit dark:text-[#F5F5F7] text-[#1D1D1F] tracking-tight">My Current Plan</h1>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-8">
        <section className="p-6 rounded-2xl shadow-sm border dark:border-white/10 border-gray-100 dark:bg-[#16161A]/70 bg-white/65 backdrop-blur-[30px] saturate-[210%]">
            <h2 className="text-xl font-semibold mb-4 font-outfit dark:text-[#F5F5F7] text-[#1D1D1F]">Plan: {planData?.current_plan || 'Free'}</h2>
            <button
                onClick={() => setShowCostDetails(!showCostDetails)}
                className="px-4 py-2 bg-[#0066FF] text-white rounded-[8px] font-medium transition-colors"
            >
                {showCostDetails ? "Hide Cost Details" : "View Cost Details"}
            </button>
        </section>

        {showCostDetails && (
            <section className="p-6 rounded-2xl shadow-sm border dark:border-[#0066FF]/30 border-blue-100 dark:bg-[#16161A]/70 bg-white/65 backdrop-blur-[30px] saturate-[210%]">
                <h2 className="text-xl font-semibold mb-4 font-outfit dark:text-blue-300 text-blue-900">Cost & Usage</h2>
                {costData ? (
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                        <div className="p-4 rounded-[16px] border dark:border-white/5 border-gray-100 dark:bg-[#16161A]/50 bg-white shadow-sm">
                            <div className="text-sm dark:text-gray-400 text-gray-500 mb-1">Total Revenue</div>
                            <div className="text-2xl font-bold dark:text-[#F5F5F7] text-[#1D1D1F]">${(costData.total_revenue / 100).toFixed(2)}</div>
                        </div>
                        <div className="p-4 rounded-[16px] border dark:border-white/5 border-gray-100 dark:bg-[#16161A]/50 bg-white shadow-sm">
                            <div className="text-sm dark:text-gray-400 text-gray-500 mb-1">Total Costs</div>
                            <div className="text-2xl font-bold text-[#FF3B30] dark:text-[#FF3B30]">${(costData.total_costs / 100).toFixed(2)}</div>
                        </div>
                        <div className="p-4 rounded-[16px] border dark:border-white/5 border-gray-100 dark:bg-[#16161A]/50 bg-white shadow-sm">
                            <div className="text-sm dark:text-gray-400 text-gray-500 mb-1">Smart Assistant Usage</div>
                            <div className="text-2xl font-bold dark:text-[#F5F5F7] text-[#1D1D1F]">${(costData.llm_cost / 100).toFixed(2)}</div>
                        </div>
                        <div className="p-4 rounded-[16px] border dark:border-white/5 border-gray-100 dark:bg-[#16161A]/50 bg-white shadow-sm">
                            <div className="text-sm dark:text-gray-400 text-gray-500 mb-1">Storage Cost</div>
                            <div className="text-2xl font-bold dark:text-[#F5F5F7] text-[#1D1D1F]">${(costData.storage_cost / 100).toFixed(2)}</div>
                        </div>
                    </div>
                ) : (
                    <p className="dark:text-gray-400 text-gray-600">Loading usage details...</p>
                )}
            </section>
        )}
      </main>
    </div>
  );
}
