"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

interface CostDashboardData {
  total_revenue: number;
  total_costs: number;
  llm_cost: number;
  storage_cost: number;
  payment_fees: number;
  period_start: string;
  period_end: string;
  llm_tokens: number;
}

export default function CostDashboardPage() {
  const router = useRouter();
  const [data, setData] = useState<CostDashboardData | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchCostData() {
      try {
        const token = localStorage.getItem('token') || 'test-token';
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
            // Fallback for UI
            setData({
                total_revenue: 0,
                total_costs: 0,
                llm_cost: 0,
                storage_cost: 0,
                payment_fees: 0,
                period_start: "2024-05-01",
                period_end: "2024-05-31",
                llm_tokens: 0,
            });
        }
      } catch (err) {
        console.error("Error fetching cost data", err);
        setData({
            total_revenue: 0,
            total_costs: 0,
            llm_cost: 0,
            storage_cost: 0,
            payment_fees: 0,
            period_start: "2024-05-01",
            period_end: "2024-05-31",
        });
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
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Business Advisory Dashboard</h1>
        <div className="flex gap-2">
            <button onClick={() => router.push('/plan')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
            Back to My Plan
            </button>
        </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">

        <section className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Advisory Summary</h2>
            <p className="text-gray-700 font-medium leading-relaxed">
              Here's what happened this week and what you should do next:<br/><br/>
              - Your revenue is steady, but your AI marketing campaigns are driving more traffic.<br/>
              - <strong>Recommendation:</strong> Consider running a seasonal promotion to capitalize on the recent influx of visitors.<br/>
              - We also noticed a few unread messages in your unified inbox. Using the AI draft feature might help you save time!
            </p>
        </section>

        {/* Overview Section */}
        <section className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <div className="flex justify-between items-center mb-6">
               <h2 className="text-xl font-bold font-outfit text-gray-900">Cost & AI Usage</h2>
               <span className="text-sm text-gray-500 font-medium">Period: {data?.period_start} to {data?.period_end}</span>
            </div>

            <div className="grid grid-cols-1 gap-6">
                <div className="p-4 bg-white rounded-xl border border-gray-100 shadow-sm">
                    <h2 className="text-sm font-medium text-gray-500 mb-1">Total Costs</h2>
                    <p className="text-3xl font-bold font-outfit text-gray-900">Total Costs: {formatCurrency(data?.total_costs || 0)}</p>
                </div>
            </div>
        </section>

        {/* Breakdown Section */}
        <section className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <h2 className="text-xl font-bold font-outfit mb-6 text-gray-900">Cost Breakdown</h2>

            <div className="space-y-4">
                <div className="flex justify-between items-center p-4 bg-white rounded-xl border border-gray-100 shadow-sm">
                    <div>

                        <p className="text-sm text-gray-500 mt-1">Cost of AI agent actions and interactions.</p>
                    </div>
                    <span className="text-lg font-semibold text-gray-900">LLM Usage: {data?.llm_tokens ? data.llm_tokens.toLocaleString() : "0"} tokens</span>
                </div>

                <div className="flex justify-between items-center p-4 bg-white rounded-xl border border-gray-100 shadow-sm">
                    <div>
                        <span className="font-medium text-gray-900">Storage</span>
                        <p className="text-sm text-gray-500 mt-1">Cost of cloud storage and file hosting.</p>
                    </div>
                    <span className="text-lg font-semibold text-gray-900">{formatCurrency(data?.storage_cost || 0)}</span>
                </div>

                <div className="flex justify-between items-center p-4 bg-white rounded-xl border border-gray-100 shadow-sm">
                    <div>
                        <span className="font-medium text-gray-900">Payment Fees</span>
                        <p className="text-sm text-gray-500 mt-1">Stripe transaction fees on processed revenue.</p>
                    </div>
                    <span className="text-lg font-semibold text-gray-900">{formatCurrency(data?.payment_fees || 0)}</span>
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
