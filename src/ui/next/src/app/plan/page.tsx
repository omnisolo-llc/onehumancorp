"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';

export default function PlanPage() {
  const router = useRouter();
  const [data, setData] = useState<{
    current_plan: string;
    ai_actions_used: number;
    storage_used_bytes: number;
    next_bill_estimated: number;
  } | null>(null);

  useEffect(() => {
    // Attempt to fetch data if the user is authenticated and the endpoint exists
    const token = typeof window !== 'undefined' ? localStorage.getItem('token') : null;
    const headers = token ? { Authorization: `Bearer ${token}` } : undefined;

    fetch('/api/billing/my-plan', { headers })
      .then((res) => {
        if (res.ok) return res.json();
        throw new Error('Failed to load plan details');
      })
      .then((data) => setData(data))
      .catch(() => {
          // If the backend isn't mockable or errors out, fallback to empty state
      });
  }, []);

  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4">
      <main className="max-w-4xl w-full">
        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-8">My Plan</h1>

        <div className="app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 hover:shadow-xl transition-shadow duration-300 rounded-2xl p-8">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">Your Current Usage</h2>

            <div className="space-y-6">
                <div>
                    <h3 className="text-lg font-semibold text-gray-800">Current Plan</h3>
                    <p className="text-gray-600 mt-1">{data?.current_plan || 'Free'}</p>
                </div>

                <div>
                    <h2 className="text-lg font-semibold text-gray-800">Estimated Next Bill:</h2>
                    <p className="text-gray-600 mt-1">${(data?.next_bill_estimated || 0).toFixed(2)}</p>
                </div>

                <div>
                    <span className="block text-sm font-medium text-gray-700">AI actions used this month</span>
                    <p className="text-xl font-semibold text-gray-900 mt-1">{data?.ai_actions_used || 0}</p>
                </div>

                <div>
                    <span className="block text-sm font-medium text-gray-700">Storage used</span>
                    <p className="text-xl font-semibold text-gray-900 mt-1">{(data?.storage_used_bytes || 0) / (1024 * 1024)} MB</p>
                </div>
            </div>

            <div className="mt-8 pt-6 border-t border-gray-200 flex gap-4">
                <Link href="/pricing" passHref>
                    <button className="bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-2 px-6 rounded-lg transition-colors">
                        Upgrade
                    </button>
                </Link>
            </div>
        </div>
      </main>
    </div>
  );
}
