"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { AppShell } from '../components/AppShell';

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
  budget_health_alert?: boolean;
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
  const [actionMessage, setActionMessage] = useState<string | null>(null);

  const handleManageBilling = async () => {
    try {
      const token = localStorage.getItem('token');
      const response = await fetch('/api/billing/create-billing-portal-session', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...(token ? { 'Authorization': `Bearer ${token}` } : {})
        },
      });

      if (!response.ok) {
        throw new Error('Failed to create billing portal session');
      }

      const session = await response.json();
      if (session.url) {
        window.location.href = session.url;
      }
    } catch (error) {
      console.error('Error initiating billing portal:', error);
      setActionMessage('Failed to initiate billing portal. Please try again.');
    }
  };

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
          <AppShell title="Cost Transparency Dashboard" subtitle="Cost and tier usage signals.">
              <div className="max-w-6xl mx-auto w-full flex flex-col gap-6 animate-pulse" data-testid="cost-dashboard-loading">
                  <div className="h-48 bg-white/70 backdrop-blur-[30px] saturate-[210%] saturate-200 border border-white/40 rounded-2xl w-full"></div>
                  <div className="h-64 bg-white/70 backdrop-blur-[30px] saturate-[210%] saturate-200 border border-white/40 rounded-2xl w-full"></div>
              </div>
          </AppShell>
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
    <AppShell
      title="Cost Transparency Dashboard"
      subtitle="Cost and tier usage signals based on connected billing, storage, and agents."
      actions={[{ label: "Back to My Plan", href: "/plan" }]}
    >
      <div className="flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6 font-inter">
        <section className="app-panel glass-panel hover:shadow-xl transition-shadow duration-300">
            <div className="app-panel-header backdrop-blur-md bg-white/70 px-6 py-4">
                <h2 className="app-panel-title text-xl font-bold font-outfit text-gray-900 dark:text-white">Advisory Summary</h2>
            </div>
            <div className="app-panel-body p-6">
                <p className="text-gray-700 dark:text-gray-300 font-medium leading-relaxed">
                  Cost and tier usage are based on connected backend billing, storage, network, and agent department usage signals.
                </p>
            </div>
        </section>

        {/* My Plan Section */}
        <section id="my-plan-section" className="app-panel glass-panel">
          <div className="app-panel-header backdrop-blur-md bg-white/70 px-6 py-4 flex justify-between items-center">
             <h2 className="app-panel-title text-xl font-bold font-outfit text-gray-900 dark:text-white">My Plan</h2>
             <button
               onClick={() => router.push('/pricing')}
               className="min-h-[44px] px-6 py-2 bg-[#0f766e] hover:bg-[#0d645d] text-white rounded-xl text-sm font-medium transition-all shadow-sm flex items-center justify-center">
               Upgrade
             </button>
          </div>
          <div className="app-panel-body p-6">
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                  <div className="p-4 app-card ohc-growth-card glass-card">
                      <h3 className="text-sm font-medium text-gray-500">Current Plan</h3>
                      <p className="text-2xl font-bold text-gray-900 dark:text-white mt-1">{myPlanData?.current_plan || 'Free'}</p>
                  </div>
                  <div className="p-4 app-card ohc-growth-card glass-card">
                      <h3 className="text-sm font-medium text-gray-500">AI actions used this month</h3>
                      <p className="text-2xl font-bold text-gray-900 dark:text-white mt-1">{myPlanData?.ai_actions_used || 0} <span className="text-sm text-gray-500 font-normal">{myPlanData?.ai_actions_limit != null && myPlanData.ai_actions_limit > 0 ? `/ ${myPlanData.ai_actions_limit}` : '/ Unlimited'}</span></p>
                  </div>
                  <div className="p-4 app-card ohc-growth-card glass-card">
                      <h3 className="text-sm font-medium text-gray-500">Storage used</h3>
                      <p className="text-2xl font-bold text-gray-900 dark:text-white mt-1">{formatStorage(myPlanData?.storage_used_bytes || 0)} <span className="text-sm text-gray-500 font-normal">{myPlanData?.storage_limit_bytes != null && myPlanData.storage_limit_bytes > 0 ? `/ ${formatStorage(myPlanData.storage_limit_bytes)}` : '/ Unlimited'}</span></p>
                  </div>
                  <div className="p-4 app-card ohc-growth-card glass-card">
                      <h3 className="text-sm font-medium text-gray-500">Estimated Next Bill:</h3>
                      <p className="text-2xl font-bold text-gray-900 dark:text-white mt-1">{formatCurrency(myPlanData?.next_bill_estimated || 0)}</p>
                  </div>
                  {myPlanData?.storage_limit_bytes && myPlanData?.storage_used_bytes != null && (
                      <div className="p-4 app-card ohc-growth-card glass-card md:col-span-2 lg:col-span-4">
                          <div className="flex justify-between text-sm text-gray-600 mb-1">
                              <span>{myPlanData.storage_limit_bytes > 0 ? `${formatStorage(myPlanData.storage_limit_bytes).replace('.0 MB', ' MB').replace('.0 GB', ' GB')} Storage Quota` : 'Unlimited Storage Quota'}</span>
                              <span>
                                  {myPlanData.storage_limit_bytes > 0 ? (
                                      myPlanData.storage_used_bytes >= myPlanData.storage_limit_bytes ? 'Limit reached' : `${formatStorage(myPlanData.storage_used_bytes)} used (${Math.round((myPlanData.storage_used_bytes / myPlanData.storage_limit_bytes) * 100)}%)`
                                  ) : `${formatStorage(myPlanData.storage_used_bytes)} used`}
                              </span>
                          </div>
                          <div className="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
                              <div className={`h-2.5 rounded-full ${myPlanData.storage_used_bytes >= myPlanData.storage_limit_bytes ? 'bg-red-600' : 'bg-[#0071E3]'}`} style={{ width: `${Math.min(100, Math.max(0, (myPlanData.storage_used_bytes / myPlanData.storage_limit_bytes) * 100))}%` }}></div>
                          </div>
                      </div>
                  )}
              </div>
              {myPlanData?.current_plan !== 'Free' && (
                  <div className="mt-6 flex flex-col md:flex-row gap-4">
                      <button
                          id="manage-billing-btn"
                          onClick={handleManageBilling}
                          className="px-6 py-2 bg-[#0f766e] hover:bg-[#0d645d] text-white rounded-xl font-medium transition-all shadow-sm flex items-center justify-center"
                      >
                          Manage Billing
                      </button>
                  </div>
              )}
              {actionMessage && (
                  <div className="mt-4 rounded-xl border border-teal-100 bg-teal-50/20 p-4 text-sm font-medium text-[#0f766e] dark:text-[#6ac5bd] shadow-sm" role="status">
                      {actionMessage}
                  </div>
              )}
          </div>
        </section>

        {/* Overview Section */}
        <section className="app-panel glass-panel hover:shadow-xl transition-shadow duration-300">
            <div className="app-panel-header backdrop-blur-md bg-white/70 px-6 py-4">
               <h2 className="app-panel-title text-xl font-bold font-outfit text-gray-900 dark:text-white">Cost Transparency Dashboard</h2>
               <span id="cost-dashboard-period" className="text-sm text-gray-500 font-medium">Period: {data?.period_start} to {data?.period_end}</span>
            </div>

            <div className="app-panel-body p-6">
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <div className="app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 rounded-2xl group">
                        <h2 className="text-sm font-medium text-gray-500 mb-1">Total Costs</h2>
                        <p id="cost-dashboard-total-costs" className="text-3xl font-bold font-outfit text-gray-900 dark:text-white">{formatCurrency(data?.total_costs || 0)}</p>
                    </div>
                    <div className="app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 rounded-2xl group">
                        <h2 className="text-sm font-medium text-gray-500 mb-1">Projected Monthly Cost</h2>
                        <p id="cost-dashboard-projected" className="text-3xl font-bold font-outfit text-[#0f766e] dark:text-[#6ac5bd]">{formatCurrency(data?.projected_monthly_cost || 0)}</p>
                    </div>
                    <div className="app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 rounded-2xl group">
                        <h2 className="text-sm font-medium text-gray-500 mb-1">Total Revenue</h2>
                        <p id="cost-dashboard-revenue" className="text-3xl font-bold font-outfit text-green-600">{formatCurrency(data?.total_revenue || 0)}</p>
                    </div>
                    <div className="app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 rounded-2xl group">
                        <h2 className="text-sm font-medium text-green-700 mb-1">Network & Storage Savings</h2>
                        <p id="cost-dashboard-total-savings" className="text-3xl font-bold font-outfit text-green-700">{formatCurrency((data?.bandwidth_savings || 0))}</p>
                        <p className="text-xs text-green-600 mt-2">Saved via auto-compression</p>
                    </div>
                </div>
            </div>
        </section>

        {/* Budget Health Alert */}
        {data && (data.budget_health_alert || (data.department_tier_usage?.departments?.some((d: any) => d.soft_limit_reached))) && (
            <div id="budget-health-alert" className="p-4 bg-amber-50/70 border border-amber-200 backdrop-blur-[30px] saturate-[210%] saturate-200 rounded-xl shadow-md hover:shadow-lg transition-all duration-300 flex items-start gap-3">
                <svg className="w-5 h-5 text-amber-600 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                </svg>
                <div>
                    <h3 className="text-sm font-semibold text-amber-800">Budget Alert</h3>
                    <p className="text-sm text-amber-700 mt-1">Heads up! Your projected monthly cost ({formatCurrency(data.projected_monthly_cost)}) is reaching your plan's soft limit. Soft Limit Approaching. Upgrade to a higher tier to avoid disruption and secure better bulk rates!</p>
                </div>
            </div>
        )}

        {/* Breakdown Section */}
        <section className="app-panel glass-panel hover:shadow-xl transition-shadow duration-300">
            <div className="app-panel-header backdrop-blur-md bg-white/70 px-6 py-4">
                <h2 className="app-panel-title text-xl font-bold font-outfit text-gray-900 dark:text-white">Cost Breakdown</h2>
            </div>

            <div className="app-panel-body p-6 space-y-4">
                <div className="flex flex-col app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 rounded-2xl">
                    <h3 className="font-medium text-gray-950 dark:text-white mb-4">7-Day Trend</h3>
                    {data?.trend && data.trend.length > 0 ? (
                        <div className="flex items-end h-32 gap-2 mt-4" id="cost-dashboard-trend">
                            {data.trend.map((daily, index) => {
                                const maxCost = Math.max(...data.trend.map(d => d.total_cost), 1);
                                const heightPercent = Math.max((daily.total_cost / maxCost) * 100, 5);
                                return (
                                    <div key={index} className="flex-1 flex flex-col items-center gap-2 group">
                                        <div className="w-full bg-teal-50/10 rounded-t-md relative flex items-end justify-center group-hover:bg-teal-50/20 transition-colors" style={{ height: '100px' }}>
                                            <div className="w-full bg-[#0f766e] rounded-t-md transition-all duration-500 group-hover:bg-[#0d645d]" style={{ height: `${heightPercent}%` }}></div>
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

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 rounded-2xl">
                    <div>
                        <span className="font-medium text-gray-900 dark:text-white flex items-center gap-2">
                            LLM Usage
                            {data?.department_tier_usage?.departments?.some(d => d.action_limit !== null && d.actions_used / d.action_limit >= 0.8) || data?.budget_health_alert ? (
                                <span id="budget-alert-badge" className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-amber-50/70 backdrop-blur-[30px] saturate-[210%] border border-amber-200 text-amber-800">
                                    <svg className="mr-1 h-3 w-3 text-amber-500" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
                                        <path fillRule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
                                    </svg>
                                    Budget Alert
                                </span>
                            ) : null}
                        </span>
                        <p className="text-sm text-gray-500 mt-1">Cost of AI agent actions and interactions.</p>
                    </div>
                    <div className="text-left sm:text-right w-full sm:w-auto">
                        <span id="cost-dashboard-llm" className="text-lg font-semibold text-gray-900 dark:text-white block">{formatCurrency(data?.llm_cost || 0)}</span>
                        <span className="text-xs text-gray-500 font-medium">Efficiency: {data?.cache_hit_rate}% cache hit rate, ${data?.cost_per_1k_tokens?.toFixed(4) || "0.0000"}/1k tokens</span>
                    </div>
                </div>

                {/* Per-Agent / Per-Feature Costs */}
                <div className="flex flex-col app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 rounded-2xl">
                    <h3 className="font-medium text-gray-900 dark:text-white mb-2">Agent & Feature Costs</h3>
                    {data?.agent_costs && data.agent_costs.length > 0 ? (
                        <ul id="cost-dashboard-agent-costs" className="space-y-2">
                            {data.agent_costs.map((agent, index) => (
                                <li key={index} className="flex justify-between items-center border-b border-gray-200 dark:border-gray-800 pb-2 last:border-b-0 last:pb-0">
                                    <span className="text-sm text-gray-700 dark:text-gray-300 capitalize">{agent.agent_id.replace(/_/g, ' ')}</span>
                                    <span className="text-sm font-medium text-gray-900 dark:text-white">{formatCurrency(agent.cost_cents)}</span>
                                </li>
                            ))}
                        </ul>
                    ) : (
                        <p className="text-sm text-gray-500">No agent cost data available.</p>
                    )}
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 rounded-2xl">
                    <div>
                        <span className="font-medium text-gray-900 dark:text-white">Storage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of cloud storage and file hosting.</p>
                    </div>
                    <span id="cost-dashboard-storage" className="text-lg font-semibold text-gray-900 dark:text-white">{formatCurrency(data?.storage_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 rounded-2xl">
                    <div>
                        <span className="font-medium text-gray-900 dark:text-white">Payment Fees</span>
                        <p className="text-sm text-gray-500 mt-1">Stripe transaction fees on processed revenue.</p>
                    </div>
                    <span id="cost-dashboard-payment-fees" className="text-lg font-semibold text-gray-900 dark:text-white">{formatCurrency(data?.payment_fees || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 rounded-2xl">
                    <div>
                        <span className="font-medium text-gray-900 dark:text-white">Compute Usage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of container execution and background processing.</p>
                    </div>
                    <span id="cost-dashboard-compute" className="text-lg font-semibold text-gray-900 dark:text-white">{formatCurrency(data?.compute_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 rounded-2xl">
                    <div>
                        <span className="font-medium text-gray-900 dark:text-white">Network & Bandwidth</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of CDN delivery and outbound traffic.</p>
                    </div>
                    <span id="cost-dashboard-network" className="text-lg font-semibold text-gray-900 dark:text-white">{formatCurrency(data?.network_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 rounded-2xl">
                    <div>
                        <span className="font-medium text-gray-900 dark:text-white">Email Sends</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of transactional and marketing email delivery.</p>
                    </div>
                    <span id="cost-dashboard-email" className="text-lg font-semibold text-gray-900 dark:text-white">{formatCurrency(data?.email_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 rounded-2xl">
                    <div>
                        <span className="font-medium text-gray-900 dark:text-white">Outbound API Calls</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of third-party integration usage.</p>
                    </div>
                    <span id="cost-dashboard-api" className="text-lg font-semibold text-gray-900 dark:text-white">{formatCurrency(data?.api_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 rounded-2xl">
                    <div>
                        <span className="font-medium text-green-700">Network & Storage Savings</span>
                        <p className="text-sm text-green-600 mt-1">Savings from automated WebP compression and minification.</p>
                    </div>
                    <span id="cost-dashboard-bandwidth-savings" className="text-lg font-semibold text-green-700">-{formatCurrency(data?.bandwidth_savings || 0)}</span>
                </div>
            </div>
        </section>

        <section className="app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg rounded-2xl hover:shadow-2xl transition-shadow duration-300 p-6 md:p-8">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 mb-6">
                <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white">Department Tier Usage</h2>
                <span className="text-sm text-gray-500 font-medium">
                  {data?.department_tier_usage?.current_plan ? `${data.department_tier_usage.current_plan} plan` : 'Loading...'} · {data?.department_tier_usage?.period || data?.period_end?.slice(0, 7) || ''}
                </span>
            </div>

            {data?.department_tier_usage?.departments?.length ? (
                <div className="space-y-4" id="department-tier-usage-list">
                    {data.department_tier_usage.departments.map((department) => (
                        <div key={department.id} className="p-5 app-card ohc-growth-card glass-card">
                            <div className="flex flex-col sm:flex-row sm:items-start justify-between gap-3">
                                <div>
                                    <h3 className="font-semibold text-gray-900 dark:text-white">{department.department_type}</h3>
                                    <p className="text-sm text-gray-500 mt-1">{department.agent_id}</p>
                                </div>
                                <div className="text-left sm:text-right">
                                    <p className="font-semibold text-gray-900 dark:text-white">
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
                                <div className="mt-4 h-2 rounded-full bg-gray-250 overflow-hidden" aria-label={`${department.department_type} usage`}>
                                    <div
                                      className={department.soft_limit_reached ? "h-full bg-amber-500" : "h-full bg-[#0f766e]"}
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
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        /* The .ohc-growth-card styles are now managed globally in globals.css for design token consistency */
      `}} />
    </AppShell>
  );
}
