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
  email_cost: number;
  api_cost: number;
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
          <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 text-gray-900 w-full overflow-x-hidden p-4 md:p-8" data-testid="cost-dashboard-loading">
              <div className="max-w-6xl mx-auto w-full flex flex-col gap-6 animate-pulse">
                  <div className="h-10 bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl w-1/4"></div>
                  <div className="h-48 bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-2xl w-full"></div>
                  <div className="h-64 bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-2xl w-full"></div>
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
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 text-gray-900">
      <header className="px-4 md:px-6 py-4 flex flex-col md:flex-row items-center justify-between border-b gap-4 sticky top-0 z-50 bg-white/70 backdrop-blur-xl saturate-200 border-b-white/40 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-center md:text-left text-gray-900 tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-gray-900 to-gray-600">Cost Transparency Dashboard</h1>
        <div className="flex gap-2">
            <button onClick={() => router.push('/plan')} className="min-w-[44px] min-h-[44px] px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-800 rounded-xl text-sm font-medium transition-all active:scale-95 shadow-sm flex items-center justify-center">
            Back to My Plan
            </button>
        </div>
      </header>

      <main id="cost-dashboard-screen" className="p-4 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">

        <section className="app-panel app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 hover:shadow-xl transition-shadow duration-300 rounded-2xl">
            <div className="app-panel-header px-6 py-4 border-b border-white/40 bg-transparent">
                <h2 className="app-panel-title text-xl font-bold font-outfit text-gray-900">Advisory Summary</h2>
            </div>
            <div className="app-panel-body p-6">
                <p className="text-gray-700 font-medium leading-relaxed">
                  Cost and tier usage are based on connected backend billing, storage, network, and agent department usage signals.
                </p>
            </div>
        </section>

        {/* My Plan Section */}
        <section id="my-plan-section" className="app-panel bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-2xl shadow-sm">
          <div className="app-panel-header flex justify-between items-center bg-transparent border-b border-white/40">
             <h2 className="app-panel-title text-xl font-bold font-outfit text-gray-900">My Plan</h2>
             <button
               onClick={() => router.push('/pricing')}
               className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-xl text-sm font-medium transition-all shadow-sm">
               Upgrade
             </button>
          </div>
          <div className="app-panel-body">
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                  <div className="p-4 rounded-xl app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40">
                      <h3 className="text-sm font-medium text-gray-500">Current Plan</h3>
                      <p className="text-2xl font-bold text-gray-900 mt-1">{myPlanData?.current_plan || 'Free'}</p>
                  </div>
                  <div className="p-4 rounded-xl app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40">
                      <h3 className="text-sm font-medium text-gray-500">AI actions used this month</h3>
                      <p className="text-2xl font-bold text-gray-900 mt-1">{myPlanData?.ai_actions_used || 0} <span className="text-sm text-gray-500 font-normal">{myPlanData?.ai_actions_limit != null ? `/ ${myPlanData.ai_actions_limit}` : '/ Unlimited'}</span></p>
                  </div>
                  <div className="p-4 rounded-xl app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40">
                      <h3 className="text-sm font-medium text-gray-500">Storage used</h3>
                      <p className="text-2xl font-bold text-gray-900 mt-1">{formatStorage(myPlanData?.storage_used_bytes || 0)} <span className="text-sm text-gray-500 font-normal">{myPlanData?.storage_limit_bytes != null ? `/ ${formatStorage(myPlanData.storage_limit_bytes)}` : '/ Unlimited'}</span></p>
                  </div>
                  <div className="p-4 rounded-xl app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40">
                      <h3 className="text-sm font-medium text-gray-500">Estimated Next Bill</h3>
                      <p className="text-2xl font-bold text-gray-900 mt-1">{formatCurrency(myPlanData?.next_bill_estimated || 0)}</p>
                  </div>
              </div>
          </div>
        </section>

        {/* Overview Section */}
        <section className="app-panel app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 hover:shadow-xl transition-shadow duration-300 rounded-2xl">
            <div className="app-panel-header flex justify-between items-center px-6 py-4 border-b border-white/40 bg-transparent">
               <h2 className="app-panel-title text-xl font-bold font-outfit text-gray-900">Cost Transparency</h2>
               <span id="cost-dashboard-period" className="text-sm text-gray-500 font-medium">Period: {data?.period_start} to {data?.period_end}</span>
            </div>

            <div className="app-panel-body p-6">
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <div className="app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300 group">
                        <h2 className="text-sm font-medium text-gray-500 mb-1">Total Costs</h2>
                        <p id="cost-dashboard-total" className="text-3xl font-bold font-outfit text-gray-900">{formatCurrency(data?.total_costs || 0)}</p>
                    </div>
                    <div className="app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300 group">
                        <h2 className="text-sm font-medium text-gray-500 mb-1">Projected Monthly Cost</h2>
                        <p id="cost-dashboard-projected" className="text-3xl font-bold font-outfit text-indigo-600">{formatCurrency(data?.projected_monthly_cost || 0)}</p>
                    </div>
                    <div className="app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300 group">
                        <h2 className="text-sm font-medium text-gray-500 mb-1">Total Revenue</h2>
                        <p id="cost-dashboard-revenue" className="text-3xl font-bold font-outfit text-green-600">{formatCurrency(data?.total_revenue || 0)}</p>
                    </div>
                    <div className="app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300 group">
                        <h2 className="text-sm font-medium text-green-700 mb-1">Network & Storage Savings</h2>
                        <p id="cost-dashboard-total-savings" className="text-3xl font-bold font-outfit text-green-700">{formatCurrency((data?.bandwidth_savings || 0))}</p>
                        <p className="text-xs text-green-600 mt-2">Saved via auto-compression</p>
                    </div>
                </div>
            </div>
        </section>

        {/* Budget Health Alert */}
        {data && data.projected_monthly_cost > 10000 && (
            <div id="budget-health-alert" className="p-4 bg-amber-50/70 border border-amber-200 backdrop-blur-xl saturate-200 rounded-xl flex items-start gap-3">
                <svg className="w-5 h-5 text-amber-600 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                </svg>
                <div>
                    <h3 className="text-sm font-semibold text-amber-800">Budget Health Warning</h3>
                    <p className="text-sm text-amber-700 mt-1">Your projected monthly cost ({formatCurrency(data.projected_monthly_cost)}) is exceeding your typical baseline. Consider reviewing your agent usage or upgrading your tier for better bulk rates.</p>
                </div>
            </div>
        )}

        {/* Breakdown Section */}
        <section className="app-panel app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 hover:shadow-xl transition-shadow duration-300 rounded-2xl">
            <div className="app-panel-header px-6 py-4 border-b border-white/40 bg-transparent">
                <h2 className="app-panel-title text-xl font-bold font-outfit text-gray-900">Cost Breakdown</h2>
            </div>

            <div className="app-panel-body p-6 space-y-4">
                <div className="flex flex-col app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <h3 className="font-medium text-gray-900 mb-4">7-Day Trend</h3>
                    {data?.trend && data.trend.length > 0 ? (
                        <div className="flex items-end h-32 gap-2 mt-4" id="cost-dashboard-trend">
                            {data.trend.map((daily, index) => {
                                const maxCost = Math.max(...data.trend.map(d => d.total_cost), 1);
                                const heightPercent = Math.max((daily.total_cost / maxCost) * 100, 5);
                                return (
                                    <div key={index} className="flex-1 flex flex-col items-center gap-2 group">
                                        <div className="w-full bg-indigo-50/50 rounded-t-md relative flex items-end justify-center group-hover:bg-indigo-100 transition-colors" style={{ height: '100px' }}>
                                            <div className="w-full bg-indigo-500 rounded-t-md transition-all duration-500 group-hover:bg-indigo-600" style={{ height: `${heightPercent}%` }}></div>
                                            <div className="absolute -top-8 bg-gray-900 text-white text-xs py-1 px-2 rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none z-10 shadow-lg">
                                                {formatCurrency(daily.total_cost)}
                                            </div>
                                        </div>
                                        <span className="text-xs text-gray-500 font-medium whitespace-nowrap">{daily.date.split('-').slice(1).join('/')}</span>
                                    </div>
                                );
                            })}
                        </div>
                    ) : (
                        <p className="text-sm text-gray-500">No trend data yet.</p>
                    )}
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">LLM Usage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of AI agent actions and interactions.</p>
                    </div>
                    <div className="text-left sm:text-right w-full sm:w-auto">
                        <span id="cost-dashboard-llm" className="text-lg font-semibold text-gray-900 block">{formatCurrency(data?.llm_cost || 0)}</span>
                        <span className="text-xs text-gray-500 font-medium">Efficiency: {data?.cache_hit_rate}% cache hit rate, ${data?.cost_per_1k_tokens?.toFixed(4) || "0.0000"}/1k tokens</span>
                    </div>
                </div>

                {/* Per-Agent / Per-Feature Costs */}
                <div className="flex flex-col app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <h3 className="font-medium text-gray-900 mb-2">Agent & Feature Costs</h3>
                    {data?.agent_costs && data.agent_costs.length > 0 ? (
                        <ul id="cost-dashboard-agent-costs" className="space-y-2">
                            {data.agent_costs.map((agent, index) => (
                                <li key={index} className="flex justify-between items-center border-b border-gray-200 pb-2 last:border-b-0 last:pb-0">
                                    <span className="text-sm text-gray-700 capitalize">{agent.agent_id.replace(/_/g, ' ')}</span>
                                    <span className="text-sm font-medium text-gray-900">{formatCurrency(agent.cost_cents)}</span>
                                </li>
                            ))}
                        </ul>
                    ) : (
                        <p className="text-sm text-gray-500">No agent cost data available.</p>
                    )}
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">Storage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of cloud storage and file hosting.</p>
                    </div>
                    <span id="cost-dashboard-storage" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.storage_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">Payment Fees</span>
                        <p className="text-sm text-gray-500 mt-1">Stripe transaction fees on processed revenue.</p>
                    </div>
                    <span id="cost-dashboard-payment-fees" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.payment_fees || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">Compute Usage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of container execution and background processing.</p>
                    </div>
                    <span id="cost-dashboard-compute" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.compute_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">Network & Bandwidth</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of CDN delivery and outbound traffic.</p>
                    </div>
                    <span id="cost-dashboard-network" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.network_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">Email Sends</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of transactional and marketing email delivery.</p>
                    </div>
                    <span id="cost-dashboard-email" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.email_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">Outbound API Calls</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of third-party integration usage.</p>
                    </div>
                    <span id="cost-dashboard-api" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.api_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-green-700">Network & Storage Savings</span>
                        <p className="text-sm text-green-600 mt-1">Savings from automated WebP compression and minification.</p>
                    </div>
                    <span id="cost-dashboard-bandwidth-savings" className="text-lg font-semibold text-green-700">-{formatCurrency(data?.bandwidth_savings || 0)}</span>
                </div>
            </div>
        </section>

        <section className="p-6 md:p-8 app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 hover:shadow-xl transition-shadow duration-300 rounded-2xl">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 mb-6">
                <h2 className="text-xl font-bold font-outfit text-gray-900">Department Tier Usage</h2>
                <span className="text-sm text-gray-500 font-medium">
                  {data?.department_tier_usage?.current_plan ? `${data.department_tier_usage.current_plan} plan` : 'Loading...'} · {data?.department_tier_usage?.period || data?.period_end?.slice(0, 7) || ''}
                </span>
            </div>

            {data?.department_tier_usage?.departments?.length ? (
                <div className="space-y-4" id="department-tier-usage-list">
                    {data.department_tier_usage.departments.map((department) => (
                        <div key={department.id} className="p-5 rounded-2xl app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40">
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
