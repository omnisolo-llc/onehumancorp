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
      } catch (error) {
        console.error("Error fetching dashboard data:", error);
      } finally {
        setLoading(false);
      }
    }
    fetchCostData();
  }, []);

  if (loading) {
      return (
          <div data-testid="cost-dashboard-loading" className="flex items-center justify-center min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 text-gray-900">
              <div className="flex items-center gap-3">
                  <div className="w-8 h-8 border-4 border-indigo-200 border-t-indigo-600 rounded-full animate-spin"></div>
                  <span className="text-gray-600 font-medium font-outfit">Loading dashboard...</span>
              </div>
          </div>
      );
  }

  const formatCurrency = (cents: number) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(cents / 100);
  };

  const formatStorage = (bytes: number) => {
      const mb = bytes / (1024 * 1024);
      if (mb < 1) return "< 1 MB";
      if (mb >= 1024) return parseFloat((mb / 1024).toFixed(2)) + " GB";
      return parseFloat(mb.toFixed(1)) + " MB";
  };

  // Derive alert logic from data if present
  let needsAlert = false;
  let alertMessage = "";
  if (data?.department_tier_usage) {
      for (const d of data.department_tier_usage.departments) {
          if (d.soft_limit_reached) {
              needsAlert = true;
              alertMessage = `The ${d.department_type} department has reached its tier limits. Check workflow queues.`;
              break;
          }
          if (d.usage_percent !== null && d.usage_percent >= 80) {
              needsAlert = true;
              alertMessage = `The ${d.department_type} department is nearing its usage limits (${d.usage_percent}%).`;
          }
      }
  }

  return (
      <div className="flex flex-col min-h-screen bg-gradient-to-br from-indigo-50 via-white to-purple-50 font-inter text-gray-900 w-full overflow-x-hidden">
        <header className="px-6 py-4 flex items-center justify-between sticky top-0 z-50 bg-white/70 backdrop-blur-xl saturate-200 border-b border-white/40 shadow-sm w-full">
            <div>
               <h1 className="text-2xl font-bold font-outfit tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-gray-900 to-gray-600">Cost Transparency</h1>
               {data && <p className="text-sm text-gray-500 font-medium mt-1">Period: {data.period_start} to {data.period_end}</p>}
            </div>
            <button
               onClick={() => router.push('/plan')}
               className="min-h-[44px] px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-800 rounded-xl text-sm font-medium transition-all shadow-sm flex items-center justify-center">
               Back to My Plan
            </button>
        </header>

        <main className="flex-1 p-6 md:p-8 max-w-7xl mx-auto w-full flex flex-col gap-8">

          {/* Advisory / Alert Section */}
          {needsAlert ? (
               <div className="p-4 rounded-xl bg-amber-50 border border-amber-200 shadow-sm flex items-start gap-3">
                   <span className="text-xl">⚠️</span>
                   <div>
                       <h3 className="font-bold text-amber-900 font-outfit">Budget Alert</h3>
                       <p className="text-sm text-amber-800 mt-1">{alertMessage}</p>
                   </div>
               </div>
          ) : (
               <div className="p-4 rounded-xl app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 shadow-sm">
                   <h3 className="font-bold text-gray-900 font-outfit">Advisory Summary</h3>
                   <p className="text-sm text-gray-600 mt-1">Cost and tier usage are based on connected backend billing, storage, network, and agent department usage signals.</p>
               </div>
          )}

          {/* Top-level Summary Cards */}
          <div className="app-panel bg-transparent shadow-none border-none">
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
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 w-full">
            {/* Main Cost Drivers */}
            <div className="lg:col-span-2 space-y-6">
                <div className="app-panel app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-2xl shadow-sm">
                    <div className="app-panel-header border-b border-white/40 p-5">
                       <h2 className="app-panel-title text-lg font-bold font-outfit text-gray-900">Cost Breakdown</h2>
                    </div>
                    <div className="app-panel-body p-0">
                        <ul className="divide-y divide-gray-100/50">
                            <li className="flex justify-between items-center p-5 hover:bg-gray-50/50 transition-colors">
                                <div>
                                    <span className="font-medium text-gray-900 block">LLM Token Usage</span>
                                    {data?.cache_hit_rate !== undefined && (
                                       <span className="text-xs text-indigo-600 font-medium">Efficiency: {data.cache_hit_rate}% cache hit rate, ${data.cost_per_1k_tokens}/1k tokens</span>
                                    )}
                                </div>
                                <span className="font-semibold font-outfit text-gray-900">{formatCurrency(data?.llm_cost || 0)}</span>
                            </li>
                            <li className="flex justify-between items-center p-5 hover:bg-gray-50/50 transition-colors">
                                <span className="font-medium text-gray-900">Object Storage & CDN</span>
                                <span className="font-semibold font-outfit text-gray-900">{formatCurrency(data?.storage_cost || 0)}</span>
                            </li>
                            <li className="flex justify-between items-center p-5 hover:bg-gray-50/50 transition-colors">
                                <span className="font-medium text-gray-900">Network Transit</span>
                                <span className="font-semibold font-outfit text-gray-900">{formatCurrency(data?.network_cost || 0)}</span>
                            </li>
                            <li className="flex justify-between items-center p-5 hover:bg-gray-50/50 transition-colors">
                                <span className="font-medium text-gray-900">Payment Processing Fees</span>
                                <span className="font-semibold font-outfit text-gray-900">{formatCurrency(data?.payment_fees || 0)}</span>
                            </li>
                            {(data?.compute_cost ?? 0) > 0 && (
                            <li className="flex justify-between items-center p-5 hover:bg-gray-50/50 transition-colors">
                                <span className="font-medium text-gray-900">Compute (Agents)</span>
                                <span className="font-semibold font-outfit text-gray-900">{formatCurrency(data.compute_cost!)}</span>
                            </li>
                            )}
                            <li className="flex justify-between items-center p-5 bg-green-50/30 border-t border-green-100">
                                <span className="font-medium text-green-800 block">Bandwidth Compression Savings</span>
                                <span className="font-semibold font-outfit text-green-700">-{formatCurrency(data?.bandwidth_savings || 0)}</span>
                            </li>
                        </ul>
                    </div>
                </div>

                {/* 7-Day Trend Chart Placeholder */}
                {data?.trend && data.trend.length > 0 && (
                    <div className="app-panel app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-2xl shadow-sm p-6">
                        <h2 className="text-lg font-bold font-outfit text-gray-900 mb-4">7-Day Trend</h2>
                        <div className="h-48 flex items-end justify-between gap-2 border-b border-gray-200 pb-2">
                           {data.trend.slice(-7).map((day, i) => {
                               // simple relative height calculation
                               const maxCost = Math.max(...data.trend.map(t => t.total_cost || 1));
                               const heightPct = Math.max((day.total_cost / maxCost) * 100, 5);
                               const dayLabel = day.date.substring(5); // roughly MM-DD

                               return (
                                   <div key={i} className="flex flex-col items-center justify-end w-full group relative">
                                       <div className="w-full bg-indigo-500 rounded-t-md transition-all duration-300 group-hover:bg-indigo-600" style={{ height: `${heightPct}%` }}></div>
                                       <span className="text-[10px] text-gray-500 mt-2 rotate-45 md:rotate-0 origin-left">{dayLabel}</span>
                                       {/* simple tooltip */}
                                       <div className="absolute -top-10 opacity-0 group-hover:opacity-100 bg-gray-900 text-white text-xs py-1 px-2 rounded pointer-events-none transition-opacity z-10 whitespace-nowrap">
                                           {formatCurrency(day.total_cost)}
                                       </div>
                                   </div>
                               );
                           })}
                        </div>
                    </div>
                )}
            </div>

            {/* Sidebar Data */}
            <div className="space-y-6">
                <div className="app-panel app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-2xl shadow-sm p-6">
                    <h2 className="text-lg font-bold font-outfit text-gray-900 mb-4">Business Performance</h2>
                    <div className="space-y-4">
                        <div>
                            <p className="text-sm text-gray-500 font-medium mb-1">Total Revenue</p>
                            <p className="text-3xl font-bold font-outfit text-green-600">{formatCurrency(data?.total_revenue || 0)}</p>
                        </div>
                        <div className="pt-4 border-t border-gray-100">
                            <p className="text-sm text-gray-500 font-medium mb-1">Total Costs</p>
                            <p className="text-2xl font-bold font-outfit text-gray-900">{formatCurrency(data?.total_costs || 0)}</p>
                        </div>
                        <div className="pt-4 border-t border-gray-100">
                            <p className="text-sm text-gray-500 font-medium mb-1">Projected Monthly</p>
                            <p className="text-2xl font-bold font-outfit text-gray-600">{formatCurrency(data?.projected_monthly_cost || 0)}</p>
                        </div>
                    </div>
                </div>

                {data?.agent_costs && data.agent_costs.length > 0 && (
                    <div className="app-panel app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-2xl shadow-sm p-6">
                        <h2 className="text-lg font-bold font-outfit text-gray-900 mb-4">Agent & Feature Costs</h2>
                        <ul className="space-y-3">
                            {data.agent_costs.map((agent, i) => (
                                <li key={i} className="flex justify-between items-center text-sm">
                                    <span className="text-gray-700 font-medium capitalize">{agent.agent_id.replace(/_/g, ' ')}</span>
                                    <span className="font-semibold text-gray-900">{formatCurrency(agent.cost_cents)}</span>
                                </li>
                            ))}
                        </ul>
                    </div>
                )}

                {/* Department Tier Usage */}
                {data?.department_tier_usage && data.department_tier_usage.departments.length > 0 && (
                    <div className="app-panel app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-2xl shadow-sm p-6">
                        <h2 className="text-lg font-bold font-outfit text-gray-900 mb-4">Department Tier Usage</h2>
                        <ul className="space-y-4">
                            {data.department_tier_usage.departments.map((dept, i) => {
                                const isSoftLimit = dept.soft_limit_reached;
                                const isWarning = !isSoftLimit && dept.usage_percent !== null && dept.usage_percent >= 80;

                                return (
                                    <li key={i} className="text-sm">
                                        <div className="flex justify-between items-center mb-1">
                                            <span className="text-gray-700 font-medium capitalize">{dept.department_type}</span>
                                            {isSoftLimit ? (
                                                <span className="text-xs font-bold text-red-600">Tier limit reached</span>
                                            ) : (
                                                <span className="text-gray-900 font-medium">{dept.actions_used} {dept.action_limit ? `/ ${dept.action_limit} actions` : 'actions'}</span>
                                            )}
                                        </div>
                                        {dept.action_limit && (
                                            <div className="w-full bg-gray-200 rounded-full h-2">
                                                <div
                                                    className={`h-2 rounded-full ${isSoftLimit ? 'bg-red-500' : (isWarning ? 'bg-amber-500' : 'bg-indigo-500')}`}
                                                    style={{ width: `${Math.min(dept.usage_percent || 0, 100)}%` }}
                                                ></div>
                                            </div>
                                        )}
                                    </li>
                                );
                            })}
                        </ul>
                    </div>
                )}
            </div>
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
