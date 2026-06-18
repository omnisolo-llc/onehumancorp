'use client';

import React, { useState, useEffect, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';

function SubscriptionManageContent() {
  const searchParams = useSearchParams();
  const token = searchParams.get('token');
  const action = searchParams.get('action');

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState(false);

  if (!token || !action) {
    return (
      <div className="flex flex-col items-center justify-center p-8 bg-white min-h-screen font-inter">
        <h2 className="text-xl font-bold text-gray-900 mb-4">Invalid Link</h2>
        <p className="text-gray-600 text-center">This subscription management link is missing required information.</p>
      </div>
    );
  }

  const actionTextMap: Record<string, string> = {
    'pause': 'Pause Subscription',
    'resume': 'Resume Subscription',
    'cancel': 'Cancel Subscription'
  };

  const actionDescriptionMap: Record<string, string> = {
    'pause': 'Are you sure you want to pause your subscription? You will not be billed or receive products until you resume.',
    'resume': 'Are you sure you want to resume your subscription? Your regular billing and deliveries will restart.',
    'cancel': 'Are you sure you want to cancel your subscription? This action is permanent.'
  };

  const displayAction = actionTextMap[action.toLowerCase()] || 'Update Subscription';
  const description = actionDescriptionMap[action.toLowerCase()] || `Are you sure you want to ${action} your subscription?`;

  const handleConfirm = async () => {
    setLoading(true);
    setError('');

    try {
      const response = await fetch('/api/subscription/magic-link', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ token, action })
      });

      if (!response.ok) {
        throw new Error('Failed to update subscription. The link may have expired.');
      }

      const data = await response.json();
      if (data.success) {
        setSuccess(true);
      } else {
        throw new Error('Update was not successful.');
      }
    } catch (err: any) {
      setError(err.message || 'An error occurred.');
    } finally {
      setLoading(false);
    }
  };

  if (success) {
    return (
      <div className="flex flex-col items-center justify-center p-8 min-h-screen bg-gray-50 font-inter">
        <div className="app-card w-full max-w-md rounded-2xl p-6 shadow-xl border border-green-100 bg-white">
          <div className="flex justify-center mb-4">
            <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-3xl shadow-inner text-green-600">
              ✅
            </div>
          </div>
          <h2 className="text-2xl font-bold font-outfit text-center text-gray-900 mb-2">Success!</h2>
          <p className="text-center text-gray-600">Your subscription has been updated successfully.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center p-4 min-h-screen bg-gray-50 font-inter">
      <div className="app-card w-full max-w-md rounded-2xl p-8 shadow-xl border border-gray-200 bg-white relative overflow-hidden">
        {/* Background embellishment */}
        <div className="absolute top-0 left-0 w-32 h-32 bg-indigo-50 rounded-br-full -z-10"></div>

        <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-4">{displayAction}</h2>
        <p className="text-gray-600 mb-8 leading-relaxed">
          {description}
        </p>

        {error && (
          <div className="mb-6 p-4 bg-red-50 border border-red-100 rounded-xl text-red-700 text-sm font-medium">
            {error}
          </div>
        )}

        <button
          onClick={handleConfirm}
          disabled={loading}
          className="w-full py-4 bg-indigo-600 text-white rounded-xl font-bold shadow-md hover:bg-indigo-700 transition-colors disabled:bg-indigo-400 disabled:cursor-not-allowed flex items-center justify-center text-lg"
        >
          {loading ? 'Processing...' : `Confirm ${action}`}
        </button>
      </div>
    </div>
  );
}

export default function SubscriptionManagePage() {
  return (
    <Suspense fallback={<div className="min-h-screen flex items-center justify-center font-inter text-gray-500">Loading...</div>}>
      <SubscriptionManageContent />
    </Suspense>
  );
}
