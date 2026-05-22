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
          const json = await res.json();
          setData(json);
        } else {
            console.error("Failed to fetch cost dashboard data:", res.status);
            setData({
                total_revenue: 0,
                total_costs: 0,
                llm_cost: 0,
                storage_cost: 0,
                payment_fees: 0,
                period_start: "-",
                period_end: "-",
            });
        }
      } catch (err) {
        console.error("Error fetching cost dashboard data", err);
        setData({
            total_revenue: 0,
            total_costs: 0,
            llm_cost: 0,
            storage_cost: 0,
            payment_fees: 0,
            period_start: "-",
            period_end: "-",
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

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Cost & AI Usage</h1>
        <button onClick={() => router.push('/plan')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
          Back to My Plan
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">
        <section className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
          <p id="cost-dashboard-total">Total Costs: ${((data?.total_costs || 0) / 100).toFixed(2)}</p>
          <p id="cost-dashboard-llm">LLM Usage: ${((data?.llm_cost || 0) / 100).toFixed(2)}</p>
          <p id="cost-dashboard-storage">Storage: ${((data?.storage_cost || 0) / 100).toFixed(2)}</p>
          <p id="cost-dashboard-period">Period: {data?.period_start} to {data?.period_end}</p>
        </section>
      </main>
    </div>
  );
}
