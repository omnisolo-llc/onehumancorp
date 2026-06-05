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
  period_start: string;
  period_end: string;
  trend: DailyCost[];
  ai_actions_used: number;
  ai_actions_limit: number | null;
  storage_used_bytes: number;
  storage_limit_bytes: number | null;
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

  const formatStorage = (bytes: number) => {
      const mb = bytes / (1024 * 1024);
      if (mb < 1) return "< 1 MB";
      if (mb > 1024) return (mb / 1024).toFixed(2) + " GB";
      return mb.toFixed(1) + " MB";
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Business Advisory Dashboard</h1>
        <div className="flex gap-2">
            <button onClick={() => router.push('/plan')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
            Back to My Plan
            </button>
        </div>
      </header>

      <main id="cost-dashboard-screen" className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">

        <section className="p-8 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.2)', borderRadius: '16px', boxShadow: '0 8px 32px rgba(0, 0, 0, 0.05)' }}>
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Advisory Summary</h2>
            <p className="text-gray-700 font-medium leading-relaxed">
              Here's what happened this week and what you should do next:<br/><br/>
              - Your revenue is steady, but your AI marketing campaigns are driving more traffic.<br/>
              - <strong>Recommendation:</strong> Consider running a seasonal promotion to capitalize on the recent influx of visitors.<br/>
              - We also noticed a few unread messages in your central inbox. Using the AI draft feature might help you save time!
            </p>
        </section>

        {/* Overview Section */}
        <section className="p-8 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.2)', borderRadius: '16px', boxShadow: '0 8px 32px rgba(0, 0, 0, 0.05)' }}>
            <div className="flex justify-between items-center mb-6">
               <h2 className="text-xl font-bold font-outfit text-gray-900">Cost Transparency</h2>
               <span id="cost-dashboard-period" className="text-sm text-gray-500 font-medium">Period: {data?.period_start} to {data?.period_end}</span>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div className="p-6 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.5)', backdropFilter: 'blur(10px)', border: '1px solid rgba(255, 255, 255, 0.3)' }}>
                    <h2 className="text-sm font-medium text-gray-500 mb-1">Total Costs</h2>
                    <p id="cost-dashboard-total" className="text-3xl font-bold font-outfit text-gray-900">{formatCurrency(data?.total_costs || 0)}</p>
                </div>
                <div className="p-6 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.5)', backdropFilter: 'blur(10px)', border: '1px solid rgba(255, 255, 255, 0.3)' }}>
                    <h2 className="text-sm font-medium text-gray-500 mb-1">Total Revenue</h2>
                    <p id="cost-dashboard-revenue" className="text-3xl font-bold font-outfit text-green-600">{formatCurrency(data?.total_revenue || 0)}</p>
                </div>

            </div>
        </section>

        {/* Breakdown Section */}
        <section className="p-8 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.2)', borderRadius: '16px', boxShadow: '0 8px 32px rgba(0, 0, 0, 0.05)' }}>
            <h2 className="text-xl font-bold font-outfit mb-6 text-gray-900">Cost Breakdown</h2>

            <div className="space-y-4">
                {/* AI Actions Usage Section */}
                <div className="flex flex-col p-6 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.5)', backdropFilter: 'blur(10px)', border: '1px solid rgba(255, 255, 255, 0.3)' }}>
                    <div className="flex justify-between items-center mb-2">
                        <span className="font-medium text-gray-900">AI Actions Usage</span>
                        <span id="cost-dashboard-ai-actions" className="text-sm font-medium text-gray-500">
                            {data?.ai_actions_used} / {data?.ai_actions_limit === null ? 'Unlimited' : data?.ai_actions_limit}
                        </span>
                    </div>
                    <div className="w-full bg-gray-200 rounded-full h-2.5">
                        <div
                            className="bg-blue-600 h-2.5 rounded-full"
                            style={{
                                width: data?.ai_actions_limit ?
                                    `${Math.min((data.ai_actions_used / data.ai_actions_limit) * 100, 100)}%`
                                    : '100%'
                            }}
                        ></div>
                    </div>
                </div>

                {/* Storage Usage Section */}
                <div className="flex flex-col p-6 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.5)', backdropFilter: 'blur(10px)', border: '1px solid rgba(255, 255, 255, 0.3)' }}>
                    <div className="flex justify-between items-center mb-2">
                        <span className="font-medium text-gray-900">Storage Usage</span>
                        <span id="cost-dashboard-storage-usage" className="text-sm font-medium text-gray-500">
                            {formatStorage(data?.storage_used_bytes || 0)} / {data?.storage_limit_bytes === null ? 'Unlimited' : formatStorage(data?.storage_limit_bytes || 0)}
                        </span>
                    </div>
                    <div className="w-full bg-gray-200 rounded-full h-2.5">
                        <div
                            className="bg-green-500 h-2.5 rounded-full"
                            style={{
                                width: data?.storage_limit_bytes ?
                                    `${Math.min(((data.storage_used_bytes || 0) / data.storage_limit_bytes) * 100, 100)}%`
                                    : '100%'
                            }}
                        ></div>
                    </div>
                </div>

                <div className="flex flex-col p-6 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.5)', backdropFilter: 'blur(10px)', border: '1px solid rgba(255, 255, 255, 0.3)' }}>
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

                <div className="flex justify-between items-center p-6 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.5)', backdropFilter: 'blur(10px)', border: '1px solid rgba(255, 255, 255, 0.3)' }}>
                    <div>
                        <span className="font-medium text-gray-900">LLM Usage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of AI agent actions and interactions.</p>
                    </div>
                    <span id="cost-dashboard-llm" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.llm_cost || 0)}</span>
                </div>

                <div className="flex justify-between items-center p-6 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.5)', backdropFilter: 'blur(10px)', border: '1px solid rgba(255, 255, 255, 0.3)' }}>
                    <div>
                        <span className="font-medium text-gray-900">Storage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of cloud storage and file hosting.</p>
                    </div>
                    <span id="cost-dashboard-storage" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.storage_cost || 0)}</span>
                </div>

                <div className="flex justify-between items-center p-6 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.5)', backdropFilter: 'blur(10px)', border: '1px solid rgba(255, 255, 255, 0.3)' }}>
                    <div>
                        <span className="font-medium text-gray-900">Payment Fees</span>
                        <p className="text-sm text-gray-500 mt-1">Stripe transaction fees on processed revenue.</p>
                    </div>
                    <span id="cost-dashboard-payment-fees" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.payment_fees || 0)}</span>
                </div>
                <div className="flex justify-between items-center p-6 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.5)', backdropFilter: 'blur(10px)', border: '1px solid rgba(255, 255, 255, 0.3)' }}>
                    <div>
                        <span className="font-medium text-gray-900">Network & Bandwidth</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of CDN delivery and outbound traffic.</p>
                    </div>
                    <span id="cost-dashboard-network" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.network_cost || 0)}</span>
                </div>

                <div className="flex justify-between items-center p-6 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.5)', backdropFilter: 'blur(10px)', border: '1px solid rgba(255, 255, 255, 0.3)' }}>
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
