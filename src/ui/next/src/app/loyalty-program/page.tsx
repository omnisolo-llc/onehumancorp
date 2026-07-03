"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function LoyaltyProgramPage() {
  const router = useRouter();
  const [pointsBalance, setPointsBalance] = useState(0);
  const [lifetimePoints, setLifetimePoints] = useState(0);
  const [isLoading, setIsLoading] = useState(true);
  const [isRedeeming, setIsRedeeming] = useState(false);
  const [rewardCode, setRewardCode] = useState('');

  useEffect(() => {
    // Fetch user points from backend API
    const fetchPoints = async () => {
      try {
        const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'default_tenant' : 'default_tenant';
        const customerId = typeof localStorage !== 'undefined' ? localStorage.getItem('customer_id') || 'default_customer' : 'default_customer';

        // Use the existing mock logic for E2E tests if API isn't wired fully, but attempt to call API
        const response = await fetch(`/api/v1/loyalty/accounts?tenant_id=${tenantId}&program_id=default&customer_id=${customerId}`);
        if (response.ok) {
          const data = await response.json();
          setPointsBalance(data.points_balance || 0);
          setLifetimePoints(data.lifetime_points || 0);
        } else {
            // For E2E fallback
            setPointsBalance(0);
        }
      } catch (err) {
        console.error("Failed to fetch points", err);
      } finally {
        setIsLoading(false);
      }
    };
    fetchPoints();
  }, []);

  const handleRedeem = async () => {
    setIsRedeeming(true);
    try {
        const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'default_tenant' : 'default_tenant';
        const customerId = typeof localStorage !== 'undefined' ? localStorage.getItem('customer_id') || 'default_customer' : 'default_customer';

        const response = await fetch('/api/v1/loyalty/transactions', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                tenant_id: tenantId,
                account_id: customerId, // In reality, fetch actual account_id
                transaction_type: 'redeem',
                amount: 100,
                reason: 'Customer Redeemed for Free Shipping'
            })
        });

        if (response.ok) {
            setPointsBalance(prev => prev - 100);
            setRewardCode('FREESHIP100');
        } else {
            alert("Failed to redeem points.");
        }
    } catch (e) {
        console.error(e);
        alert("An error occurred during redemption.");
    } finally {
        setIsRedeeming(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gray-50 items-center py-8">
        <main className="w-full max-w-[375px] px-4 flex flex-col gap-6">
            <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight text-center">My Wallet</h1>

            <div className="glassmorphism rounded-2xl p-6 shadow-lg border border-white/40 bg-gradient-to-br from-indigo-500 to-purple-600 text-white relative overflow-hidden">
                <div className="absolute top-0 right-0 -mr-8 -mt-8 w-32 h-32 rounded-full bg-white opacity-10 blur-2xl"></div>

                <p className="text-sm font-medium opacity-80 uppercase tracking-wider mb-2">Available Points</p>
                <h2 className="text-5xl font-bold mb-6">{isLoading ? '...' : pointsBalance}</h2>

                <div className="flex justify-between items-end">
                    <div>
                        <p className="text-xs opacity-75">Lifetime Earned</p>
                        <p className="text-sm font-semibold">{isLoading ? '...' : lifetimePoints} pts</p>
                    </div>
                </div>
            </div>

            <div className="glassmorphism rounded-2xl p-6 shadow-sm border border-gray-200 bg-white">
                <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2">Available Rewards</h3>

                {rewardCode ? (
                    <div className="p-4 bg-green-50 text-green-700 rounded-lg border border-green-200">
                        <p className="text-sm font-bold mb-1">Reward Unlocked!</p>
                        <p className="text-xs mb-2">Use this code at checkout:</p>
                        <code className="bg-white px-2 py-1 rounded text-lg font-mono border border-green-300 block text-center select-all">{rewardCode}</code>
                    </div>
                ) : (
                    <div className="flex items-center justify-between border border-gray-100 p-4 rounded-xl">
                        <div>
                            <p className="font-semibold text-gray-900 text-sm">Free Shipping</p>
                            <p className="text-xs text-gray-500">100 points</p>
                        </div>
                        <button
                            onClick={handleRedeem}
                            disabled={pointsBalance < 100 || isRedeeming}
                            className="px-4 py-2 bg-indigo-600 text-white text-sm font-medium rounded-full hover:bg-indigo-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {isRedeeming ? 'Redeeming...' : 'Redeem 100pts for Free Shipping'}
                        </button>
                    </div>
                )}
            </div>

            <div className="mt-8 text-center">
                <button onClick={() => router.push('/')} className="text-sm text-gray-500 hover:text-gray-900 underline">
                    Back to Store
                </button>
            </div>
        </main>
    </div>
  );
}
