"use client";

import { useState, useEffect } from "react";

export default function MyPlan() {
  const [planData, setPlanData] = useState<any>(null);

  useEffect(() => {
    async function fetchPlan() {
      try {
        const res = await fetch('/api/billing/my-plan');
        const data = await res.json();
        setPlanData(data);
      } catch (e) {
        console.error("Failed to fetch plan info", e);
      }
    }
    fetchPlan();
  }, []);

  if (!planData) {
    return <div className="p-8">Loading...</div>;
  }

  const aiLimit = planData.ai_actions_limit ? planData.ai_actions_limit : 'Unlimited';
  const storageUsedMB = Math.round(planData.storage_used_bytes / (1024 * 1024));
  const storageLimitText = planData.storage_limit_bytes ? Math.round(planData.storage_limit_bytes / (1024 * 1024)) + 'MB' : 'Unlimited';

  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4">
      <main className="max-w-2xl w-full p-8 shadow-sm flex flex-col gap-6" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
        <h1 className="text-3xl font-bold font-outfit text-gray-900">My Plan</h1>

        <div className="flex flex-col gap-4">
          <div className="flex justify-between items-center border-b pb-4 border-gray-100">
            <span className="text-gray-600 font-medium text-lg">Current Plan</span>
            <span className="font-semibold text-xl text-blue-600">{planData.current_plan}</span>
          </div>

          <div className="flex justify-between items-center border-b pb-4 border-gray-100">
            <span className="text-gray-600 font-medium text-lg">AI Actions Used</span>
            <span className="font-semibold text-lg text-gray-900">{planData.ai_actions_used} / {aiLimit}</span>
          </div>

          <div className="flex justify-between items-center border-b pb-4 border-gray-100">
            <span className="text-gray-600 font-medium text-lg">Storage Used</span>
            <span className="font-semibold text-lg text-gray-900">{storageUsedMB}MB / {storageLimitText}</span>
          </div>

          <div className="flex justify-between items-center border-b pb-4 border-gray-100">
            <span className="text-gray-600 font-medium text-lg">Estimated Next Bill</span>
            <span className="font-semibold text-xl text-gray-900">${planData.next_bill_estimated}.00</span>
          </div>
        </div>

        <div className="mt-6">
          <button onClick={() => window.location.href = '/billing/pricing'} className="w-full py-3 font-semibold text-white bg-blue-600 hover:bg-blue-700 transition-colors shadow-sm rounded-lg text-lg">
            Upgrade Plan
          </button>
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
