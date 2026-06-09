"use client";

// Cost Dashboard Implementation - Refactored for UniFi-inspired Premium Layout
import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

interface DailyCost {
  date: string;
  total_cost: number;
  llm_cost: number;
  storage_cost: number;
  network_cost: number;
  compute_cost?: number;
}

interface CostDashboardData {
  total_revenue: number;
  total_costs: number;
  projected_monthly_cost: number;
  compute_cost?: number;
  llm_cost: number;
  storage_cost: number;
  payment_fees: number;
  network_cost: number;
  bandwidth_savings: number;
  cache_hit_rate: number;
  cost_per_1k_tokens: number;
  period_start: string;
  period_end: string;
  trend: DailyCost[];
  agent_costs?: { agent_id: string; cost_cents: number; }[];
  department_tier_usage?: DepartmentTierUsage;
}

interface DepartmentTierUsage {
  current_plan: string;
  period: string;
  departments: DepartmentTierUsageRow[];
}

interface DepartmentTierUsageRow {
  id: string;
  department_type: string;
  agent_id: string;
  actions_used: number;
  action_limit: number | null;
  usage_percent: number | null;
  soft_limit_reached: boolean;
}

export default function CostDashboardPage() {
  const router = useRouter();
  const [data, setData] = useState<CostDashboardData | null>(null);
  const [myPlanData, setMyPlanData] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchCostData() {
      try {
        const token = localStorage.getItem('token');
        const headers = { 'Authorization': `Bearer ${token}` };

        const [costRes, planRes] = await Promise.all([
          fetch('/api/billing/cost-dashboard', { headers }),
          fetch('/api/billing/my-plan', { headers })
        ]);

        if (costRes.ok) {
            const result = await costRes.json();
            setData(result);
        } else {
            console.error("Failed to fetch cost data:", costRes.status);
        }

        if (planRes.ok) {
            const planResult = await planRes.json();
            setMyPlanData(planResult);
        } else {
            console.error("Failed to fetch plan data:", planRes.status);
        }
      } catch (err) {
        console.error("Error fetching cost data", err);
      } finally {
        setLoading(false);
      }
    }
    fetchCostData();
  }, []);

  if (loading) {
      return (
        <div className="min-h-screen flex items-center justify-center bg-[#f4f6f8]">
          <div className="flex flex-col items-center gap-4">
            <div className="w-12 h-12 border-4 border-indigo-200 border-t-indigo-600 rounded-full animate-spin" />
            <p className="text-sm font-bold text-gray-500 font-outfit uppercase tracking-widest">Gathering Cost Signals...</p>
          </div>
        </div>
      );
  }

  const formatCurrency = (cents: number) => {
      return '$' + (cents / 100).toFixed(2);
  };

  const formatStorage = (bytes: number) => {
      const mb = bytes / (1024 * 1024);
      if (mb < 1) return "< 1 MB";
      if (mb >= 1024) return parseFloat((mb / 1024).toFixed(2)) + " GB";
      return parseFloat(mb.toFixed(1)) + " MB";
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#f4f6f8] text-[#1D1D1F]">
      <header className="px-4 md:px-8 py-5 flex items-center justify-between sticky top-0 z-50 bg-white/75 backdrop-blur-3xl saturate-150 border-b border-gray-200/40 shadow-sm">
        <div className="flex items-center gap-4">
          <div className="w-10 h-10 bg-gray-900 rounded-xl flex items-center justify-center text-white font-bold shadow-lg">A</div>
          <h1 className="text-xl font-extrabold font-outfit tracking-tight text-gray-900">
            Advisory Dashboard
          </h1>
        </div>
        <button
          onClick={() => router.push('/plan')}
          className="min-w-[44px] h-10 px-4 bg-white hover:bg-gray-50 border border-gray-200 rounded-xl text-sm font-bold transition-all active:scale-95 shadow-sm flex items-center gap-2"
        >
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
          </svg>
          Account Plan
        </button>
      </header>

      <main id="cost-dashboard-screen" className="p-4 md:p-10 flex-1 max-w-[1600px] mx-auto w-full grid grid-cols-1 lg:grid-cols-12 gap-8">

        {/* Left Column: Metrics & Breakdown */}
        <div className="lg:col-span-8 flex flex-col gap-8">

          {/* Summary Banner */}
          <div className="bg-indigo-600 rounded-[32px] p-8 text-white shadow-xl shadow-indigo-200 flex flex-col md:flex-row items-center justify-between gap-8 relative overflow-hidden group">
            <div className="absolute top-0 right-0 w-64 h-64 bg-white/10 rounded-full blur-3xl -mr-32 -mt-32 transition-transform duration-700 group-hover:scale-110" />
            <div className="relative z-10">
              <h2 className="text-indigo-100 font-bold uppercase tracking-widest text-xs mb-2">Monthly Economic Health</h2>
              <p className="text-4xl font-black font-outfit tracking-tighter">
                {data?.total_revenue ? (data.total_revenue > data.total_costs ? 'Business in Surplus' : 'Managing Growth') : 'Observing Trends'}
              </p>
              <div className="flex items-center gap-6 mt-6">
                <div>
                  <p className="text-indigo-200 text-xs font-bold uppercase tracking-wider">Total Revenue</p>
                  <p className="text-2xl font-bold">{formatCurrency(data?.total_revenue || 0)}</p>
                </div>
                <div className="w-px h-10 bg-indigo-400/30" />
                <div>
                  <p className="text-indigo-200 text-xs font-bold uppercase tracking-wider">Total Costs</p>
                  <p className="text-2xl font-bold">{formatCurrency(data?.total_costs || 0)}</p>
                </div>
              </div>
            </div>
            <div className="bg-white/10 backdrop-blur-xl rounded-3xl p-6 border border-white/10 min-w-[240px]">
              <p className="text-indigo-100 text-xs font-bold uppercase tracking-wider mb-1">Projected End of Month</p>
              <p className="text-3xl font-black font-outfit">{formatCurrency(data?.projected_monthly_cost || 0)}</p>
              <p className="text-[10px] text-indigo-200 mt-2 font-medium">Based on trailing 7-day usage velocity</p>
            </div>
          </div>

          {/* Activity Trend */}
          <div className="bg-white border border-gray-200/60 shadow-sm rounded-[32px] p-8">
            <div className="flex items-center justify-between mb-8">
              <h3 className="text-lg font-extrabold font-outfit text-gray-900">Resource Consumption Trend</h3>
              <div className="px-3 py-1 bg-gray-100 rounded-full text-[10px] font-black uppercase tracking-tighter text-gray-500">Trailing 7 Days</div>
            </div>

            <div className="h-48 flex items-end gap-3 md:gap-6" id="cost-dashboard-trend">
              {data?.trend && data.trend.length > 0 ? (
                data.trend.map((daily, index) => {
                  const maxCost = Math.max(...data.trend.map(d => d.total_cost), 1);
                  const heightPercent = Math.max((daily.total_cost / maxCost) * 100, 4);
                  return (
                    <div key={index} className="flex-1 flex flex-col items-center group relative h-full">
                      <div className="w-full bg-gray-50 rounded-2xl relative flex items-end justify-center h-full overflow-hidden transition-colors hover:bg-gray-100">
                        <div className="w-full bg-indigo-500 transition-all duration-700 ease-out group-hover:bg-indigo-600 rounded-t-lg" style={{ height: `${heightPercent}%` }} />
                        {/* Tooltip */}
                        <div className="absolute bottom-full mb-3 left-1/2 -translate-x-1/2 bg-gray-900 text-white text-[10px] font-bold py-1.5 px-3 rounded-xl opacity-0 group-hover:opacity-100 transition-all duration-200 scale-90 group-hover:scale-100 whitespace-nowrap pointer-events-none z-20 shadow-xl">
                          {formatCurrency(daily.total_cost)}
                        </div>
                      </div>
                      <span className="text-[10px] text-gray-400 font-bold mt-3 uppercase tracking-tighter">{daily.date.split('-').slice(2)}</span>
                    </div>
                  );
                })
              ) : (
                <div className="w-full h-full flex items-center justify-center border-2 border-dashed border-gray-100 rounded-2xl text-gray-300 font-bold text-sm italic">Insufficient trend data available</div>
              )}
            </div>
          </div>

          {/* Efficiency & Detailed Breakdown */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            {/* LLM Efficiency */}
            <div className="bg-white border border-gray-200/60 shadow-sm rounded-[32px] p-8 flex flex-col">
              <div className="flex items-center gap-3 mb-6">
                <div className="w-8 h-8 bg-blue-50 text-blue-600 rounded-lg flex items-center justify-center">
                  <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M13 10V3L4 14h7v7l9-11h-7z" />
                  </svg>
                </div>
                <h3 className="font-extrabold font-outfit text-gray-900">LLM Efficiency</h3>
              </div>

              <div className="flex-1 flex flex-col justify-center">
                <div className="flex items-baseline gap-2">
                  <span className="text-4xl font-black font-outfit text-gray-900">{data?.cache_hit_rate || 0}%</span>
                  <span className="text-xs font-bold text-gray-400 uppercase tracking-widest">Cache Hit Rate</span>
                </div>
                <div className="mt-4 w-full h-2.5 bg-gray-100 rounded-full overflow-hidden">
                  <div className="h-full bg-blue-500 rounded-full transition-all duration-1000" style={{ width: `${data?.cache_hit_rate || 0}%` }} />
                </div>
                <p className="mt-4 text-xs font-bold text-gray-500 leading-relaxed">
                  Your assistant is reusing memories effectively, reducing input costs by <span className="text-blue-600">${data?.cost_per_1k_tokens.toFixed(4)}</span> per 1k tokens.
                </p>
              </div>
            </div>

            {/* Storage Savings */}
            <div className="bg-white border border-gray-200/60 shadow-sm rounded-[32px] p-8 flex flex-col">
              <div className="flex items-center gap-3 mb-6">
                <div className="w-8 h-8 bg-green-50 text-green-600 rounded-lg flex items-center justify-center">
                  <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
                  </svg>
                </div>
                <h3 className="font-extrabold font-outfit text-gray-900">Economic Gains</h3>
              </div>

              <div className="flex-1 flex flex-col justify-center">
                <div className="flex items-baseline gap-2">
                  <span className="text-4xl font-black font-outfit text-green-600">-{formatCurrency(data?.bandwidth_savings || 0)}</span>
                  <span className="text-xs font-bold text-gray-400 uppercase tracking-widest">Optimization Saved</span>
                </div>
                <p className="mt-4 text-xs font-bold text-gray-500 leading-relaxed">
                  Through automated WebP image conversion and lossless prompt minification, OHC has reduced your infrastructure bill.
                </p>
                <div className="mt-4 flex items-center gap-2 px-3 py-1.5 bg-green-50 rounded-xl self-start">
                  <span className="w-1.5 h-1.5 rounded-full bg-green-500" />
                  <span className="text-[10px] font-black text-green-700 uppercase tracking-tighter">Auto-compression active</span>
                </div>
              </div>
            </div>
          </div>

          {/* Agent Cost Table */}
          <div className="bg-white border border-gray-200/60 shadow-sm rounded-[32px] p-8">
            <h3 className="text-lg font-extrabold font-outfit text-gray-900 mb-6">Agent Resource Allocation</h3>
            <div className="overflow-hidden">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-gray-100">
                    <th className="text-left py-4 text-[10px] font-black text-gray-400 uppercase tracking-widest">Agent Identity</th>
                    <th className="text-right py-4 text-[10px] font-black text-gray-400 uppercase tracking-widest">Monthly Cost</th>
                  </tr>
                </thead>
                <tbody id="cost-dashboard-agent-costs">
                  {data?.agent_costs && data.agent_costs.length > 0 ? (
                    data.agent_costs.map((agent, index) => (
                      <tr key={index} className="border-b border-gray-50 last:border-0 transition-colors hover:bg-gray-50/50 group">
                        <td className="py-5 font-bold text-sm text-gray-700 capitalize flex items-center gap-3">
                          <div className="w-8 h-8 rounded-full bg-indigo-50 border border-indigo-100 flex items-center justify-center text-[10px] text-indigo-600 transition-transform group-hover:scale-110">
                            {agent.agent_id.charAt(0).toUpperCase()}
                          </div>
                          {agent.agent_id.replace(/_/g, ' ')}
                        </td>
                        <td className="py-5 text-right font-black text-sm text-gray-900">{formatCurrency(agent.cost_cents)}</td>
                      </tr>
                    ))
                  ) : (
                    <tr>
                      <td colSpan={2} className="py-10 text-center text-xs font-bold text-gray-300 italic">No agent missions recorded this period</td>
                    </tr>
                  )}
                </tbody>
              </table>
            </div>
          </div>
        </div>

        {/* Right Column: Plan & Quotas */}
        <div className="lg:col-span-4 flex flex-col gap-8">

          {/* Current Plan Card */}
          <div className="bg-white border border-gray-200/60 shadow-sm rounded-[32px] p-8">
            <h3 className="text-xs font-black text-gray-400 uppercase tracking-widest mb-6">Connected Plan</h3>
            <div className="flex items-center justify-between mb-8">
              <p className="text-4xl font-black font-outfit text-gray-900">{myPlanData?.current_plan || 'Free'}</p>
              <div className="px-4 py-1.5 bg-green-50 rounded-2xl text-[10px] font-black text-green-700 uppercase tracking-tighter">Active</div>
            </div>

            <div className="space-y-8">
              {/* AI actions quota */}
              <div>
                <div className="flex justify-between items-center mb-3">
                  <span className="text-xs font-black text-gray-500 uppercase tracking-tighter">AI Operations</span>
                  <span className="text-xs font-black text-gray-900">
                    {myPlanData?.ai_actions_used} / {myPlanData?.ai_actions_limit || '∞'}
                  </span>
                </div>
                <div className="w-full h-2 bg-gray-100 rounded-full overflow-hidden">
                  <div
                    className={`h-full transition-all duration-1000 ${myPlanData?.ai_actions_used > (myPlanData?.ai_actions_limit || 0) * 0.9 ? 'bg-amber-500' : 'bg-indigo-600'}`}
                    style={{ width: `${Math.min((myPlanData?.ai_actions_used / (myPlanData?.ai_actions_limit || 1)) * 100, 100)}%` }}
                  />
                </div>
              </div>

              {/* Storage quota */}
              <div>
                <div className="flex justify-between items-center mb-3">
                  <span className="text-xs font-black text-gray-500 uppercase tracking-tighter">Cloud Vault Storage</span>
                  <span className="text-xs font-black text-gray-900">
                    {formatStorage(myPlanData?.storage_used_bytes || 0)} / {myPlanData?.storage_limit_bytes ? formatStorage(myPlanData.storage_limit_bytes) : '∞'}
                  </span>
                </div>
                <div className="w-full h-2 bg-gray-100 rounded-full overflow-hidden">
                  <div
                    className="h-full bg-indigo-600 transition-all duration-1000"
                    style={{ width: `${Math.min(((myPlanData?.storage_used_bytes || 0) / (myPlanData?.storage_limit_bytes || 1)) * 100, 100)}%` }}
                  />
                </div>
              </div>
            </div>

            <button
              onClick={() => router.push('/pricing')}
              className="w-full mt-10 h-12 bg-gray-900 hover:bg-black text-white rounded-2xl text-sm font-bold shadow-lg transition-all active:scale-95"
            >
              Upgrade Capabilities
            </button>
          </div>

          {/* Department Breakdown */}
          <div className="bg-white border border-gray-200/60 shadow-sm rounded-[32px] p-8 flex-1">
            <div className="flex items-center justify-between mb-8">
              <h3 className="text-lg font-extrabold font-outfit text-gray-900">Departments</h3>
              <span className="text-[10px] font-black text-gray-400 uppercase tracking-widest">{data?.department_tier_usage?.period || 'Current'}</span>
            </div>

            <div className="space-y-6" id="department-tier-usage-list">
              {data?.department_tier_usage?.departments?.length ? (
                data.department_tier_usage.departments.map((dept) => (
                  <div key={dept.id} className="group">
                    <div className="flex justify-between items-start mb-3">
                      <div>
                        <p className="text-sm font-bold text-gray-900 capitalize">{dept.department_type}</p>
                        <p className="text-[10px] font-bold text-gray-400 uppercase tracking-tighter">{dept.actions_used} actions</p>
                      </div>
                      {dept.soft_limit_reached && (
                        <div className="p-1 bg-amber-50 text-amber-600 rounded-lg">
                          <svg className="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                            <path fillRule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
                          </svg>
                        </div>
                      )}
                    </div>
                    <div className="w-full h-1.5 bg-gray-50 rounded-full overflow-hidden">
                      <div
                        className={`h-full transition-all duration-700 ${dept.soft_limit_reached ? 'bg-amber-500' : 'bg-indigo-300'}`}
                        style={{ width: `${dept.usage_percent || 0}%` }}
                      />
                    </div>
                  </div>
                ))
              ) : (
                <div className="py-20 text-center">
                  <p className="text-xs font-bold text-gray-300 uppercase tracking-widest italic">No operational data</p>
                </div>
              )}
            </div>
          </div>
        </div>

      </main>

      <footer className="p-10 flex flex-col items-center gap-6 border-t border-gray-100 bg-white/50">
        <PoweredByOHC tenantId="ohc" />
        <p className="text-[10px] font-black text-gray-400 uppercase tracking-widest text-center">
          Enterprise economic advisory &bull; One Human Corp Internal
        </p>
      </footer>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800;900&family=Outfit:wght@500;600;700;800;900&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
