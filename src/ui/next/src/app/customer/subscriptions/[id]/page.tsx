'use client';

import React, { useState, useEffect } from 'react';
import { useParams } from 'next/navigation';
import { PoweredByOHC } from '../../../components/PoweredByOHC';

export default function CustomerSubscriptionPortal() {
  const params = useParams();
  const subscriptionId = params?.id as string;
  const [subscription, setSubscription] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [actionStatus, setActionStatus] = useState<{ message: string; type: 'success' | 'error' } | null>(null);
  const [isProcessing, setIsProcessing] = useState(false);

  useEffect(() => {
    fetch(`/api/subscriptions/${subscriptionId}`)
      .then(res => res.json())
      .then(data => {
        setSubscription(data);
        setLoading(false);
      })
      .catch(err => {
        console.error('Failed to fetch subscription details', err);
        setLoading(false);
      });
  }, [subscriptionId]);

  const handleAction = async (action: 'pause' | 'skip' | 'cancel') => {
    setIsProcessing(true);
    setActionStatus(null);

    try {
      const res = await fetch('/api/subscriptions/magic-link', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ token: subscriptionId, action }), // the endpoint seems to use the token to lookup the subscription
      });

      if (!res.ok) throw new Error('Failed to perform action');
      const resData = await res.json();

      let updatedStatus = subscription.status;
      let nextDate = subscription.next_delivery_date;
      let message = '';

      if (action === 'pause') {
        updatedStatus = 'Paused';
        message = 'Your subscription has been paused.';
      } else if (action === 'skip') {
        if (resData.next_delivery_date) {
            nextDate = resData.next_delivery_date;
        } else {
            // For simple optimistic update if the server doesn't return the new date.
            const parts = nextDate.split('-');
            if (parts.length >= 3) {
              let year = parseInt(parts[0], 10);
              let month = parseInt(parts[1], 10);
              const rest = parts.slice(2).join('-');
              month++;
              if (month > 12) {
                month = 1;
                year++;
              }
              nextDate = `${year}-${month.toString().padStart(2, '0')}-${rest}`;
            }
        }
        message = 'Your next delivery has been skipped.';
      } else if (action === 'cancel') {
        updatedStatus = 'Canceled';
        message = 'Your subscription has been cancelled.';
      }

      setSubscription({
        ...subscription,
        status: updatedStatus,
        next_delivery_date: nextDate,
      });
      setActionStatus({ message, type: 'success' });
    } catch (err) {
      console.error(err);
      setActionStatus({ message: 'Failed to perform action. Please try again.', type: 'error' });
    } finally {
      setIsProcessing(false);
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50 font-inter">
        <p className="text-gray-500 font-medium">Loading your subscription...</p>
      </div>
    );
  }

  if (!subscription || !subscription.id) {
     return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50 font-inter">
        <p className="text-red-500 font-medium">Subscription not found.</p>
      </div>
    );
  }

  const getStatusBadge = (status: string) => {
    const s = status.toLowerCase();
    switch (s) {
      case 'active':
        return <span className="px-2.5 py-1 text-xs font-bold rounded-full bg-green-100 text-green-800 border border-green-200">Active</span>;
      case 'paused':
        return <span className="px-2.5 py-1 text-xs font-bold rounded-full bg-orange-100 text-orange-800 border border-orange-200">Paused</span>;
      case 'canceled':
      case 'cancelled':
        return <span className="px-2.5 py-1 text-xs font-bold rounded-full bg-red-100 text-red-800 border border-red-200">Cancelled</span>;
      default:
        return null;
    }
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] font-inter p-4 pb-20 sm:p-6 sm:pb-24">
      <div className="max-w-[480px] mx-auto">
        <header className="mb-6 text-center">
          <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">Manage Subscription</h1>
        </header>

        {actionStatus && (
          <div className={`mb-6 p-4 rounded-xl border ${actionStatus.type === 'success' ? 'bg-green-50 border-green-200 text-green-800' : 'bg-red-50 border-red-200 text-red-800'} animate-fade-in-up`} role="status">
            <p className="text-sm font-semibold text-center">{actionStatus.message}</p>
          </div>
        )}

        <div className="relative overflow-hidden p-6 rounded-[24px] shadow-[0_8px_30px_rgb(0,0,0,0.04)] mb-6 transition-all duration-300"
             style={{
                background: 'rgba(255, 255, 255, 0.7)',
                backdropFilter: 'blur(40px) saturate(210%)',
                border: '1px solid rgba(255, 255, 255, 0.8)'
             }}>
          <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10 opacity-60"></div>

          <div className="flex justify-between items-start mb-6 border-b border-gray-100/50 pb-5">
            <div>
              <h2 className="text-xl font-bold font-outfit text-gray-900 mb-1">{subscription.product_name}</h2>
              <p className="text-sm text-gray-500 font-medium">Subscription ID: <span className="text-gray-400 font-mono text-xs">{subscriptionId}</span></p>
            </div>
            {getStatusBadge(subscription.status)}
          </div>

          <div className="space-y-4 mb-8">
            <div className="flex justify-between items-center py-2 border-b border-gray-100/50">
              <span className="text-sm font-semibold text-gray-500 uppercase tracking-wider">Next Delivery</span>
              <span className="font-bold text-gray-900">{subscription.status.toLowerCase() === 'canceled' || subscription.status.toLowerCase() === 'cancelled' ? '-' : subscription.next_delivery_date.split(' ')[0]}</span>
            </div>
            <div className="flex justify-between items-center py-2 border-b border-gray-100/50">
              <span className="text-sm font-semibold text-gray-500 uppercase tracking-wider">Frequency</span>
              <span className="font-bold text-gray-900">{subscription.frequency}</span>
            </div>
            <div className="flex justify-between items-center py-2">
              <span className="text-sm font-semibold text-gray-500 uppercase tracking-wider">Price</span>
              <div className="text-right">
                <span className="font-bold text-gray-900 text-lg">${subscription.discounted_price.toFixed(2)}</span>
                {subscription.price !== subscription.discounted_price && (
                    <span className="block text-xs text-gray-400 line-through">${subscription.price.toFixed(2)}</span>
                )}
              </div>
            </div>
          </div>

          {subscription.status.toLowerCase() !== 'canceled' && subscription.status.toLowerCase() !== 'cancelled' && (
            <div className="space-y-3 pt-2">
               <button
                  onClick={() => handleAction('skip')}
                  disabled={isProcessing || subscription.status.toLowerCase() === 'paused'}
                  className="w-full py-3.5 px-4 bg-white border border-gray-200 text-gray-900 font-semibold rounded-xl shadow-sm hover:bg-gray-50 hover:border-gray-300 disabled:opacity-50 disabled:cursor-not-allowed transition-all active:scale-[0.98]"
                >
                  Skip Next Delivery
                </button>

                {subscription.status.toLowerCase() === 'active' ? (
                  <button
                    onClick={() => handleAction('pause')}
                    disabled={isProcessing}
                    className="w-full py-3.5 px-4 bg-orange-50 border border-orange-200 text-orange-700 font-semibold rounded-xl shadow-sm hover:bg-orange-100 disabled:opacity-50 disabled:cursor-not-allowed transition-all active:scale-[0.98]"
                  >
                    Pause Subscription
                  </button>
                ) : (
                  <button
                    onClick={() => handleAction('pause')}
                    disabled={true} // In a real app, this would unpause
                    className="w-full py-3.5 px-4 bg-gray-100 border border-gray-200 text-gray-400 font-semibold rounded-xl cursor-not-allowed transition-all"
                  >
                    Subscription Paused
                  </button>
                )}

                <div className="pt-4 mt-2 border-t border-gray-100/50">
                  <button
                    onClick={() => handleAction('cancel')}
                    disabled={isProcessing}
                    className="w-full py-3 px-4 text-red-600 font-semibold hover:bg-red-50 rounded-xl transition-all text-sm"
                  >
                    Cancel Subscription
                  </button>
                </div>
            </div>
          )}

          {(subscription.status.toLowerCase() === 'canceled' || subscription.status.toLowerCase() === 'cancelled') && (
             <div className="pt-2">
                <p className="text-center text-sm text-gray-500 mb-4">You have cancelled this subscription.</p>
             </div>
          )}
        </div>

        <div className="mt-8 flex justify-center">
            <PoweredByOHC tenantId="demo" />
        </div>
      </div>
    </div>
  );
}
