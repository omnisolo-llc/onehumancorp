"use client";

import React, { useState, useEffect } from 'react';

interface ReferralTier {
  current_tier: string;
  next_tier: string | null;
  referrals_needed_for_next: number | null;
  total_conversions: number;
}

interface ReferralStats {
  total_clicks: number;
  total_conversions: number;
  channels: Record<string, number>;
}

export default function ReferralsDashboardPage() {
  const [tierData, setTierData] = useState<ReferralTier | null>(null);
  const [statsData, setStatsData] = useState<ReferralStats | null>(null);
  const [referralLink, setReferralLink] = useState<string>('');
  const [tenant, setTenant] = useState('my-store');
  const [copied, setCopied] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const tid = typeof localStorage !== 'undefined' ? (localStorage.getItem('tenant') || 'my-store') : 'my-store';
    setTenant(tid);

    const fetchData = async () => {
      try {
        const [tierRes, statsRes] = await Promise.all([
          fetch(`/api/v1/growth/referrals/tier`),
          fetch(`/api/v1/growth/referrals/stats`)
        ]);

        if (tierRes.ok) {
            const tierJson = await tierRes.json();
            setTierData(tierJson);
        } else {
            // fallback
            setTierData({ current_tier: "Bronze", next_tier: "Silver", referrals_needed_for_next: 5, total_conversions: 0 });
        }

        if (statsRes.ok) {
            const statsJson = await statsRes.json();
            setStatsData(statsJson);
        } else {
            // fallback
            setStatsData({ total_clicks: 0, total_conversions: 0, channels: {} });
        }

      } catch (e) {
         setTierData({ current_tier: "Bronze", next_tier: "Silver", referrals_needed_for_next: 5, total_conversions: 0 });
         setStatsData({ total_clicks: 0, total_conversions: 0, channels: {} });
      } finally {
        setIsLoading(false);
      }
    };

    fetchData();
  }, []);

  const generateLink = async () => {
      try {
        const res = await fetch('/api/v1/growth/referrals/generate', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenantId: tenant, customMessage: 'Join me on OHC!' })
        });
        if (!res.ok) {
           throw new Error('Failed to fetch');
        }
        const data = await res.json();
        setReferralLink(data.referral_link);
      } catch (e) {
        console.error(e);
        setReferralLink(`http://localhost:3000/onboarding?ref=${tenant}`);
      }
  };

  useEffect(() => {
      if (tenant) {
          generateLink();
      }
  }, [tenant]);

  const handleCopy = () => {
      if (referralLink) {
          navigator.clipboard.writeText(referralLink);
          setCopied(true);
          setTimeout(() => setCopied(false), 2000);
      }
  };

  const calculateProgress = () => {
      if (!tierData) return 0;
      if (!tierData.next_tier) return 100; // Max tier

      const currentConversions = tierData.total_conversions;
      const target = currentConversions + (tierData.referrals_needed_for_next || 0);

      let lowerBound = 0;
      if (tierData.current_tier === 'Silver') lowerBound = 5;
      if (tierData.current_tier === 'Gold') lowerBound = 20;
      if (tierData.current_tier === 'Platinum') lowerBound = 50;

      const progress = ((currentConversions - lowerBound) / (target - lowerBound)) * 100;
      return Math.min(Math.max(progress, 0), 100);
  };

  if (isLoading) {
      return (
          <div className="min-h-screen bg-gray-50 flex items-center justify-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
          </div>
      );
  }

  return (
    <div className="min-h-screen bg-[#F8FAFC] text-gray-900 p-4 md:p-8 font-sans">
      <div className="max-w-4xl mx-auto space-y-6">

        {/* Header */}
        <div>
          <h1 className="text-3xl font-bold font-outfit text-gray-900 tracking-tight">Referrals & Rewards</h1>
          <p className="text-gray-500 mt-2">Invite other businesses and unlock premium tiers and rewards.</p>
        </div>

        {/* Tier Status Card */}
        <div className="bg-white/70 backdrop-blur-xl border border-gray-200/50 p-6 md:p-8 rounded-3xl shadow-sm">
            <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-6">
                <div>
                    <h2 className="text-sm font-semibold text-gray-500 uppercase tracking-wider mb-1">Your Tier</h2>
                    <div className="text-4xl font-bold text-gray-900 font-outfit flex items-center gap-3">
                        {tierData?.current_tier || 'Bronze'}
                        {tierData?.current_tier === 'Platinum' && <span className="text-2xl">🏆</span>}
                        {tierData?.current_tier === 'Gold' && <span className="text-2xl">🥇</span>}
                        {tierData?.current_tier === 'Silver' && <span className="text-2xl">🥈</span>}
                        {tierData?.current_tier === 'Bronze' && <span className="text-2xl">🥉</span>}
                    </div>
                </div>

                {tierData?.next_tier && (
                    <div className="flex-1 w-full max-w-md">
                        <div className="flex justify-between text-sm mb-2">
                            <span className="font-medium text-gray-700">{tierData.total_conversions} referrals</span>
                            <span className="text-gray-500">{tierData.referrals_needed_for_next} more for {tierData.next_tier}</span>
                        </div>
                        <div className="h-3 w-full bg-gray-100 rounded-full overflow-hidden">
                            <div
                                className="h-full bg-gradient-to-r from-blue-500 to-indigo-600 rounded-full transition-all duration-1000 ease-out"
                                style={{ width: `${calculateProgress()}%` }}
                            />
                        </div>
                    </div>
                )}
                {!tierData?.next_tier && !isLoading && (
                     <div className="text-green-600 font-medium bg-green-50 px-4 py-2 rounded-full border border-green-100">
                         Max Tier Reached! 🎉
                     </div>
                )}
            </div>
        </div>

        {/* How it works */}
        <div className="bg-white/70 backdrop-blur-xl border border-gray-200/50 p-6 md:p-8 rounded-3xl shadow-sm">
           <h3 className="text-xl font-bold font-outfit text-gray-900 mb-6">How it works</h3>
           <div className="grid grid-cols-1 sm:grid-cols-3 gap-6">
               <div className="flex items-start gap-4">
                   <div className="w-8 h-8 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center font-bold">1</div>
                   <div>
                       <h4 className="font-semibold text-gray-900">Share Link</h4>
                       <p className="text-sm text-gray-500 mt-1">Send your unique link</p>
                   </div>
               </div>
               <div className="flex items-start gap-4">
                   <div className="w-8 h-8 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center font-bold">2</div>
                   <div>
                       <h4 className="font-semibold text-gray-900">They Sign Up</h4>
                       <p className="text-sm text-gray-500 mt-1">Your friend creates an account</p>
                   </div>
               </div>
               <div className="flex items-start gap-4">
                   <div className="w-8 h-8 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center font-bold">3</div>
                   <div>
                       <h4 className="font-semibold text-gray-900">You Get $50</h4>
                       <p className="text-sm text-gray-500 mt-1">Earn credit for premium features</p>
                   </div>
               </div>
           </div>
        </div>

        {/* Action & Stats Grid */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">

            {/* Share Link Card */}
            <div className="md:col-span-2 bg-white/70 backdrop-blur-xl border border-gray-200/50 p-6 rounded-3xl shadow-sm flex flex-col justify-center">
                <h3 className="text-lg font-bold text-gray-900 font-outfit mb-4">Share Your Link</h3>
                <p className="text-sm text-gray-600 mb-6">Give friends $50 in OHC credits, and get a free month of Pro when they subscribe.</p>

                <div className="flex items-center gap-3 bg-gray-50/80 p-2 pl-4 rounded-2xl border border-gray-200">
                    <span id="referral-link" className="flex-1 text-gray-700 font-mono text-sm truncate">
                        {referralLink || 'Generating your unique link...'}
                    </span>
                    <button
                        onClick={handleCopy}
                        disabled={!referralLink}
                        className={`px-6 py-3 rounded-xl text-sm font-semibold transition-all ${
                            copied ? 'bg-green-500 text-white' : 'bg-blue-600 text-white hover:bg-blue-700'
                        } disabled:opacity-50 disabled:cursor-not-allowed`}
                    >
                        {copied ? 'Copied!' : 'Copy Link'}
                    </button>
                </div>
            </div>

            {/* Quick Stats */}
            <div className="bg-white/70 backdrop-blur-xl border border-gray-200/50 p-6 rounded-3xl shadow-sm flex flex-col gap-4">
                <h3 className="text-lg font-bold text-gray-900 font-outfit">Performance</h3>

                <div className="bg-blue-50/50 border border-blue-100 p-4 rounded-2xl">
                    <div className="text-sm text-gray-500 mb-1">Total Clicks</div>
                    <div className="text-2xl font-bold text-blue-700 font-outfit">{statsData?.total_clicks || 0}</div>
                </div>

                <div className="bg-indigo-50/50 border border-indigo-100 p-4 rounded-2xl">
                    <div className="text-sm text-gray-500 mb-1">Conversions</div>
                    <div className="text-2xl font-bold text-indigo-700 font-outfit">{statsData?.total_conversions || 0}</div>
                </div>
            </div>

        </div>

      </div>

      <footer className="mt-8 text-center pb-8">
        <a href="/onboarding" className="text-sm font-semibold text-gray-500 hover:text-gray-900 transition-colors">⚡ Powered by OHC</a>
      </footer>
    </div>
  );
}
