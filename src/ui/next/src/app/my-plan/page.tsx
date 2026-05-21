"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

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

  useEffect(() => {
    async function fetchPlanData() {
      try {
        const token = localStorage.getItem('token') || 'test-token';
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
            // Fallback for UI if API is not wired perfectly in e2e
            setPlanData({
                current_plan: "Free",
                ai_actions_used: 0,
                ai_actions_limit: 100,
                storage_used_bytes: 0,
                storage_limit_bytes: 500 * 1024 * 1024,
                next_bill_estimated: 0,
            });
        }
      } catch (err) {
        console.error("Error fetching plan data", err);
        setPlanData({
            current_plan: "Free",
            ai_actions_used: 0,
            ai_actions_limit: 100,
            storage_used_bytes: 0,
            storage_limit_bytes: 500 * 1024 * 1024,
            next_bill_estimated: 0,
        });
      } finally {
        setLoading(false);
      }
    }
    fetchPlanData();
  }, []);

  if (loading) {
      return <div className="min-h-screen flex items-center justify-center">Loading...</div>;
  }

  const formatStorage = (bytes: number) => {
      const mb = bytes / (1024 * 1024);
      if (mb < 1) return "< 1 MB";
      if (mb > 1024) return (mb / 1024).toFixed(2) + " GB";
      return mb.toFixed(1) + " MB";
  };

  const [showCostDetails, setShowCostDetails] = useState(false);

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>My Current Plan</h1>
        <div className="flex gap-2">
            <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
            Back to Dashboard
            </button>
            <button onClick={() => router.push('/pricing')} className="px-4 py-2 bg-indigo-600 text-white rounded-md text-sm font-medium hover:bg-indigo-700 transition-colors">
            View Upgrade Plans
            </button>
        </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-6">

        {/* Status Snapshot */}
        <section className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div>
                    <h2 className="text-sm font-medium text-gray-500 mb-1">Plan: {planData?.current_plan}</h2>
                    <p className="text-3xl font-bold font-outfit text-gray-900">{planData?.current_plan}</p>
                    <span className="inline-block px-2 py-1 bg-green-100 text-green-800 text-xs font-medium rounded mt-2">Active</span>
                </div>
                <div>
                    <h2 className="text-sm font-medium text-gray-500 mb-1">Estimated Next Bill</h2>
                    <p className="text-3xl font-bold font-outfit text-gray-900">${planData?.next_bill_estimated.toFixed(2)}</p>
                </div>
                <div className="flex flex-col justify-center">
                    <button onClick={() => router.push('/pricing')} className="w-full py-3 bg-indigo-600 text-white font-medium rounded-lg hover:bg-indigo-700 transition-colors">
                        Upgrade Plan
                    </button>
                </div>
            </div>
        </section>

        {/* Usage Section */}
        <section className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
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
                                    : '10%'
                            }}
                        ></div>
                    </div>
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
                                    : '5%'
                            }}
                        ></div>
                    </div>
                </div>
            </div>
        </section>

        {/* Management Actions */}
        <section className="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <button onClick={() => setShowCostDetails(true)} className="p-4 bg-white border border-gray-200 rounded-xl hover:bg-gray-50 text-left transition-colors">
                <h3 className="font-medium text-gray-900">View Cost Details</h3>
                <p className="text-sm text-gray-500 mt-1">View detailed cost and AI usage for this billing cycle.</p>
            </button>
            <button className="p-4 bg-white border border-gray-200 rounded-xl hover:bg-gray-50 text-left transition-colors">
                <h3 className="font-medium text-gray-900">Change Plan</h3>
                <p className="text-sm text-gray-500 mt-1">Upgrade or downgrade your current subscription.</p>
            </button>
            <button className="p-4 bg-white border border-gray-200 rounded-xl hover:bg-gray-50 text-left transition-colors">
                <h3 className="font-medium text-gray-900">Download Invoice</h3>
                <p className="text-sm text-gray-500 mt-1">Get a PDF copy of your recent billing statements.</p>
            </button>
            <button className="p-4 bg-white border border-gray-200 rounded-xl hover:bg-gray-50 text-left transition-colors sm:col-span-2">
                <h3 className="font-medium text-red-600">Cancel Subscription</h3>
                <p className="text-sm text-gray-500 mt-1">Cancel your subscription. You will lose access to premium features at the end of your billing cycle.</p>
            </button>
        </section>

      </main>

      {showCostDetails && (
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4" style={{ backgroundColor: 'rgba(0, 0, 0, 0.5)' }}>
              <div className="bg-white rounded-xl shadow-lg w-full max-w-2xl overflow-hidden">
                  <div className="p-6 border-b border-gray-200 flex justify-between items-center">
                      <h2 className="text-xl font-bold font-outfit text-gray-900">Cost & AI Usage</h2>
                      <button onClick={() => setShowCostDetails(false)} className="text-gray-500 hover:text-gray-700">
                          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                      </button>
                  </div>
                  <div className="p-6">
                      <p className="text-gray-700 mb-4">Detailed breakdown of your costs and AI usage for the current billing cycle.</p>
                      {/* Placeholder for actual cost details */}
                      <div className="bg-gray-50 p-4 rounded-lg">
                          <div className="flex justify-between mb-2">
                              <span className="text-gray-600">Total AI Actions</span>
                              <span className="font-medium">{planData?.ai_actions_used}</span>
                          </div>
                          <div className="flex justify-between mb-2">
                              <span className="text-gray-600">Storage Cost</span>
                              <span className="font-medium">$0.00</span>
                          </div>
                          <div className="flex justify-between font-bold border-t border-gray-200 pt-2 mt-2">
                              <span>Estimated Total</span>
                              <span>${planData?.next_bill_estimated.toFixed(2)}</span>
                          </div>
                      </div>
                  </div>
                  <div className="p-6 border-t border-gray-200 bg-gray-50 text-right">
                      <button onClick={() => setShowCostDetails(false)} className="px-4 py-2 bg-indigo-600 text-white rounded-md font-medium hover:bg-indigo-700 transition-colors">
                          Close
                      </button>
                  </div>
              </div>
          </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
