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
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 text-gray-900">
      <header className="px-4 md:px-6 py-4 flex flex-col md:flex-row items-center justify-between border-b gap-4 sticky top-0 z-50 bg-white/70 backdrop-blur-xl saturate-200 border-b-white/40 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-center md:text-left text-gray-900 tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-gray-900 to-gray-600">Business Advisory Dashboard</h1>
        <div className="flex gap-2">
            <button onClick={() => router.push('/plan')} className="min-w-[44px] min-h-[44px] px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-800 rounded-xl text-sm font-medium transition-all active:scale-95 shadow-sm flex items-center justify-center">
            Back to My Plan
            </button>
        </div>
      </header>

      <main id="cost-dashboard-screen" className="p-4 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">

        <section className="app-panel">
            <div className="app-panel-header">
                <h2 className="app-panel-title">Advisory Summary</h2>
            </div>
            <div className="app-panel-body">
                <p className="text-gray-700 font-medium leading-relaxed">
                  Cost and tier usage are based on connected backend billing, storage, network, and agent department usage signals.
                </p>
            </div>
        </section>

        {/* My Plan Section */}
        <section id="my-plan-section" className="p-6 md:p-8 shadow-lg bg-white/60 backdrop-blur-2xl saturate-200 border border-white/40 rounded-2xl md:rounded-[24px] hover:shadow-xl transition-shadow duration-300">
          <div className="flex justify-between items-center mb-6">
             <h2 className="text-xl font-bold font-outfit text-gray-900">My Plan</h2>
             <button
               onClick={() => router.push('/pricing')}
               className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-xl text-sm font-medium transition-all shadow-sm">
               Upgrade
             </button>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
              <div className="p-4 rounded-xl bg-white/50 border border-white/50">
                  <h3 className="text-sm font-medium text-gray-500">Current Plan</h3>
                  <p className="text-2xl font-bold text-gray-900 mt-1">{myPlanData?.current_plan || 'Free'}</p>
              </div>
              <div className="p-4 rounded-xl bg-white/50 border border-white/50">
                  <h3 className="text-sm font-medium text-gray-500">AI Actions Used</h3>
                  <p className="text-2xl font-bold text-gray-900 mt-1">{myPlanData?.ai_actions_used || 0} <span className="text-sm text-gray-500 font-normal">{myPlanData?.ai_actions_limit ? `/ ${myPlanData.ai_actions_limit}` : ''}</span></p>
              </div>
              <div className="p-4 rounded-xl bg-white/50 border border-white/50">
                  <h3 className="text-sm font-medium text-gray-500">Storage Used</h3>
                  <p className="text-2xl font-bold text-gray-900 mt-1">{((myPlanData?.storage_used_bytes || 0) / (1024 * 1024)).toFixed(1)} MB <span className="text-sm text-gray-500 font-normal">{myPlanData?.storage_limit_bytes ? `/ ${(myPlanData.storage_limit_bytes / (1024 * 1024)).toFixed(0)} MB` : ''}</span></p>
              </div>
              <div className="p-4 rounded-xl bg-white/50 border border-white/50">
                  <h3 className="text-sm font-medium text-gray-500">Estimated Next Bill</h3>
                  <p className="text-2xl font-bold text-gray-900 mt-1">{formatCurrency(myPlanData?.next_bill_estimated || 0)}</p>
              </div>
          </div>
        </section>

        {/* Overview Section */}
        <section className="app-panel">
            <div className="app-panel-header">
               <h2 className="app-panel-title">Cost Transparency</h2>
               <span id="cost-dashboard-period" className="text-sm text-gray-500 font-medium">Period: {data?.period_start} to {data?.period_end}</span>
            </div>

            <div className="app-panel-body">
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <div className="app-card hover:-translate-y-1 hover:shadow-md transition-all duration-300 group">
                        <h2 className="text-sm font-medium text-gray-500 mb-1">Total Costs</h2>
                        <p id="cost-dashboard-total" className="text-3xl font-bold font-outfit text-gray-900">{formatCurrency(data?.total_costs || 0)}</p>
                    </div>
                    <div className="app-card hover:-translate-y-1 hover:shadow-md transition-all duration-300 group">
                        <h2 className="text-sm font-medium text-gray-500 mb-1">Total Revenue</h2>
                        <p id="cost-dashboard-revenue" className="text-3xl font-bold font-outfit text-green-600">{formatCurrency(data?.total_revenue || 0)}</p>
                    </div>
                    <div className="app-card hover:-translate-y-1 hover:shadow-md transition-all duration-300 group">
                        <h2 className="text-sm font-medium text-green-700 mb-1">Network & Storage Savings</h2>
                        <p id="cost-dashboard-total-savings" className="text-3xl font-bold font-outfit text-green-700">{formatCurrency((data?.bandwidth_savings || 0))}</p>
                        <p className="text-xs text-green-600 mt-2">Saved via auto-compression</p>
                    </div>
                </div>
            </div>
        </section>

        {/* Breakdown Section */}
        <section className="app-panel">
            <div className="app-panel-header">
                <h2 className="app-panel-title">Cost Breakdown</h2>
            </div>

            <div className="app-panel-body space-y-4">
                <div className="flex flex-col app-card hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <h3 className="font-medium text-gray-900 mb-2">7-Day Trend</h3>
                    <ul id="cost-dashboard-trend" className="space-y-2">
                        {(data?.trend?.length ? data.trend : [{ date: 'No trend data yet', total_cost: 0 } as DailyCost]).map((daily, index) => (
                            <li key={index} className="flex justify-between items-center border-b border-gray-200 pb-2 last:border-b-0 last:pb-0">
                                <span className="text-sm text-gray-700">{daily.date}</span>
                                <span className="text-sm font-medium text-gray-900">{formatCurrency(daily.total_cost)}</span>
                            </li>
                        ))}
                    </ul>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">LLM Usage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of AI agent actions and interactions.</p>
                    </div>
                    <div className="text-left sm:text-right w-full sm:w-auto">
                        <span id="cost-dashboard-llm" className="text-lg font-semibold text-gray-900 block">{formatCurrency(data?.llm_cost || 0)}</span>
                        <span className="text-xs text-gray-500 font-medium">Efficiency: {data?.cache_hit_rate}% cache hit rate, ${data?.cost_per_1k_tokens.toFixed(4)}/1k tokens</span>
                    </div>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">Storage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of cloud storage and file hosting.</p>
                    </div>
                    <span id="cost-dashboard-storage" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.storage_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">Payment Fees</span>
                        <p className="text-sm text-gray-500 mt-1">Stripe transaction fees on processed revenue.</p>
                    </div>
                    <span id="cost-dashboard-payment-fees" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.payment_fees || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">Compute Usage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of container execution and background processing.</p>
                    </div>
                    <span id="cost-dashboard-compute" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.compute_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">Network & Bandwidth</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of CDN delivery and outbound traffic.</p>
                    </div>
                    <span id="cost-dashboard-network" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.network_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-green-700">Bandwidth Savings</span>
                        <p className="text-sm text-green-600 mt-1">Savings from automated WebP compression and minification.</p>
                    </div>
                    <span id="cost-dashboard-bandwidth-savings" className="text-lg font-semibold text-green-700">-{formatCurrency(data?.bandwidth_savings || 0)}</span>
                </div>
            </div>
        </section>

        <section className="p-6 md:p-8 shadow-lg bg-white/60 backdrop-blur-2xl saturate-200 border border-white/40 rounded-2xl md:rounded-[24px] hover:shadow-xl transition-shadow duration-300">
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
