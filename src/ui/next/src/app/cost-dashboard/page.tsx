"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

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
  trend: Array<{
    date: string;
    total_cost: number;
    llm_cost: number;
    storage_cost: number;
    network_cost: number;
    compute_cost: number;
  }>;
}

export default function CostDashboardPage() {
  const router = useRouter();
  const [data, setData] = useState<CostDashboardData | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchCostData() {
      try {
        const token = localStorage.getItem('token');
        const res = await fetch('http://127.0.0.1:18789/cost-dashboard', {
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

        <section className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Advisory Summary</h2>
            <p className="text-gray-700 font-medium leading-relaxed">
              Here's what happened this week and what you should do next:<br/><br/>
              - Your revenue is steady, but your AI marketing campaigns are driving more traffic.<br/>
              - <strong>Recommendation:</strong> Consider running a seasonal promotion to capitalize on the recent influx of visitors.<br/>
              - We also noticed a few unread messages in your central inbox. Using the AI draft feature might help you save time!
            </p>
        </section>

        {/* Overview Section */}
        <section className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <div className="flex justify-between items-center mb-6">
               <h2 className="text-xl font-bold font-outfit text-gray-900">Cost Transparency</h2>
               <span id="cost-dashboard-period" className="text-sm text-gray-500 font-medium">Period: {data?.period_start} to {data?.period_end}</span>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div className="p-4 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
                    <h2 className="text-sm font-medium text-gray-500 mb-1">Total Costs</h2>
                    <p id="cost-dashboard-total" className="text-3xl font-bold font-outfit text-gray-900">{formatCurrency(data?.total_costs || 0)}</p>
                </div>
                <div className="p-4 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
                    <h2 className="text-sm font-medium text-gray-500 mb-1">Total Revenue</h2>
                    <p id="cost-dashboard-revenue" className="text-3xl font-bold font-outfit text-green-600">{formatCurrency(data?.total_revenue || 0)}</p>
                </div>

            </div>
        </section>

        {/* Breakdown Section */}
        <section className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <h2 className="text-xl font-bold font-outfit mb-6 text-gray-900">Cost Breakdown</h2>

            <div className="space-y-4">
                <div className="flex justify-between items-center p-4 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
                    <div>
                        <span className="font-medium text-gray-900">LLM Usage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of AI agent actions and interactions.</p>
                    </div>
                    <span id="cost-dashboard-llm" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.llm_cost || 0)}</span>
                </div>

                <div className="flex justify-between items-center p-4 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
                    <div>
                        <span className="font-medium text-gray-900">Storage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of cloud storage and file hosting.</p>
                    </div>
                    <span id="cost-dashboard-storage" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.storage_cost || 0)}</span>
                </div>

                <div className="flex justify-between items-center p-4 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
                    <div>
                        <span className="font-medium text-gray-900">Payment Fees</span>
                        <p className="text-sm text-gray-500 mt-1">Stripe transaction fees on processed revenue.</p>
                    </div>
                    <span id="cost-dashboard-payment-fees" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.payment_fees || 0)}</span>
                </div>
                <div className="flex justify-between items-center p-4 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
                    <div>
                        <span className="font-medium text-gray-900">Network & Bandwidth</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of CDN delivery and outbound traffic.</p>
                    </div>
                    <span id="cost-dashboard-network" className="text-lg font-semibold text-gray-900">{formatCurrency(data?.network_cost || 0)}</span>
                </div>

                <div className="flex justify-between items-center p-4 rounded-xl shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
                    <div>
                        <span className="font-medium text-green-700">Bandwidth Savings</span>
                        <p className="text-sm text-green-600 mt-1">Savings from automated WebP compression and minification.</p>
                    </div>
                    <span id="cost-dashboard-bandwidth-savings" className="text-lg font-semibold text-green-700">-{formatCurrency(data?.bandwidth_savings || 0)}</span>
                </div>
            </div>
        </section>

        {/* Trend Section */}
        <section className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <h2 className="text-xl font-bold font-outfit mb-6 text-gray-900">7-Day Cost Trend</h2>

            <div className="overflow-x-auto">
                <table className="w-full text-left">
                    <thead>
                        <tr className="text-gray-500 text-sm border-b border-gray-200">
                            <th className="pb-4 font-medium">Date</th>
                            <th className="pb-4 font-medium">LLM</th>
                            <th className="pb-4 font-medium">Storage</th>
                            <th className="pb-4 font-medium">Compute</th>
                            <th className="pb-4 font-medium text-right">Total</th>
                        </tr>
                    </thead>
                    <tbody className="divide-y divide-gray-100">
                        {data?.trend && data.trend.length > 0 ? (
                            data.trend.map((day) => (
                                <tr key={day.date} className="text-sm">
                                    <td className="py-4 text-gray-900 font-medium">{day.date}</td>
                                    <td className="py-4 text-gray-600">{formatCurrency(day.llm_cost)}</td>
                                    <td className="py-4 text-gray-600">{formatCurrency(day.storage_cost)}</td>
                                    <td className="py-4 text-gray-600">{formatCurrency(day.compute_cost)}</td>
                                    <td className="py-4 text-gray-900 font-bold text-right">{formatCurrency(day.total_cost)}</td>
                                </tr>
                            ))
                        ) : (
                            <tr>
                                <td colSpan={5} className="py-8 text-center text-gray-500">No trend data available for this period.</td>
                            </tr>
                        )}
                    </tbody>
                </table>
            </div>

            {/* Visual Bar simulation */}
            <div className="mt-8 flex items-end gap-2 h-32 px-2">
                {data?.trend?.map((day, idx) => {
                    const maxCost = Math.max(...data.trend.map(d => d.total_cost), 1);
                    const height = (day.total_cost / maxCost) * 100;
                    return (
                        <div key={idx} className="flex-1 flex flex-col items-center gap-2">
                            <div
                                className="w-full bg-indigo-500 rounded-t-md opacity-80 hover:opacity-100 transition-opacity"
                                style={{ height: `${Math.max(height, 5)}%`, minHeight: '4px' }}
                                title={`${day.date}: ${formatCurrency(day.total_cost)}`}
                            ></div>
                            <span className="text-[10px] text-gray-400 rotate-45 mt-2">{day.date.split('-').slice(1).join('/')}</span>
                        </div>
                    );
                })}
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
