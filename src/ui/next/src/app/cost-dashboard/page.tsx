"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

interface DailyCost {
  date: string;
  total_cost: number;
  llm_cost: number;
  storage_cost: number;
  network_cost: number;
  compute_cost: number;
}

interface CostDashboardData {
  total_revenue: number;
  total_costs: number;
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
}

export default function CostDashboardPage() {
  const router = useRouter();
  const [data, setData] = useState<CostDashboardData | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchCostData() {
      try {
        const token = localStorage.getItem('token');
        const res = await fetch('/api/billing/cost-dashboard', {
            headers: {
                'Authorization': `Bearer ${token}`
            }
        });
        if (res.ok) {
          const fetchedData = await res.json();
          setData(fetchedData);
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

        <section className="p-6 md:p-8 shadow-lg bg-white/60 hover:shadow-xl transition-shadow duration-300" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.2)', borderRadius: '16px', boxShadow: '0 8px 32px rgba(0, 0, 0, 0.05)' }}>
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Advisory Summary</h2>
            <p className="text-gray-700 font-medium leading-relaxed">
              Here's what happened this week and what you should do next:<br/><br/>
              - Your revenue is steady, but your AI marketing campaigns are driving more traffic.<br/>
              - <strong>Recommendation:</strong> Consider running a seasonal promotion to capitalize on the recent influx of visitors.<br/>
              - We also noticed a few unread messages in your central inbox. Using the AI draft feature might help you save time!
            </p>
        </section>

        {/* Overview Section */}
        <section className="p-6 md:p-8 shadow-lg bg-white/60 hover:shadow-xl transition-shadow duration-300" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.2)', borderRadius: '16px', boxShadow: '0 8px 32px rgba(0, 0, 0, 0.05)' }}>
            <div className="flex justify-between items-center mb-6">
               <h2 className="text-xl font-bold font-outfit text-gray-900">Cost Transparency</h2>
               <span id="cost-dashboard-period" className="text-sm text-gray-500 font-medium">Period: {data?.period_start} to {data?.period_end}</span>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div className="p-6 rounded-2xl shadow-sm bg-white/50 backdrop-blur-lg border border-white/50 hover:-translate-y-1 hover:shadow-md transition-all duration-300 group">
                    <h2 className="text-sm font-medium text-gray-500 mb-1">Total Costs</h2>
                    <p id="cost-dashboard-total" className="text-3xl font-bold font-outfit text-gray-900">{formatCurrency(data?.total_costs || 0)}</p>
                </div>
                <div className="p-6 rounded-2xl shadow-sm bg-white/50 backdrop-blur-lg border border-white/50 hover:-translate-y-1 hover:shadow-md transition-all duration-300 group">
                    <h2 className="text-sm font-medium text-gray-500 mb-1">Total Revenue</h2>
                    <p id="cost-dashboard-revenue" className="text-3xl font-bold font-outfit text-green-600">{formatCurrency(data?.total_revenue || 0)}</p>
                </div>
                <div className="p-6 rounded-2xl shadow-sm bg-green-50/80 backdrop-blur-lg border border-green-200/50 hover:-translate-y-1 hover:shadow-md transition-all duration-300 group">
                    <h2 className="text-sm font-medium text-green-700 mb-1">Network & Storage Savings</h2>
                    <p id="cost-dashboard-total-savings" className="text-3xl font-bold font-outfit text-green-700">{formatCurrency((data?.bandwidth_savings || 0))}</p>
                    <p className="text-xs text-green-600 mt-2">Saved via auto-compression</p>
                </div>
            </div>
        </section>

        {/* Breakdown Section */}
        <section className="p-6 md:p-8 shadow-lg bg-white/60 hover:shadow-xl transition-shadow duration-300" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.2)', borderRadius: '16px', boxShadow: '0 8px 32px rgba(0, 0, 0, 0.05)' }}>
            <h2 className="text-xl font-bold font-outfit mb-6 text-gray-900">Cost Breakdown</h2>

            <div className="space-y-4">
                <div className="flex flex-col p-6 rounded-2xl shadow-sm bg-white/50 backdrop-blur-lg border border-white/50 hover:-translate-y-1 hover:shadow-md transition-all duration-300">
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

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 p-6 rounded-2xl shadow-sm bg-white/50 backdrop-blur-lg border border-white/50 hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">LLM Usage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of AI agent actions and interactions.</p>
                    </div>
                    <div className="text-left sm:text-right w-full sm:w-auto">
                        <span id="cost-dashboard-llm" className="text-lg font-semibold text-gray-900 block">{formatCurrency(data?.llm_cost || 0)}</span>
                        <span className="text-xs text-gray-500 font-medium">Efficiency: {data?.cache_hit_rate}% cache hit rate, ${data?.cost_per_1k_tokens.toFixed(4)}/1k tokens</span>
                    </div>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 p-6 rounded-2xl shadow-sm bg-white/50 backdrop-blur-lg border border-white/50 hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">Storage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of cloud storage and file hosting.</p>
                    </div>
                    <span id="cost-dashboard-storage" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.storage_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 p-6 rounded-2xl shadow-sm bg-white/50 backdrop-blur-lg border border-white/50 hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">Payment Fees</span>
                        <p className="text-sm text-gray-500 mt-1">Stripe transaction fees on processed revenue.</p>
                    </div>
                    <span id="cost-dashboard-payment-fees" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.payment_fees || 0)}</span>
                </div>
                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 p-6 rounded-2xl shadow-sm bg-white/50 backdrop-blur-lg border border-white/50 hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-gray-900">Network & Bandwidth</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of CDN delivery and outbound traffic.</p>
                    </div>
                    <span id="cost-dashboard-network" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.network_cost || 0)}</span>
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 p-6 rounded-2xl shadow-sm bg-white/50 backdrop-blur-lg border border-white/50 hover:-translate-y-1 hover:shadow-md transition-all duration-300">
                    <div>
                        <span className="font-medium text-green-700">Bandwidth Savings</span>
                        <p className="text-sm text-green-600 mt-1">Savings from automated WebP compression and minification.</p>
                    </div>
                    <span id="cost-dashboard-bandwidth-savings" className="text-lg font-semibold text-green-700">-{formatCurrency(data?.bandwidth_savings || 0)}</span>
                </div>
            </div>
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
