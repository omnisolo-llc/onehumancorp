"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function MyPlanPage() {
  const router = useRouter();
  const [myPlanData, setMyPlanData] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchPlanData() {
      try {
        const token = localStorage.getItem('token');
        const headers: Record<string, string> = {
            'Content-Type': 'application/json'
        };
        if (token) {
            headers['Authorization'] = `Bearer ${token}`;
        }

        const planRes = await fetch('/api/billing/my-plan', { headers });
        if (planRes.ok) {
            const planResult = await planRes.json();
            setMyPlanData(planResult);
        } else {
            console.error("Failed to fetch plan data:", planRes.status);
        }
      } catch (err) {
        console.error("Error fetching plan data", err);
      } finally {
        setLoading(false);
      }
    }
    fetchPlanData();
  }, []);

  const formatCurrency = (cents: number) => {
      return new Intl.NumberFormat('en-US', {
          style: 'currency',
          currency: 'USD',
      }).format(cents / 100);
  };

  const formatStorage = (bytes: number) => {
      const mb = bytes / (1024 * 1024);
      if (mb < 1) return "< 1 MB";
      if (mb >= 1024) return parseFloat((mb / 1024).toFixed(2)) + " GB";
      return parseFloat(mb.toFixed(1)) + " MB";
  };

  if (loading) {
    return (
      <div className="flex justify-center items-center h-screen" data-testid="plan-loading">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
      </div>
    );
  }

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 text-gray-900">
      <header className="px-4 md:px-6 py-4 flex flex-col md:flex-row items-center justify-between border-b gap-4 sticky top-0 z-50 bg-white/70 backdrop-blur-xl saturate-200 border-b-white/40 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-center md:text-left text-gray-900 tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-gray-900 to-gray-600">My Plan</h1>
        <div className="flex gap-2">
            <button onClick={() => router.push('/dashboard')} className="min-w-[44px] min-h-[44px] px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-800 rounded-xl text-sm font-medium transition-all active:scale-95 shadow-sm flex items-center justify-center">
            Back to Dashboard
            </button>
        </div>
      </header>

      <main id="my-plan-screen" className="p-4 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">
        <section id="my-plan-section" className="app-panel bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-2xl shadow-sm">
          <div className="app-panel-header flex justify-between items-center bg-transparent border-b border-white/40 px-6 py-4">
             <h2 className="app-panel-title text-xl font-bold font-outfit text-gray-900">Plan: <span id="my-plan-name" className="text-indigo-600">{myPlanData?.current_plan || 'Free'}</span></h2>
             <button
               onClick={() => router.push('/pricing')}
               className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-xl text-sm font-medium transition-all shadow-sm">
               Upgrade
             </button>
          </div>
          <div className="app-panel-body p-6">
              <h3 className="text-lg font-semibold text-gray-800 mb-4">Your Current Usage</h3>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                  <div className="p-4 rounded-xl app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40">
                      <div className="stat-title text-sm font-medium text-gray-500">AI actions used this month</div>
                      <p className="text-2xl font-bold text-gray-900 mt-1">{myPlanData?.ai_actions_used || 0} <span className="text-sm text-gray-500 font-normal">{myPlanData?.ai_actions_limit != null ? `/ ${myPlanData.ai_actions_limit}` : '/ Unlimited'}</span></p>
                  </div>
                  <div className="p-4 rounded-xl app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40">
                      <div className="stat-title text-sm font-medium text-gray-500">Storage used</div>
                      <p className="text-2xl font-bold text-gray-900 mt-1">{formatStorage(myPlanData?.storage_used_bytes || 0)} <span className="text-sm text-gray-500 font-normal">{myPlanData?.storage_limit_bytes != null ? `/ ${formatStorage(myPlanData.storage_limit_bytes)}` : '/ Unlimited'}</span></p>
                  </div>
                  <div className="p-4 rounded-xl app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40">
                      <div className="stat-title text-sm font-medium text-gray-500">Estimated Next Bill</div>
                      <p id="my-plan-next-bill" className="text-2xl font-bold text-gray-900 mt-1">{formatCurrency(myPlanData?.next_bill_estimated || 0)}</p>
                  </div>
              </div>

              <div className="mt-8 flex justify-end">
                <button
                    onClick={() => router.push('/cost-dashboard')}
                    className="px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-800 rounded-xl text-sm font-medium transition-all active:scale-95 shadow-sm flex items-center justify-center">
                    View Cost Transparency Dashboard
                </button>
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
