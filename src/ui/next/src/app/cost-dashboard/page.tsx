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
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchCostData() {
      try {
        const token = localStorage.getItem('token');
        const headers: Record<string, string> = {
            'Content-Type': 'application/json'
        };
        if (token) {
            headers['Authorization'] = `Bearer ${token}`;
        }

        const res = await fetch('/api/billing/cost-dashboard', { headers });

        if (res.ok) {
            const result = await res.json();
            setData(result);
        } else {
            console.error("Failed to fetch cost data:", res.status);
        }
      } catch (err) {
        console.error("Error fetching cost data", err);
      } finally {
        setLoading(false);
      }
    }
    fetchCostData();
  }, []);

  const formatCurrency = (cents: number) => {
      return new Intl.NumberFormat('en-US', {
          style: 'currency',
          currency: 'USD',
      }).format(cents / 100);
  };

  const hasBudgetAlert = data?.department_tier_usage?.departments?.some(d => d.usage_percent && d.usage_percent >= 80) ?? false;

  if (loading) {
    return (
      <div className="flex justify-center items-center h-screen" data-testid="cost-dashboard-loading">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
      </div>
    );
  }

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
                        <p className="text-3xl font-bold font-outfit text-green-600">{formatCurrency(data?.total_revenue || 0)}</p>
                    </div>
                    <div className="app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300 group">
                        <h2 className="text-sm font-medium text-gray-500 mb-1">Gross Margin</h2>
                        <p className="text-3xl font-bold font-outfit text-gray-900">
                          {data?.total_revenue && data.total_revenue > 0 ? (((data.total_revenue - data.total_costs) / data.total_revenue) * 100).toFixed(1) + '%' : '0.0%'}
                        </p>
                    </div>
                </div>

                {hasBudgetAlert && (
                  <div className="mt-6 p-4 bg-amber-50 border border-amber-200 rounded-xl flex items-start gap-3">
                      <div className="text-amber-500 font-bold mt-0.5">!</div>
                      <div>
                          <h4 className="font-semibold text-amber-800">Budget Health Warning</h4>
                          <p className="text-sm text-amber-700 mt-1">One or more agent departments are exceeding 80% of their allocated tier limits. Review agent activities or upgrade your plan to avoid service interruption.</p>
                      </div>
                  </div>
                )}
            </div>
        </section>

        {/* Breakdown Section */}
        <section className="app-panel app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 hover:shadow-xl transition-shadow duration-300 rounded-2xl">
            <div className="app-panel-header px-6 py-4 border-b border-white/40 bg-transparent">
                <h2 className="app-panel-title text-xl font-bold font-outfit text-gray-900">Detailed Breakdown</h2>
            </div>
            <div className="app-panel-body p-6 space-y-6">

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">AI Tokens & Inference</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of running LLM models and embeddings.</p>
                    </div>
                    <div className="text-left sm:text-right">
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

        {/* 7-Day Trend Section */}
        <section className="p-6 md:p-8 app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 hover:shadow-xl transition-shadow duration-300 rounded-2xl">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">7-Day Trend</h2>
            {data?.trend && data.trend.length > 0 ? (
                <div className="overflow-x-auto pb-4">
                    <div className="min-w-[600px] flex items-end h-48 gap-2 relative">
                        {data.trend.map((day, i) => {
                            const maxCost = Math.max(...data.trend.map(d => d.total_cost));
                            const heightPercentage = maxCost > 0 ? (day.total_cost / maxCost) * 100 : 0;
                            return (
                                <div key={i} className="flex-1 flex flex-col justify-end items-center group relative h-full">
                                    <div
                                      className="w-full max-w-[40px] bg-indigo-500/80 hover:bg-indigo-600 rounded-t-sm transition-all"
                                      style={{ height: `${heightPercentage}%`, minHeight: '4px' }}
                                    />
                                    <span className="text-xs text-gray-500 mt-2 rotate-45 md:rotate-0 origin-left block w-full text-center truncate">
                                        {day.date.split('-').slice(1).join('/')}
                                    </span>

                                    {/* Tooltip */}
                                    <div className="opacity-0 group-hover:opacity-100 absolute bottom-full mb-2 bg-gray-900 text-white text-xs rounded-xl py-2 px-3 whitespace-nowrap pointer-events-none transition-opacity z-10 shadow-lg">
                                        <div className="font-medium mb-1 border-b border-gray-700 pb-1">{day.date}</div>
                                        <div className="grid grid-cols-2 gap-x-3 gap-y-1">
                                            <span className="text-gray-400">Total:</span> <span>{formatCurrency(day.total_cost)}</span>
                                            <span className="text-gray-400">LLM:</span> <span>{formatCurrency(day.llm_cost)}</span>
                                            <span className="text-gray-400">Storage:</span> <span>{formatCurrency(day.storage_cost)}</span>
                                            <span className="text-gray-400">Network:</span> <span>{formatCurrency(day.network_cost)}</span>
                                            {day.compute_cost !== undefined && <><span className="text-gray-400">Compute:</span> <span>{formatCurrency(day.compute_cost)}</span></>}
                                        </div>
                                    </div>
                                </div>
                            );
                        })}
                    </div>
                </div>
            ) : (
                <p className="text-sm text-gray-500">No trend data available for this period.</p>
            )}
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
