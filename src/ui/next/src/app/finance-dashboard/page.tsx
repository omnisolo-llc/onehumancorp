"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

interface CashflowForecast {
  projected_inflow: number;
  cash_gap_alert: boolean;
}

export default function FinanceDashboardPage() {
  const router = useRouter();
  const [data, setData] = useState<CashflowForecast | null>(null);
  const [loading, setLoading] = useState(true);
  const [remindersSent, setRemindersSent] = useState(false);

  useEffect(() => {
    async function fetchForecast() {
      try {
        const tenant_id = localStorage.getItem('tenant') || 'default_tenant';
        const res = await fetch(`/api/ledger/cashflow-forecast/${tenant_id}`);
        if (res.ok) {
          const fetchedData = await res.json();
          setData(fetchedData);
        } else {
            console.error("Failed to fetch forecast");
            setData({ projected_inflow: 0, cash_gap_alert: true });
        }
      } catch (err) {
        console.error("Error fetching forecast", err);
        setData({ projected_inflow: 0, cash_gap_alert: true });
      } finally {
        setLoading(false);
      }
    }
    fetchForecast();
  }, []);

  if (loading) {
      return <div className="min-h-screen flex items-center justify-center">Loading...</div>;
  }

  const formatCurrency = (amount: number) => {
      return Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(amount);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Financial Health Dashboard</h1>
        <div className="flex gap-2">
            <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
            Back to Dashboard
            </button>
        </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">

        {/* Financial Health Forecasting Section */}
        <section className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <h2 className="text-xl font-bold font-outfit mb-6 text-gray-900">Cashflow Forecast (Next 30 Days)</h2>
            <div className="flex justify-between items-center p-4 bg-white rounded-xl border border-gray-100 shadow-sm mb-4">
                <div>
                    <span className="font-medium text-gray-900">Projected Inflow</span>
                    <p className="text-sm text-gray-500 mt-1">Based on invoices currently marked as 'Sent' and due soon.</p>
                </div>
                <span className="text-lg font-semibold text-gray-900">{formatCurrency(data?.projected_inflow || 0)}</span>
            </div>

            {data?.cash_gap_alert && (
                <div className="p-4 bg-red-50 text-red-800 rounded-xl mb-4 border border-red-200">
                    <p className="font-semibold">⚠️ Cash Gap Alert</p>
                    <p className="text-sm mt-1">Your projected inflow is below the recommended threshold. Resolving unpaid invoices can help cover upcoming operational expenses.</p>
                </div>
            )}

            <button
                onClick={() => setRemindersSent(true)}
                disabled={remindersSent}
                className={`w-full py-2 rounded-lg text-sm font-semibold transition-all ${remindersSent ? 'bg-gray-200 text-gray-500 cursor-not-allowed' : 'bg-blue-100 text-blue-700 hover:bg-blue-200'}`}>
                {remindersSent ? 'Reminders Sent' : 'Auto-send invoice reminders'}
            </button>
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
