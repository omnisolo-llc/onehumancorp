'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function CustomerPortalPage() {
  const [subscriptions, setSubscriptions] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchSubscriptions = async () => {
    try {
      const response = await fetch('/api/v1/subscription/customer/my');
      if (!response.ok) throw new Error('Failed to load subscriptions');
      const data = await response.json();
      setSubscriptions(data);
    } catch (err) {
      setError('Could not load your subscriptions.');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchSubscriptions();
  }, []);

  const handleAction = async (id: string, action: string) => {
    try {
      const response = await fetch(`/api/v1/subscription/customer/${id}/${action}`, {
        method: 'POST',
      });
      if (response.ok) {
        fetchSubscriptions();
      }
    } catch (err) {
      console.error('Action failed', err);
    }
  };

  if (loading) return <div className="p-8 text-center font-inter">Loading portal...</div>;

  return (
    <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col font-inter pb-20">
      <header className="mb-6 border-b border-gray-200 pb-4">
        <h1 className="text-2xl font-bold font-outfit text-gray-900">Your Subscriptions</h1>
        <p className="text-sm text-gray-500">Manage your recurring orders here.</p>
      </header>

      {error && (
        <div className="mb-4 rounded-xl bg-red-50 p-4 text-sm text-red-700 font-semibold border border-red-100">
          {error}
        </div>
      )}

      {subscriptions.length === 0 && !error && (
        <div className="flex-1 flex flex-col items-center justify-center text-center p-8">
           <div className="text-4xl mb-4">📦</div>
           <h2 className="text-lg font-bold text-gray-800">No active subscriptions</h2>
           <p className="text-sm text-gray-500 mt-2">You haven't subscribed to any products yet.</p>
           <Link href="/" className="mt-6 text-blue-600 font-bold hover:underline">Explore Products</Link>
        </div>
      )}

      <div className="space-y-4">
        {subscriptions.map((sub) => (
          <div key={sub.id} className="p-5 rounded-2xl bg-white shadow-sm border border-gray-100 flex flex-col gap-4">
            <div className="flex justify-between items-start">
              <div>
                <h3 className="font-bold text-gray-900">{sub.plan_name}</h3>
                <p className="text-xs text-gray-500 uppercase font-semibold tracking-wider">{sub.frequency}</p>
              </div>
              <span className={`px-2 py-1 rounded-full text-[10px] font-bold uppercase tracking-widest ${
                sub.status === 'ACTIVE' ? 'bg-green-100 text-green-700' : 'bg-amber-100 text-amber-700'
              }`}>
                {sub.status}
              </span>
            </div>

            <div className="flex justify-between items-end">
              <div className="text-lg font-bold text-gray-900">
                ${(sub.amount / 100).toFixed(2)}
              </div>
              <div className="text-xs text-gray-400">
                Next: {sub.current_period_end > 0 ? new Date(sub.current_period_end * 1000).toLocaleDateString() : 'TBD'}
              </div>
            </div>

            <div className="flex gap-2 pt-2 border-t border-gray-50">
              {sub.status === 'ACTIVE' ? (
                <button
                  onClick={() => handleAction(sub.id, 'pause')}
                  className="flex-1 py-2 text-xs font-bold text-amber-700 bg-amber-50 rounded-lg hover:bg-amber-100 transition-colors"
                >
                  Pause
                </button>
              ) : (
                <button
                  onClick={() => handleAction(sub.id, 'resume')}
                  className="flex-1 py-2 text-xs font-bold text-green-700 bg-green-50 rounded-lg hover:bg-green-100 transition-colors"
                >
                  Resume
                </button>
              )}
              <button
                onClick={() => handleAction(sub.id, 'cancel')}
                className="flex-1 py-2 text-xs font-bold text-red-700 bg-red-50 rounded-lg hover:bg-red-100 transition-colors"
              >
                Cancel
              </button>
            </div>
          </div>
        ))}
      </div>

      <div className="mt-8 pt-6 border-t border-gray-200">
         <Link href="/" className="text-sm font-semibold text-gray-400 hover:text-gray-600 transition-colors">
           &larr; Back to Shop
         </Link>
      </div>
    </div>
  );
}
