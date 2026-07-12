"use client";

import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { WithTooltip } from '../../components/TooltipRegistry';

interface MyPlanData {
  current_plan: string;
  ai_actions_used: number;
  ai_actions_limit: number | null;
  storage_used_bytes: number;
  storage_limit_bytes: number | null;
  next_bill_estimated: number;
  soft_limit_reached?: boolean;
  user_message?: string;
}

export default function MyPlanPage() {
  const router = useRouter();
  const [data, setData] = useState<MyPlanData | null>(null);
  const [loading, setLoading] = useState(true);
  const [isManagingBilling, setIsManagingBilling] = useState(false);

  useEffect(() => {
    const fetchPlanData = async () => {
      try {
        const token = localStorage.getItem('token');
        const response = await fetch('/api/billing/my-plan', {
          headers: token ? { 'Authorization': `Bearer ${token}` } : {}
        });
        if (response.ok) {
          const json = await response.json();
          setData(json);
        }
      } catch (error) {
        console.error('Failed to fetch plan data:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchPlanData();
  }, []);

  const formatStorage = (bytes: number) => {
      const mb = bytes / (1024 * 1024);
      if (mb < 1) return "< 1 MB";
      if (mb >= 1024) return parseFloat((mb / 1024).toFixed(2)) + " GB";
      return parseFloat(mb.toFixed(1)) + " MB";
  };

  const handleManageBilling = async () => {
    setIsManagingBilling(true);
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

      const data = await response.json();
      if (data.url) {
        window.location.href = data.url;
      }
    } catch (error) {
      console.error('Billing portal error:', error);
      alert('Failed to initiate billing portal. Please try again.');
      setIsManagingBilling(false);
    }
  };

  const formatCurrency = (amount: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(amount / 100);
  };

  if (loading) {
    return (
      <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 justify-center items-center p-4">
        <div className="flex flex-col items-center justify-center p-8 app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/10 border border-white/20 shadow-lg rounded-2xl w-full max-w-sm animate-pulse">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
          <p className="mt-6 text-gray-600 font-medium">Loading your plan data...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 text-gray-900 w-full overflow-x-hidden max-w-[100vw]">
      <header className="px-4 py-4 flex items-center justify-between sticky top-0 z-50 app-panel-header shadow-sm w-full glass-panel backdrop-blur-xl bg-white/40 border-b border-white/20">
        <div className="flex items-center gap-3">
          <button onClick={() => router.push('/dashboard')} className="min-w-[44px] min-h-[44px] px-3 py-2 glass-card backdrop-blur-md bg-white/30 border border-white/30 shadow-sm rounded-xl text-sm font-medium text-gray-800 hover:-translate-y-0.5 hover:shadow-md transition-all duration-300 flex items-center justify-center">
            Back
          </button>
          <WithTooltip id="my-plan-tooltip" defaultText="View and manage your subscription plan and usage.">
            <h1 className="text-xl md:text-2xl font-bold font-outfit text-gray-900 tracking-tight">My Plan</h1>
          </WithTooltip>
        </div>
      </header>

      <main className="p-4 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">

        {data?.soft_limit_reached && data?.user_message && (
            <div className="mb-2 p-4 bg-amber-50 border border-amber-200 rounded-xl text-amber-800 flex items-start gap-3 shadow-sm">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-5 h-5 mt-0.5 shrink-0 text-amber-600" style={{ width: '20px', height: '20px' }}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
              </svg>
              <p className="text-sm font-medium">{data.user_message}</p>
            </div>
        )}

        {/* Status Snapshot */}
        <section className="app-card ohc-growth-card glass-card backdrop-blur-2xl bg-white/40 border border-white/40 shadow-lg hover:shadow-2xl transition-all duration-300 p-6 rounded-2xl">
            <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 mb-6">
                <div>
                    <h2 className="text-2xl font-bold font-outfit text-gray-900 flex items-center gap-2">
                        Plan: <span className="text-indigo-600">{data?.current_plan || 'Free'}</span>
                    </h2>
                </div>
                <div>
                    <h2 className="text-xl font-bold font-outfit text-gray-900 flex items-center gap-2">
                        Estimated Next Bill <span className="text-gray-900">{formatCurrency(data?.next_bill_estimated || 0)}</span>
                    </h2>
                </div>
            </div>

            <div className="flex flex-col sm:flex-row gap-4 mt-6">
                <button
                    onClick={() => router.push('/pricing')}
                    className="w-full sm:w-auto px-6 py-3 bg-[#0f766e] hover:bg-[#0d645d] text-white rounded-xl font-medium transition-all shadow-sm hover:shadow-md hover:-translate-y-0.5 duration-300 text-center">
                    Upgrade
                </button>
                <button
                    onClick={handleManageBilling}
                    disabled={isManagingBilling}
                    className="w-full sm:w-auto px-6 py-3 glass-card glass-control backdrop-blur-md bg-white/50 border border-white/30 shadow-sm rounded-xl font-medium transition-all hover:shadow-md hover:-translate-y-0.5 duration-300 text-center disabled:opacity-75 disabled:cursor-not-allowed disabled:hover:translate-y-0 disabled:hover:shadow-sm">
                    {isManagingBilling ? "Redirecting..." : "Manage Billing"}
                </button>
                <button
                    onClick={() => router.push('/cost-dashboard')}
                    className="w-full sm:w-auto px-6 py-3 glass-card glass-control backdrop-blur-md bg-white/50 border border-white/30 text-gray-700 rounded-xl font-medium transition-all shadow-sm hover:shadow-md hover:-translate-y-0.5 duration-300 text-center">
                    View Detailed Costs
                </button>
            </div>
        </section>

        {/* Current Usage Section */}
        <section className="app-card ohc-growth-card glass-panel glass-card backdrop-blur-2xl bg-white/40 border border-white/40 shadow-lg hover:shadow-2xl transition-all duration-300 mt-4 rounded-2xl overflow-hidden">
          <div className="app-panel-header glass-panel backdrop-blur-lg bg-white/20 px-6 py-4 border-b border-white/40">
             <h2 className="app-panel-title text-xl font-bold font-outfit text-gray-900">Your Current Usage</h2>
          </div>
          <div className="app-panel-body p-6">
              <div className="flex flex-col gap-8">
                  {/* AI Actions */}
                  <div>
                      <div className="flex justify-between items-end mb-2">
                          <span className="font-medium text-gray-700 text-lg">AI Actions Used</span>
                          <span className="font-bold text-gray-900 text-lg">
                              {data?.ai_actions_used || 0} <span className="text-gray-500 font-normal text-base">{data?.ai_actions_limit != null && data.ai_actions_limit > 0 ? `/ ${data.ai_actions_limit}` : '/ Unlimited'}</span>
                          </span>
                      </div>
                      <div className="w-full bg-gray-200 rounded-full h-3 overflow-hidden">
                          <div
                              className="bg-gradient-to-r from-indigo-500 to-purple-600 h-3 rounded-full transition-all duration-500"
                              style={{ width: data?.ai_actions_limit != null && data.ai_actions_limit > 0 ? `${Math.min(100, ((data?.ai_actions_used || 0) / data.ai_actions_limit) * 100)}%` : '5%' }}>
                          </div>
                      </div>
                  </div>

                  {/* Storage */}
                  <div>
                      <div className="flex justify-between items-end mb-2">
                          <span className="font-medium text-gray-700 text-lg">Storage Used</span>
                          <span className="font-bold text-gray-900 text-lg">
                              {formatStorage(data?.storage_used_bytes || 0)} <span className="text-gray-500 font-normal text-base">{(data?.storage_limit_bytes ?? 0) > 0 ? `/ ${formatStorage(data!.storage_limit_bytes!)}` : '/ Unlimited'}</span>
                          </span>
                      </div>
                      <div className="w-full bg-gray-200 rounded-full h-3 overflow-hidden">
                          <div
                              className="bg-gradient-to-r from-blue-500 to-cyan-400 h-3 rounded-full transition-all duration-500"
                              style={{ width: (data?.storage_limit_bytes ?? 0) > 0 ? `${Math.min(100, ((data?.storage_used_bytes || 0) / data.storage_limit_bytes!) * 100)}%` : '5%' }}>
                          </div>
                      </div>
                  </div>
              </div>
          </div>
        </section>

      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        /* The .ohc-growth-card styles are now managed globally in globals.css for design token consistency */
      `}} />
    </div>
  );
}
