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
}

export default function MyPlanPage() {
  const router = useRouter();
  const [data, setData] = useState<MyPlanData | null>(null);
  const [loading, setLoading] = useState(true);

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
    if (bytes === 0) return '0 GB';
    const gb = bytes / (1024 * 1024 * 1024);
    return `${gb.toFixed(2)} GB`;
  };

  const formatCurrency = (amount: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(amount);
  };

  if (loading) {
    return (
      <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 justify-center items-center">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
        <p className="mt-4 text-gray-600">Loading your plan data...</p>
      </div>
    );
  }

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 text-gray-900 w-full overflow-x-hidden">
      <header className="px-4 py-4 flex items-center justify-between sticky top-0 z-50 bg-white/70 backdrop-blur-xl saturate-200 border-b border-white/40 shadow-sm w-full">
        <div className="flex items-center gap-3">
          <button onClick={() => router.push('/dashboard')} className="min-w-[44px] min-h-[44px] px-3 py-2 bg-gray-100 rounded-xl text-sm font-medium text-gray-800 hover:bg-gray-200 transition-colors flex items-center justify-center">
            Back
          </button>
          <WithTooltip id="my-plan-tooltip" defaultText="View and manage your subscription plan and usage.">
            <h1 className="text-xl md:text-2xl font-bold font-outfit text-gray-900 tracking-tight">My Plan</h1>
          </WithTooltip>
        </div>
      </header>

      <main className="p-4 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">

        {/* Status Snapshot */}
        <section className="app-panel bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-2xl shadow-lg p-6 dark:bg-gray-900/70 dark:border-white/10">
            <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 mb-6">
                <div>
                    <h2 className="text-2xl font-bold font-outfit text-gray-900 flex items-center gap-2">
                        Plan: <span className="text-indigo-600">{data?.current_plan || 'Free'}</span>
                    </h2>
                </div>
                <div>
                    <h2 className="text-xl font-bold font-outfit text-gray-900 flex items-center gap-2">
                        Estimated Next Bill: <span className="text-green-600">{formatCurrency(data?.next_bill_estimated || 0)}</span>
                    </h2>
                </div>
            </div>

            <div className="flex flex-col sm:flex-row gap-4 mt-6">
                <button
                    onClick={() => router.push('/pricing')}
                    className="w-full sm:w-auto px-6 py-3 bg-indigo-600 hover:bg-indigo-700 text-white rounded-xl font-medium transition-all shadow-sm text-center">
                    View Upgrade Plans
                </button>
                <button
                    onClick={() => router.push('/cost-dashboard')}
                    className="w-full sm:w-auto px-6 py-3 bg-white hover:bg-gray-50 text-gray-700 border border-gray-300 rounded-xl font-medium transition-all shadow-sm text-center">
                    View Detailed Costs
                </button>
            </div>
        </section>

        {/* Current Usage Section */}
        <section className="app-panel bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-2xl shadow-lg mt-4 dark:bg-gray-900/70 dark:border-white/10">
          <div className="app-panel-header px-6 py-4 border-b border-white/40 bg-transparent">
             <h2 className="app-panel-title text-xl font-bold font-outfit text-gray-900">Your Current Usage</h2>
          </div>
          <div className="app-panel-body p-6">
              <div className="flex flex-col gap-8">
                  {/* AI Actions */}
                  <div>
                      <div className="flex justify-between items-end mb-2">
                          <span className="font-medium text-gray-700 text-lg">AI actions used this month</span>
                          <span className="font-bold text-gray-900 text-lg">
                              {data?.ai_actions_used || 0} <span className="text-gray-500 font-normal text-base">{data?.ai_actions_limit != null ? `/ ${data.ai_actions_limit}` : '/ Unlimited'}</span>
                          </span>
                      </div>
                      <div className="w-full bg-gray-200 rounded-full h-3 overflow-hidden">
                          <div
                              className="bg-indigo-500 h-3 rounded-full transition-all duration-500"
                              style={{ width: data?.ai_actions_limit ? `${Math.min(100, ((data?.ai_actions_used || 0) / data.ai_actions_limit) * 100)}%` : '5%' }}>
                          </div>
                      </div>
                  </div>

                  {/* Storage */}
                  <div>
                      <div className="flex justify-between items-end mb-2">
                          <span className="font-medium text-gray-700 text-lg">Storage used</span>
                          <span className="font-bold text-gray-900 text-lg">
                              {formatStorage(data?.storage_used_bytes || 0)} <span className="text-gray-500 font-normal text-base">{data?.storage_limit_bytes != null ? `/ ${formatStorage(data.storage_limit_bytes)}` : '/ Unlimited'}</span>
                          </span>
                      </div>
                      <div className="w-full bg-gray-200 rounded-full h-3 overflow-hidden">
                          <div
                              className="bg-blue-500 h-3 rounded-full transition-all duration-500"
                              style={{ width: data?.storage_limit_bytes ? `${Math.min(100, ((data?.storage_used_bytes || 0) / data.storage_limit_bytes) * 100)}%` : '5%' }}>
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
      `}} />
    </div>
  );
}
