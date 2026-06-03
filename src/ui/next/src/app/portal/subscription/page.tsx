'use client';

import React, { useState } from 'react';

export default function SubscriptionPortalPage() {
  const [status, setStatus] = useState('active');

  const handleAction = (action: string) => {
    setStatus(action === 'pause' ? 'paused' : action === 'cancel' ? 'canceled' : 'active');
  };

  return (
    <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col font-inter" id="magic-link-portal">
      <div className="mb-8 pt-8 text-center">
        <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Your Subscription</h1>
        <p className="text-gray-500 text-sm">Manage your VIP Membership</p>
      </div>

      <div className="bg-white p-6 rounded-3xl border border-gray-100 shadow-sm mb-6" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
        <div className="flex justify-between items-center mb-6">
          <span className="font-bold text-gray-900">Status</span>
          <span className={`px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider ${status === 'active' ? 'bg-green-100 text-green-700' : status === 'paused' ? 'bg-yellow-100 text-yellow-700' : 'bg-red-100 text-red-700'}`}>
            {status}
          </span>
        </div>

        <div className="flex justify-between items-center mb-6">
          <span className="text-gray-600 text-sm">Next Billing Date</span>
          <span className="font-medium text-gray-900">Jul 1, 2024</span>
        </div>

        <div className="flex justify-between items-center">
          <span className="text-gray-600 text-sm">Amount</span>
          <span className="font-medium text-gray-900">$49.00</span>
        </div>
      </div>

      <div className="space-y-3 mt-auto mb-8">
        {status === 'active' ? (
          <button onClick={() => handleAction('pause')} id="btn-pause-sub" className="w-full py-4 bg-yellow-50 text-yellow-700 border border-yellow-200 rounded-xl font-bold shadow-sm active:scale-95 transition-all">
            Pause Subscription
          </button>
        ) : status === 'paused' ? (
          <button onClick={() => handleAction('resume')} id="btn-resume-sub" className="w-full py-4 bg-green-50 text-green-700 border border-green-200 rounded-xl font-bold shadow-sm active:scale-95 transition-all">
            Resume Subscription
          </button>
        ) : null}

        {status !== 'canceled' && (
          <button onClick={() => handleAction('cancel')} id="btn-cancel-sub" className="w-full py-4 bg-red-50 text-red-700 border border-red-200 rounded-xl font-bold shadow-sm active:scale-95 transition-all">
            Cancel Subscription
          </button>
        )}
      </div>
    </div>
  );
}
