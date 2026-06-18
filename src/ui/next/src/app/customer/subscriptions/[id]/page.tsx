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
    // Simulate fetching subscription details
    setTimeout(() => {
      setSubscription({
        id: subscriptionId,
        productName: 'Artisan Coffee Blend',
        frequency: 'Monthly',
        status: 'active',
        nextDeliveryDate: '2023-11-15',
        price: 24.00,
        discountedPrice: 21.60,
      });
      setLoading(false);
    }, 500);
  }, [subscriptionId]);

  const handleAction = (action: 'pause' | 'skip' | 'cancel') => {
    setIsProcessing(true);
    setActionStatus(null);

    // Simulate API call to handle action
    setTimeout(() => {
      let updatedStatus = subscription.status;
      let nextDate = subscription.nextDeliveryDate;
      let message = '';

      if (action === 'pause') {
        updatedStatus = 'paused';
        message = 'Your subscription has been paused.';
      } else if (action === 'skip') {
        // Simple mock of adding 1 month
        nextDate = '2023-12-15';
        message = 'Your next delivery has been skipped.';
      } else if (action === 'cancel') {
        updatedStatus = 'cancelled';
        message = 'Your subscription has been cancelled.';
      }

      setSubscription({
        ...subscription,
        status: updatedStatus,
        nextDeliveryDate: nextDate,
      });
      setIsProcessing(false);
      setActionStatus({ message, type: 'success' });
    }, 1000);
  };

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50 font-inter">
        <p className="text-gray-500 font-medium">Loading your subscription...</p>
      </div>
    );
  }

  if (!subscription) {
     return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50 font-inter">
        <p className="text-red-500 font-medium">Subscription not found.</p>
      </div>
    );
  }

  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'active':
        return <span className="px-2.5 py-1 text-xs font-bold rounded-full bg-green-100 text-green-800 border border-green-200">Active</span>;
      case 'paused':
        return <span className="px-2.5 py-1 text-xs font-bold rounded-full bg-orange-100 text-orange-800 border border-orange-200">Paused</span>;
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
              <h2 className="text-xl font-bold font-outfit text-gray-900 mb-1">{subscription.productName}</h2>
              <p className="text-sm text-gray-500 font-medium">Subscription ID: <span className="text-gray-400 font-mono text-xs">{subscriptionId}</span></p>
            </div>
            {getStatusBadge(subscription.status)}
          </div>

          <div className="space-y-4 mb-8">
            <div className="flex justify-between items-center py-2 border-b border-gray-100/50">
              <span className="text-sm font-semibold text-gray-500 uppercase tracking-wider">Next Delivery</span>
              <span className="font-bold text-gray-900">{subscription.status === 'cancelled' ? '-' : subscription.nextDeliveryDate}</span>
            </div>
            <div className="flex justify-between items-center py-2 border-b border-gray-100/50">
              <span className="text-sm font-semibold text-gray-500 uppercase tracking-wider">Frequency</span>
              <span className="font-bold text-gray-900">{subscription.frequency}</span>
            </div>
            <div className="flex justify-between items-center py-2">
              <span className="text-sm font-semibold text-gray-500 uppercase tracking-wider">Price</span>
              <div className="text-right">
                <span className="font-bold text-gray-900 text-lg">${subscription.discountedPrice.toFixed(2)}</span>
                <span className="block text-xs text-gray-400 line-through">${subscription.price.toFixed(2)}</span>
              </div>
            </div>
          </div>

          {subscription.status !== 'cancelled' && (
            <div className="space-y-3 pt-2">
               <button
                  onClick={() => handleAction('skip')}
                  disabled={isProcessing || subscription.status === 'paused'}
                  className="w-full py-3.5 px-4 bg-white border border-gray-200 text-gray-900 font-semibold rounded-xl shadow-sm hover:bg-gray-50 hover:border-gray-300 disabled:opacity-50 disabled:cursor-not-allowed transition-all active:scale-[0.98]"
                >
                  Skip Next Delivery
                </button>

                {subscription.status === 'active' ? (
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

          {subscription.status === 'cancelled' && (
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
