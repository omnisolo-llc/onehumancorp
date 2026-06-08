'use client';

import React, { useState } from 'react';
import Link from 'next/link';

export default function ManageSubscriptionPage() {
  const [status, setStatus] = useState<'Active' | 'Paused' | 'Canceled'>('Active');
  const [notification, setNotification] = useState<string | null>(null);

  const handlePause = () => {
    setStatus('Paused');
    setNotification('Your subscription has been paused.');
    setTimeout(() => setNotification(null), 3000);
  };

  const handleSkip = () => {
    setNotification('Your next delivery has been skipped.');
    setTimeout(() => setNotification(null), 3000);
  };

  const handleCancel = () => {
    setStatus('Canceled');
    setNotification('Your subscription has been canceled.');
    setTimeout(() => setNotification(null), 3000);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: "#F5F5F7" }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: "rgba(255, 255, 255, 0.65)", backdropFilter: "blur(30px) saturate(210%)", borderBottom: "1px solid rgba(255, 255, 255, 0.4)", position: "sticky", top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: "#1D1D1F", letterSpacing: "-0.02em" }}>Manage Subscription</h1>
        <Link href="/dashboard" className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">Back to Dashboard</Link>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col gap-6">
        {notification && (
          <div className="p-4 bg-green-50 border border-green-200 text-green-800 rounded-lg text-sm font-medium">
            {notification}
          </div>
        )}

        <div className="p-6 shadow-sm flex flex-col gap-4 mb-4" style={{ background: "rgba(255, 255, 255, 0.65)", backdropFilter: "blur(30px) saturate(210%)", border: "1px solid rgba(255, 255, 255, 0.4)", borderRadius: "16px" }}>
          <div className="flex justify-between items-start pb-4 border-b border-gray-100">
            <div>
              <h2 className="text-lg font-semibold text-gray-900">Premium Coffee Beans</h2>
              <p className="text-sm text-gray-500">Delivered Monthly</p>
            </div>
            <span className={`px-2 py-1 text-xs font-bold rounded-full ${status === 'Active' ? 'bg-green-100 text-green-700' : status === 'Paused' ? 'bg-yellow-100 text-yellow-700' : 'bg-red-100 text-red-700'}`}>
              {status}
            </span>
          </div>

          <div className="flex flex-col gap-3 mt-2">
            <button
              onClick={handleSkip}
              disabled={status !== 'Active'}
              className="w-full px-4 py-3 bg-white text-indigo-600 border border-indigo-200 rounded-lg font-medium hover:bg-indigo-50 transition-colors shadow-sm disabled:opacity-50"
            >
              Skip Next Delivery
            </button>
            <button
              onClick={handlePause}
              disabled={status !== 'Active'}
              className="w-full px-4 py-3 bg-white text-yellow-600 border border-yellow-200 rounded-lg font-medium hover:bg-yellow-50 transition-colors shadow-sm disabled:opacity-50"
            >
              Pause Subscription
            </button>
            <button
              onClick={handleCancel}
              disabled={status === 'Canceled'}
              className="w-full px-4 py-3 bg-white text-red-600 border border-red-200 rounded-lg font-medium hover:bg-red-50 transition-colors shadow-sm disabled:opacity-50"
            >
              Cancel Subscription
            </button>
          </div>
        </div>
      </main>
    </div>
  );
}
