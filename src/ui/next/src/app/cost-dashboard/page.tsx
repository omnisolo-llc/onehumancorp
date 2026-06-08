"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

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
      return <div className="min-h-screen flex items-center justify-center">Loading...</div>;
  }

  const formatCurrency = (cents: number) => {
      return '$' + (cents / 100).toFixed(2);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-slate-50 text-slate-900 selection:bg-indigo-100">
      <header className="px-4 md:px-8 py-4 flex flex-col md:flex-row items-center justify-between border-b gap-4 sticky top-0 z-50 bg-white/60 backdrop-blur-3xl saturate-150 border-b-white/20 shadow-[0_1px_2px_rgba(0,0,0,0.02)]">
        <h1 className="text-2xl font-bold font-outfit text-center md:text-left text-slate-900 tracking-tight">Business Advisory Dashboard</h1>
        <div className="flex gap-3">
            <button onClick={() => router.push('/plan')} className="min-w-[140px] min-h-[44px] px-6 py-2 bg-slate-200/50 hover:bg-slate-200 text-slate-800 rounded-2xl text-sm font-semibold transition-all active:scale-[0.98] border border-white/40 flex items-center justify-center">
            Back to Plan
            </button>
        </div>
      </header>

      <main id="cost-dashboard-screen" className="p-4 md:p-10 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-8">

        <section className="p-8 md:p-10 shadow-2xl shadow-indigo-500/5 bg-white/40 backdrop-blur-2xl border border-white/60 rounded-[32px] w-full">
            <div className="flex items-center gap-4 mb-6">
                <div className="w-12 h-12 rounded-2xl bg-indigo-50 flex items-center justify-center shadow-inner">
                    <span className="text-2xl">💡</span>
                </div>
                <div>
                    <h2 className="text-xl font-bold font-outfit text-slate-900">Advisory Summary</h2>
                    <p className="text-sm text-slate-500 font-medium">Insights based on connected backend billing and usage signals.</p>
                </div>
            </div>
            <p className="text-slate-700 font-medium leading-relaxed bg-indigo-50/30 p-6 rounded-2xl border border-indigo-100/50">
                Cost and tier usage are tracked in real-time. OHC Miser optimizes your token footprint and storage automatically to ensure maximum business efficiency.
            </p>
        </section>

        {/* My Plan Section */}
        <section id="my-plan-section" className="p-8 md:p-10 shadow-2xl shadow-indigo-500/5 bg-white/40 backdrop-blur-2xl border border-white/60 rounded-[32px] hover:border-white transition-all duration-500">
          <div className="flex justify-between items-center mb-8">
             <h2 className="text-2xl font-bold font-outfit text-slate-900">Subscription Status</h2>
             <button
               onClick={() => router.push('/pricing')}
               className="min-h-[44px] px-6 py-2 bg-indigo-600 hover:bg-indigo-500 text-white rounded-2xl text-sm font-bold transition-all shadow-lg shadow-indigo-600/20 active:scale-[0.98]">
               Upgrade
             </button>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
              <div className="p-6 rounded-[24px] bg-white/50 border border-white/50 shadow-sm">
                  <h3 className="text-xs font-bold text-slate-400 uppercase tracking-widest">Active Plan</h3>
                  <p className="text-3xl font-extrabold text-slate-900 mt-2 font-outfit">{myPlanData?.current_plan || 'Free'}</p>
              </div>
              <div className="p-6 rounded-[24px] bg-white/50 border border-white/50 shadow-sm">
                  <h3 className="text-xs font-bold text-slate-400 uppercase tracking-widest">AI Capacity</h3>
                  <p className="text-3xl font-extrabold text-slate-900 mt-2 font-outfit">{myPlanData?.ai_actions_used || 0} <span className="text-sm text-slate-400 font-bold">{myPlanData?.ai_actions_limit != null ? `/ ${myPlanData.ai_actions_limit}` : '/ ∞'}</span></p>
              </div>
              <div className="p-6 rounded-[24px] bg-white/50 border border-white/50 shadow-sm">
                  <h3 className="text-xs font-bold text-slate-400 uppercase tracking-widest">Cloud Assets</h3>
                  <p className="text-3xl font-extrabold text-slate-900 mt-2 font-outfit">{((myPlanData?.storage_used_bytes || 0) / (1024 * 1024)).toFixed(1)} <span className="text-sm text-slate-400 font-bold">MB</span></p>
              </div>
              <div className="p-6 rounded-[24px] bg-white/50 border border-white/50 shadow-sm">
                  <h3 className="text-xs font-bold text-slate-400 uppercase tracking-widest">Next Bill</h3>
                  <p className="text-3xl font-extrabold text-slate-900 mt-2 font-outfit">{formatCurrency(myPlanData?.next_bill_estimated || 0)}</p>
              </div>
          </div>
        </section>

        {/* Overview Section */}
        <section className="p-8 md:p-10 shadow-2xl shadow-indigo-500/5 bg-white/40 backdrop-blur-2xl border border-white/60 rounded-[32px] w-full">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8">
               <h2 className="text-2xl font-bold font-outfit text-slate-900">Cost Transparency</h2>
               <span id="cost-dashboard-period" className="text-xs font-bold text-slate-400 bg-slate-200/50 px-4 py-2 rounded-full uppercase tracking-wider">{data?.period_start} — {data?.period_end}</span>
            </div>

            <div className="">
                <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
                    <div className="p-8 rounded-[28px] bg-slate-900 text-white shadow-xl shadow-slate-900/20 hover:-translate-y-1 transition-all duration-300">
                        <h2 className="text-xs font-bold text-slate-400 uppercase tracking-widest mb-3">Total Investment</h2>
                        <p id="cost-dashboard-total" className="text-4xl font-extrabold font-outfit">{formatCurrency(data?.total_costs || 0)}</p>
                    </div>
                    <div className="p-8 rounded-[28px] bg-white/60 backdrop-blur-xl border border-white hover:-translate-y-1 transition-all duration-300">
                        <h2 className="text-xs font-bold text-slate-400 uppercase tracking-widest mb-3">Gross Revenue</h2>
                        <p id="cost-dashboard-revenue" className="text-4xl font-extrabold font-outfit text-emerald-600">{formatCurrency(data?.total_revenue || 0)}</p>
                    </div>
                    <div className="p-8 rounded-[28px] bg-emerald-50/50 border border-emerald-100 hover:-translate-y-1 transition-all duration-300">
                        <h2 className="text-xs font-bold text-emerald-600 uppercase tracking-widest mb-3">Miser Savings</h2>
                        <p id="cost-dashboard-total-savings" className="text-4xl font-extrabold font-outfit text-emerald-700">{formatCurrency((data?.bandwidth_savings || 0))}</p>
                        <p className="text-[10px] font-bold text-emerald-600 mt-3 uppercase tracking-wider">Optimized via auto-WebP</p>
                    </div>
                </div>
            </div>
        </section>

        {/* Breakdown Section */}
        <section className="p-8 md:p-10 shadow-2xl shadow-indigo-500/5 bg-white/40 backdrop-blur-2xl border border-white/60 rounded-[32px] w-full">
            <div className="app-panel-header mb-8">
                <h2 className="text-2xl font-bold font-outfit text-slate-900">Cost Breakdown</h2>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
                <div className="flex flex-col p-8 rounded-[28px] bg-white/50 border border-white shadow-sm hover:shadow-md transition-all duration-300">
                    <h3 className="text-xs font-bold text-slate-400 uppercase tracking-widest mb-6">7-Day Expenditure Trend</h3>
                    <ul id="cost-dashboard-trend" className="space-y-3">
                        {(data?.trend?.length ? data.trend : [{ date: 'No trend data yet', total_cost: 0 } as DailyCost]).map((daily, index) => (
                            <li key={index} className="flex justify-between items-center bg-slate-50/50 p-3 rounded-xl border border-slate-100/50">
                                <span className="text-sm font-semibold text-slate-600">{daily.date}</span>
                                <span className="text-sm font-extrabold text-slate-900">{formatCurrency(daily.total_cost)}</span>
                            </li>
                        ))}
                    </ul>
                </div>

                <div className="flex flex-col p-8 rounded-[28px] bg-white/50 border border-white shadow-sm hover:shadow-md transition-all duration-300">
                    <h3 className="text-xs font-bold text-slate-400 uppercase tracking-widest mb-6">Agent & Feature Costs</h3>
                    {data?.agent_costs && data.agent_costs.length > 0 ? (
                        <ul id="cost-dashboard-agent-costs" className="space-y-3">
                            {data.agent_costs.map((agent, index) => (
                                <li key={index} className="flex justify-between items-center bg-indigo-50/30 p-3 rounded-xl border border-indigo-100/30">
                                    <span className="text-sm font-semibold text-slate-700 capitalize">{agent.agent_id.replace(/_/g, ' ')}</span>
                                    <span className="text-sm font-extrabold text-indigo-700">{formatCurrency(agent.cost_cents)}</span>
                                </li>
                            ))}
                        </ul>
                    ) : (
                        <div className="flex flex-col items-center justify-center py-10">
                            <span className="text-4xl mb-3 opacity-20">🤖</span>
                            <p className="text-sm text-slate-400 font-bold uppercase tracking-wider">No agent activity recorded</p>
                        </div>
                    )}
                </div>
            </div>

            <div className="mt-8 space-y-4">
                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 p-8 rounded-[28px] bg-white/50 border border-white hover:border-indigo-100 transition-all">
                    <div>
                        <span className="text-lg font-bold text-slate-900">Intelligence (LLM)</span>
                        <p className="text-sm text-slate-500 mt-1 font-medium">Core brain operations and reasoning logic.</p>
                    </div>
                    <div className="text-left sm:text-right w-full sm:w-auto">
                        <span id="cost-dashboard-llm" className="text-2xl font-extrabold text-slate-900 block">{formatCurrency(data?.llm_cost || 0)}</span>
                        <div className="flex gap-2 mt-2">
                           <span className="text-[10px] font-bold text-indigo-600 bg-indigo-50 px-2 py-1 rounded uppercase tracking-wider">Efficiency: {data?.cache_hit_rate}% cache hit</span>
                           <span className="text-[10px] font-bold text-slate-500 bg-slate-100 px-2 py-1 rounded uppercase tracking-wider">${data?.cost_per_1k_tokens.toFixed(4)}/1k</span>
                        </div>
                    </div>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 p-8 rounded-[28px] bg-white/50 border border-white hover:border-indigo-100 transition-all">
                    <div>
                        <span className="text-lg font-bold text-slate-900">Cloud Persistence</span>
                        <p className="text-sm text-slate-500 mt-1 font-medium">High-speed file storage and asset hosting.</p>
                    </div>
                    <span id="cost-dashboard-storage" className="text-2xl font-extrabold text-slate-900">{formatCurrency(data?.storage_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 p-8 rounded-[28px] bg-white/50 border border-white hover:border-indigo-100 transition-all">
                    <div>
                        <span className="text-lg font-bold text-slate-900">Transaction Fees</span>
                        <p className="text-sm text-slate-500 mt-1 font-medium">Stripe ecosystem routing and processing.</p>
                    </div>
                    <span id="cost-dashboard-payment-fees" className="text-2xl font-extrabold text-slate-900">{formatCurrency(data?.payment_fees || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 p-8 rounded-[28px] bg-white/50 border border-white hover:border-indigo-100 transition-all">
                    <div>
                        <span className="text-lg font-bold text-slate-900">Infrastructure</span>
                        <p className="text-sm text-slate-500 mt-1 font-medium">Compute hours and background task workers.</p>
                    </div>
                    <span id="cost-dashboard-compute" className="text-2xl font-extrabold text-slate-900">{formatCurrency(data?.compute_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 p-8 rounded-[28px] bg-emerald-50/20 border border-emerald-100/50 hover:bg-emerald-50/40 transition-all">
                    <div>
                        <span className="text-lg font-bold text-emerald-700">Miser Efficiency Gains</span>
                        <p className="text-sm text-emerald-600 mt-1 font-medium">Savings from automated WebP compression and token pruning.</p>
                    </div>
                    <span id="cost-dashboard-bandwidth-savings" className="text-2xl font-extrabold text-emerald-700">-{formatCurrency(data?.bandwidth_savings || 0)}</span>
                </div>
            </div>
        </section>

        <section className="p-8 md:p-10 shadow-2xl shadow-indigo-500/5 bg-white/40 backdrop-blur-2xl border border-white/60 rounded-[32px] hover:border-white transition-all duration-500">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 mb-6">
                <h2 className="text-xl font-bold font-outfit text-gray-900">Department Tier Usage</h2>
                <span className="text-sm text-gray-500 font-medium">
                  {data?.department_tier_usage?.current_plan || 'Free'} plan · {data?.department_tier_usage?.period || data?.period_end?.slice(0, 7) || ''}
                </span>
            </div>

            {data?.department_tier_usage?.departments?.length ? (
                <div className="space-y-4" id="department-tier-usage-list">
                    {data.department_tier_usage.departments.map((department) => (
                        <div key={department.id} className="p-5 rounded-2xl shadow-sm bg-white/50 backdrop-blur-lg border border-white/50">
                            <div className="flex flex-col sm:flex-row sm:items-start justify-between gap-3">
                                <div>
                                    <h3 className="font-semibold text-gray-900">{department.department_type}</h3>
                                    <p className="text-sm text-gray-500 mt-1">{department.agent_id}</p>
                                </div>
                                <div className="text-left sm:text-right">
                                    <p className="font-semibold text-gray-900">
                                      {department.action_limit === null
                                        ? `${department.actions_used} actions`
                                        : `${department.actions_used} / ${department.action_limit} actions`}
                                    </p>
                                    {department.soft_limit_reached ? (
                                      <p className="text-sm text-amber-700 font-medium mt-1">Tier limit reached</p>
                                    ) : null}
                                </div>
                            </div>
                            {department.usage_percent !== null ? (
                                <div className="mt-4 h-2 rounded-full bg-gray-200 overflow-hidden" aria-label={`${department.department_type} usage`}>
                                    <div
                                      className={department.soft_limit_reached ? "h-full bg-amber-500" : "h-full bg-indigo-500"}
                                      style={{ width: `${department.usage_percent}%` }}
                                    />
                                </div>
                            ) : null}
                        </div>
                    ))}
                </div>
            ) : (
                <p className="text-sm text-gray-500" id="department-tier-usage-empty">No department usage recorded for this period.</p>
            )}
        </section>

      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
