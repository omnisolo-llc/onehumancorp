"use client";

import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

interface MyPlanData {
  current_plan: string;
  ai_actions_used: number;
  ai_actions_limit: number | null;
  storage_used_mb: number;
  storage_limit_mb: number | null;
  estimated_next_bill_cents: number;
}

export default function MyPlanPage() {
  const router = useRouter();
  const [data, setData] = useState<MyPlanData | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchData() {
      try {
        const headers: Record<string, string> = {
            'Content-Type': 'application/json'
        };
        const token = localStorage.getItem('ohc_auth_token');
        if (token) {
            headers['Authorization'] = `Bearer ${token}`;
        }
        const res = await fetch('/api/billing/my-plan', { headers });
        if (res.ok) {
          const result = await res.json();
          setData(result);
        } else {
          console.error("Failed to fetch plan data:", res.status);
        }
      } catch (err) {
        console.error("Error fetching plan data:", err);
      } finally {
        setLoading(false);
      }
    }
    fetchData();
  }, []);

  const formatCurrency = (cents: number) => {
      return new Intl.NumberFormat('en-US', {
          style: 'currency',
          currency: 'USD'
      }).format(cents / 100);
  };

  if (loading) {
      return (
          <div className="flex flex-col min-h-screen items-center justify-center bg-[#F5F5F7] text-gray-900">
              <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900"></div>
          </div>
      );
  }

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 text-gray-900 w-full overflow-x-hidden">
      <header className="px-4 py-4 flex items-center justify-between sticky top-0 z-50 bg-white/70 backdrop-blur-xl saturate-200 border-b border-white/40 shadow-sm w-full">
          <h1 className="text-xl md:text-2xl font-bold font-outfit text-gray-900 tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-gray-900 to-gray-600">My Plan</h1>
        <button onClick={() => router.push('/dashboard')} className="min-w-[44px] min-h-[44px] px-3 py-2 bg-gray-100 rounded-xl text-sm font-medium text-gray-800 hover:bg-gray-200 transition-colors flex items-center justify-center">
          Back
        </button>
      </header>

      <main className="p-4 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">
          <section className="app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-2xl p-6 shadow-sm">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">Status Snapshot</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="p-4 bg-white/50 rounded-xl border border-gray-100 flex flex-col">
                     <h2 className="text-sm font-medium text-gray-500 mb-1">Plan:</h2>
                     <p className="text-2xl font-bold text-gray-900">{data?.current_plan || 'Free'}</p>
                </div>
                 <div className="p-4 bg-white/50 rounded-xl border border-gray-100 flex flex-col">
                     <h2 className="text-sm font-medium text-gray-500 mb-1">Estimated Next Bill:</h2>
                     <p className="text-2xl font-bold text-gray-900">{formatCurrency(data?.estimated_next_bill_cents || 0)}</p>
                </div>
            </div>

            <div className="mt-6 flex justify-end">
                <button
                  onClick={() => router.push('/pricing')}
                  className="px-6 py-3 bg-indigo-600 text-white rounded-xl font-medium hover:bg-indigo-700 transition-colors shadow-sm"
                >
                    View Upgrade Plans
                </button>
            </div>
          </section>

          <section className="app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-2xl p-6 shadow-sm mt-4">
             <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">Your Current Usage</h2>
             <div className="space-y-6">
                <div>
                    <div className="flex justify-between items-center mb-2">
                        <span className="font-medium text-gray-900">AI actions used this month</span>
                        <span className="text-sm text-gray-500">
                             {data?.ai_actions_used || 0} / {data?.ai_actions_limit ? data.ai_actions_limit : 'Unlimited'}
                        </span>
                    </div>
                     {data?.ai_actions_limit && (
                        <div className="w-full bg-gray-200 rounded-full h-2.5">
                            <div className="bg-indigo-600 h-2.5 rounded-full" style={{ width: `${Math.min(((data?.ai_actions_used || 0) / data.ai_actions_limit) * 100, 100)}%` }}></div>
                        </div>
                    )}
                </div>

                <div>
                     <div className="flex justify-between items-center mb-2">
                        <span className="font-medium text-gray-900">Storage used</span>
                        <span className="text-sm text-gray-500">
                             {data?.storage_used_mb || 0} MB / {data?.storage_limit_mb ? `${data.storage_limit_mb} MB` : 'Unlimited'}
                        </span>
                    </div>
                    {data?.storage_limit_mb && (
                        <div className="w-full bg-gray-200 rounded-full h-2.5">
                            <div className="bg-indigo-600 h-2.5 rounded-full" style={{ width: `${Math.min(((data?.storage_used_mb || 0) / data.storage_limit_mb) * 100, 100)}%` }}></div>
                        </div>
                    )}
                </div>
             </div>
          </section>

          <div className="flex justify-center mt-8">
               <PoweredByOHC tenantId="ohc" />
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
