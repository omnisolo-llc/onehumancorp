'use client';

import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';

interface DepartmentTierUsage {
  current_plan: string;
  period: string;
  departments: {
    id: string;
    department_type: string;
    agent_id: string;
    actions_used: number;
    action_limit: number | null;
    usage_percent: number | null;
    soft_limit_reached: boolean;
  }[];
}

interface CostDashboardResponse {
  department_tier_usage?: DepartmentTierUsage;
}

export default function AiUsagePaywallPage() {
  const router = useRouter();
  const [data, setData] = useState<CostDashboardResponse | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchData() {
      try {
        const headers: Record<string, string> = {};
        if (typeof window !== 'undefined') {
          const tenantId = localStorage.getItem('ohc_active_tenant_id');
          if (tenantId) headers['x-ohc-tenant-id'] = tenantId;
        }

        const costRes = await fetch('/api/billing/cost-dashboard', { headers });
        if (costRes.ok) {
          const costData = await costRes.json();
          setData(costData);
        } else {
          console.error("Failed to fetch cost data:", costRes.status);
        }
      } catch (err) {
        console.error("Failed to fetch cost data:", err);
      } finally {
        setLoading(false);
      }
    }
    fetchData();
  }, []);

  const handleShareOnX = () => {
    router.push('/growth-loop?ref=twitter');
  };

  const handleUpgrade = () => {
    router.push('/pricing');
  };

  if (loading) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter">
        <p className="text-gray-500 font-medium">Loading AI Usage...</p>
      </div>
    );
  }

  const usage = data?.department_tier_usage;
  const currentPlan = usage?.current_plan || 'Free';
  const departments = usage?.departments || [];

  // Calculate total actions
  const totalUsed = departments.reduce((acc, d) => acc + d.actions_used, 0);
  const totalLimit = departments.reduce((acc, d) => acc + (d.action_limit || 0), 0);
  const overallPercent = totalLimit > 0 ? Math.min(100, Math.round((totalUsed / totalLimit) * 100)) : 0;

  const isLimitReached = departments.some(d => d.soft_limit_reached) || (totalLimit > 0 && totalUsed >= totalLimit);

  return (
    <div className="min-h-screen bg-gray-50 font-inter p-6 flex flex-col items-center">
      <div className="w-full max-w-2xl mt-12 mb-8 text-center">
        <div className="inline-flex items-center justify-center p-3 bg-blue-100 text-blue-600 rounded-full mb-6 shadow-sm">
          <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
        </div>
        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-4 tracking-tight">AI Agent Usage</h1>
        <p className="text-gray-500 text-lg">Monitor your AI automation and unlock unlimited capabilities.</p>
      </div>

      <div className="w-full max-w-2xl p-8 rounded-[24px] glassmorphism/60 backdrop-blur-2xl saturate-200 border border-white/40 shadow-xl mb-8 relative overflow-hidden">
        {/* Glow effect */}
        <div className="absolute top-0 right-0 -mt-20 -mr-20 w-64 h-64 bg-blue-400/20 rounded-full blur-[80px] pointer-events-none"></div>

        <div className="flex flex-col md:flex-row items-center justify-between gap-6 mb-8 relative z-10">
          <div>
            <span className="text-sm font-bold text-blue-600 uppercase tracking-widest">{currentPlan} Plan</span>
            <h2 className="text-4xl font-black text-gray-900 font-outfit mt-1">
              {totalUsed} <span className="text-xl text-gray-500 font-medium">/ {totalLimit > 0 ? totalLimit : 'Unlimited'} actions</span>
            </h2>
          </div>
          <div className="w-full md:w-1/2">
             <div className="flex justify-between text-sm font-medium mb-2">
                <span className="text-gray-600">Capacity</span>
                <span className={isLimitReached ? "text-red-600" : "text-gray-900"}>{overallPercent}%</span>
             </div>
             <div className="h-3 w-full bg-gray-100 rounded-full overflow-hidden shadow-inner">
                <div
                  className={`h-full rounded-full transition-all duration-1000 ${isLimitReached ? 'bg-red-500' : 'bg-gradient-to-r from-blue-500 to-indigo-500'}`}
                  style={{ width: `${overallPercent}%` }}
                ></div>
             </div>
          </div>
        </div>

        {departments.length > 0 && (
          <div className="space-y-4 mb-8 border-t border-gray-100 pt-6 relative z-10">
            <h3 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4">Breakdown by Department</h3>
            {departments.map((dept) => (
              <div key={dept.id} className="flex justify-between items-center p-4 glassmorphism/80 rounded-xl shadow-sm border border-gray-50">
                <div className="flex items-center gap-3">
                   <div className={`w-2 h-2 rounded-full ${dept.soft_limit_reached ? 'bg-red-500' : 'bg-green-500'}`}></div>
                   <div>
                     <p className="font-semibold text-gray-900 capitalize">{dept.department_type}</p>
                     <p className="text-xs text-gray-500">{dept.agent_id.replace(/_/g, ' ')}</p>
                   </div>
                </div>
                <div className="text-right">
                  <p className="font-bold text-gray-900">{dept.actions_used} <span className="text-gray-400 font-normal">/ {dept.action_limit || '∞'}</span></p>
                  {dept.soft_limit_reached && <p className="text-xs text-red-500 font-medium mt-1">Limit reached</p>}
                </div>
              </div>
            ))}
          </div>
        )}

        <div className="flex flex-col sm:flex-row gap-4 relative z-10">
          <button
            onClick={handleUpgrade}
            className="flex-1 py-4 px-6 bg-gray-900 text-white rounded-xl font-bold hover:bg-black transition-colors shadow-lg hover:shadow-xl active:scale-[0.98] flex justify-center items-center gap-2"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" /></svg>
            Upgrade to Pro
          </button>
          <button
            onClick={handleShareOnX}
            className="flex-1 py-4 px-6 bg-[#1DA1F2]/10 text-[#1DA1F2] rounded-xl font-bold hover:bg-[#1DA1F2]/20 transition-colors border border-[#1DA1F2]/20 active:scale-[0.98] flex justify-center items-center gap-2 group"
          >
            <svg className="w-5 h-5 fill-current" viewBox="0 0 24 24"><path d="M23.953 4.57a10 10 0 01-2.825.775 4.958 4.958 0 002.163-2.723c-.951.555-2.005.959-3.127 1.184a4.92 4.92 0 00-8.384 4.482C7.69 8.095 4.067 6.13 1.64 3.162a4.822 4.822 0 00-.666 2.475c0 1.71.87 3.213 2.188 4.096a4.904 4.904 0 01-2.228-.616v.06a4.923 4.923 0 003.946 4.827 4.996 4.996 0 01-2.212.085 4.936 4.936 0 004.604 3.417 9.867 9.867 0 01-6.102 2.105c-.39 0-.779-.023-1.17-.067a13.995 13.995 0 007.557 2.209c9.053 0 13.998-7.496 13.998-13.985 0-.21 0-.42-.015-.63A9.935 9.935 0 0024 4.59z"/></svg>
            Share to get 10 free tasks
          </button>
        </div>
      </div>

      <div className="mt-4 text-center">
        <button onClick={() => router.push('/about-ohc')} className="inline-flex items-center gap-1 text-sm font-bold text-gray-400 hover:text-gray-600 transition-colors">
          ⚡ Powered by OHC
        </button>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;700;900&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}