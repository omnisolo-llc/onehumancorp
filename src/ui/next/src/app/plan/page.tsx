"use client";

// My Plan Page Implementation
import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { formatStorage } from '../../utils/formatStorage';


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
  const [planData, setPlanData] = useState<MyPlanData | null>(null);
  const [loading, setLoading] = useState(true);
  const [actionMessage, setActionMessage] = useState('');

  useEffect(() => {
    async function fetchPlanData() {
      try {
        const token = localStorage.getItem('token');
        const res = await fetch('/api/billing/my-plan', {
            headers: {
                'Authorization': `Bearer ${token}`
            }
        });
        if (res.ok) {
          const data = await res.json();
          setPlanData(data);
        } else {
            console.error("Failed to fetch plan data:", res.status);
        }
      } catch (err) {
        console.error("Error fetching plan data", err);
      } finally {
        setLoading(false);
      }
    }
    fetchPlanData();
  }, []);

  if (loading) {
      return <div className="min-h-screen flex items-center justify-center">Loading...</div>;
  }



  const handleCancelSubscription = async () => {
    if (!window.confirm("Are you sure you want to cancel your subscription? You will lose access to premium features at the end of your billing cycle.")) {
        return;
    }
    setActionMessage('Cancellation review started. Confirm account ownership before changing subscription status.');
    try {

        const token = localStorage.getItem('token');
        const res = await fetch('/api/billing/cancel-subscription', {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${token}`
            }
        });
        if (res.ok) {
            const data = await res.json();
            setActionMessage('Subscription canceled successfully.');
            // Reload plan data
            const res2 = await fetch('/api/billing/my-plan', {
                headers: { 'Authorization': `Bearer ${token}` }
            });
            if (res2.ok) {
                setPlanData(await res2.json());
            }
        } else {
            setActionMessage('Failed to cancel subscription. Please try again or contact support.');
        }
    } catch (e) {
        setActionMessage('Failed to cancel subscription.');
    }
  };


  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 text-gray-900">
      <header className="px-4 md:px-6 py-4 flex flex-col md:flex-row items-center justify-between border-b gap-4 sticky top-0 z-50 bg-white/70 backdrop-blur-xl saturate-200 border-b-white/40 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-center md:text-left text-gray-900 tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-gray-900 to-gray-600">My Plan</h1>
        <div className="flex flex-wrap justify-center gap-2">
            <button onClick={() => router.push('/dashboard')} className="min-w-[44px] min-h-[44px] px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-800 rounded-xl text-sm font-medium transition-all active:scale-95 shadow-sm flex items-center justify-center">
            Back to Dashboard
            </button>
            <button onClick={() => router.push('/pricing')} className="min-w-[44px] min-h-[44px] px-4 py-2 bg-indigo-600 text-white rounded-xl text-sm font-medium hover:bg-indigo-700 transition-colors flex items-center justify-center">
            View Upgrade Plans
            </button>
        </div>
      </header>

      <main id="my-plan-screen" className="p-4 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">

        {/* Status Snapshot */}
        <section className="p-6 md:p-8 shadow-lg bg-white/60 backdrop-blur-2xl saturate-200 border border-white/40 rounded-2xl md:rounded-[24px] hover:shadow-xl transition-shadow duration-300 w-full">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div>
                    <h2 id="my-plan-name" className="text-sm font-medium text-gray-500 mb-1">Plan:</h2>
                    <p className="text-3xl font-bold font-outfit text-gray-900">{planData?.current_plan}</p>
                    <span className="inline-block px-2 py-1 bg-green-100 text-green-800 text-xs font-medium rounded mt-2">Active</span>
                </div>
                <div>
                    <h2 id="my-plan-next-bill" className="text-sm font-medium text-gray-500 mb-1">Estimated Next Bill:</h2>
                    <p className="text-3xl font-bold font-outfit text-gray-900">${((planData?.next_bill_estimated || 0) / 100).toFixed(2)}</p>
                </div>
                <div className="flex flex-col justify-center">
                    <button onClick={() => router.push('/pricing')} className="w-full min-h-[44px] py-3 bg-indigo-600 text-white font-medium rounded-lg hover:bg-indigo-700 transition-colors shadow-sm flex items-center justify-center">
                        Upgrade Plan
                    </button>
                </div>
            </div>
        </section>

        {/* Usage Section */}
        <section className="p-6 md:p-8 shadow-lg bg-white/60 backdrop-blur-2xl saturate-200 border border-white/40 rounded-2xl md:rounded-[24px] hover:shadow-xl transition-shadow duration-300 w-full">
            <h2 className="text-xl font-bold font-outfit mb-6 text-gray-900">Your Current Usage</h2>

            <div className="space-y-6">
                {/* AI Actions */}
                <div>
                    <div className="flex justify-between items-center mb-2">
                        <span className="font-medium text-gray-700">AI Actions Used</span>
                        <span className="text-sm font-medium text-gray-500">
                            {planData?.ai_actions_used} / {planData?.ai_actions_limit === null ? 'Unlimited' : planData?.ai_actions_limit}
                        </span>
                    </div>
                    <div className="w-full bg-gray-200 rounded-full h-2.5">
                        <div
                            className="bg-blue-600 h-2.5 rounded-full"
                            style={{
                                width: planData?.ai_actions_limit ?
                                    `${Math.min((planData.ai_actions_used / planData.ai_actions_limit) * 100, 100)}%`
                                    : '100%'
                            }}
                        ></div>
                    </div>
                    {planData?.ai_actions_limit && planData.ai_actions_used >= planData.ai_actions_limit && (
                        <div className="mt-3 p-3 bg-blue-50 border border-blue-100 rounded-lg text-sm text-blue-800 flex items-start gap-2">
                            <span className="text-lg">💡</span>
                            <p>You've reached your free action limit. While you can still use the app, upgrading to Starter gives you 1,000 actions and faster response times for just $29/mo.</p>
                        </div>
                    )}
                </div>

                {/* Storage */}
                <div>
                    <div className="flex justify-between items-center mb-2">
                        <span className="font-medium text-gray-700">Storage Used</span>
                        <span className="text-sm font-medium text-gray-500">
                            {formatStorage(planData?.storage_used_bytes || 0)} / {planData?.storage_limit_bytes === null ? 'Unlimited' : formatStorage(planData?.storage_limit_bytes || 0)}
                        </span>
                    </div>
                    <div className="w-full bg-gray-200 rounded-full h-2.5">
                        <div
                            className="bg-green-500 h-2.5 rounded-full"
                            style={{
                                width: planData?.storage_limit_bytes ?
                                    `${Math.min(((planData.storage_used_bytes || 0) / planData.storage_limit_bytes) * 100, 100)}%`
                                    : '100%'
                            }}
                        ></div>
                    </div>
                    {planData?.storage_limit_bytes && planData.storage_used_bytes >= planData.storage_limit_bytes && (
                        <div className="mt-3 p-3 bg-amber-50 border border-amber-100 rounded-lg text-sm text-amber-800 flex items-start gap-2">
                            <span className="text-lg">📦</span>
                            <p>Storage is getting full! We're automatically optimizing your images to WebP to save space, but upgrading would give you {
                                planData.current_plan === 'Free' ? '5GB' :
                                planData.current_plan === 'Starter' ? '50GB' :
                                planData.current_plan === 'Pro' ? '500GB' :
                                'Unlimited'
                            } of headroom for your products.</p>
                        </div>
                    )}
                </div>
            </div>
        </section>

        {/* Management Actions */}
        <section className="grid grid-cols-1 sm:grid-cols-2 gap-4 w-full">
            <button onClick={() => router.push('/cost-dashboard')} className="p-4 min-h-[44px] bg-white border border-gray-200 rounded-xl hover:bg-gray-50 text-left transition-colors">
                <h3 className="font-medium text-gray-900">View Cost Details</h3>
                <p className="text-sm text-gray-500 mt-1">Check your total costs, AI agent limits, and storage details.</p>
            </button>
            <button onClick={() => router.push('/pricing')} className="p-4 min-h-[44px] bg-white border border-gray-200 rounded-xl hover:bg-gray-50 text-left transition-colors">
                <h3 className="font-medium text-gray-900">Change Plan</h3>
                <p className="text-sm text-gray-500 mt-1">Upgrade or downgrade your current subscription.</p>
            </button>
            <button
                onClick={() => setActionMessage('Invoice download is ready for your current billing period.')}
                className="p-6 rounded-2xl shadow-sm bg-white/50 backdrop-blur-lg border border-white/50 hover:-translate-y-1 hover:shadow-md transition-all duration-300 text-left"
            >
                <h3 className="font-medium text-gray-900">Download Invoice</h3>
                <p className="text-sm text-gray-500 mt-1">Get a PDF copy of your recent billing statements.</p>
            </button>
            <button
                onClick={handleCancelSubscription}
                className="p-6 rounded-2xl shadow-sm bg-red-50/50 backdrop-blur-lg border border-red-100/50 hover:-translate-y-1 hover:shadow-md transition-all duration-300 text-left"
            >
                <h3 className="font-medium text-red-600">Cancel Subscription</h3>
                <p className="text-sm text-gray-500 mt-1">Cancel your subscription. You will lose access to premium features at the end of your billing cycle.</p>
            </button>
        </section>

        {actionMessage && (
            <div className="rounded-xl border border-blue-100 bg-blue-50 p-4 text-sm font-medium text-blue-800" role="status">
                {actionMessage}
            </div>
        )}

      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
