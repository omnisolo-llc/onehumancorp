"use client";

import React, { useState, useEffect } from 'react';
import { FiCopy as Copy, FiCheck as Check } from 'react-icons/fi';
import { motion } from 'framer-motion';

export function ReferAFriendDashboardWidget() {
  const [tenant, setTenant] = useState('my-business');
  const [copied, setCopied] = useState(false);
  const [isClient, setIsClient] = useState(false);

  const [rewardAmount, setRewardAmount] = useState('');
  const [referrerReward, setReferrerReward] = useState('');
  const [referralCode, setReferralCode] = useState('');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setIsClient(true);
    let currentTenant = 'my-business';
    if (typeof localStorage !== 'undefined') {
      currentTenant = localStorage.getItem('tenant') || 'my-business';
      setTenant(currentTenant);
    }

    // Fetch data from real backend endpoint
    fetch(`/api/v1/growth/refer-a-friend?tenant=${currentTenant}`)
      .then(res => res.json())
      .then(data => {
        setRewardAmount(data.rewardAmount);
        setReferrerReward(data.referrerReward);
        setReferralCode(data.referralCode);
        setLoading(false);
      })
      .catch(err => {
        console.error('Failed to load referral data', err);
        setLoading(false);
      });
  }, []);

  const referralLink = `https://ohc.app/ref/${referralCode}?tenant=${tenant}`;
  const defaultMessage = `Use my referral link to get ${rewardAmount} on your first order!`;

  const trackEvent = async (action: string) => {
    try {
      await fetch('/api/v1/growth/referrals/click', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          code: referralCode,
          tenant_id: tenant,
          action: action
        })
      });
    } catch (error) {
      console.error('Failed to track event:', error);
    }
  };

  const handleCopy = () => {
    if (typeof navigator !== 'undefined' && navigator.clipboard) {
      navigator.clipboard.writeText(referralLink);
      setCopied(true);
      trackEvent('copy_link');
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleShareTwitter = () => {
    const text = encodeURIComponent(defaultMessage);
    const url = encodeURIComponent(referralLink);
    window.open(`https://twitter.com/intent/tweet?text=${text}&url=${url}`, '_blank');
    trackEvent('share_twitter');
  };

  const handleShareWhatsApp = () => {
    const text = encodeURIComponent(`${defaultMessage} ${referralLink}`);
    window.open(`https://wa.me/?text=${text}`, '_blank');
    trackEvent('share_whatsapp');
  };

  if (!isClient) return null;
  if (loading) return (
    <div className="p-5 mb-6 text-left relative overflow-hidden flex flex-col items-center justify-center animate-pulse"
      style={{
        borderRadius: '16px',
        background: 'rgba(255, 255, 255, 0.65)',
        backdropFilter: 'blur(40px) saturate(220%)',
        WebkitBackdropFilter: 'blur(40px) saturate(220%)',
        border: '1px solid rgba(255, 255, 255, 0.4)',
        minHeight: '150px'
      }}>
      <div className="text-[#86868b]">Loading your referral data...</div>
    </div>
  );
  if (!referralCode) return null;

  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      className="p-5 mb-6 text-left relative overflow-hidden flex flex-col"
      style={{
        borderRadius: '16px',
        background: 'rgba(255, 255, 255, 0.65)',
        backdropFilter: 'blur(40px) saturate(220%)',
        WebkitBackdropFilter: 'blur(40px) saturate(220%)',
        border: '1px solid rgba(255, 255, 255, 0.4)',
        boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)'
      }}
    >
      <div className="flex items-center gap-3 mb-3">
        <div className="text-3xl">🤝</div>
        <div>
          <h2 className="font-['Outfit'] font-bold text-xl m-0 text-[#1d1d1f] dark:text-[#f5f5f7]">
            Refer a Friend
          </h2>
          <p className="text-[#86868b] text-sm m-0">
            Give <strong className="font-bold text-[#1d1d1f] dark:text-[#f5f5f7]">{rewardAmount}</strong>,
            Get <strong className="font-bold text-[#1d1d1f] dark:text-[#f5f5f7]">{referrerReward}</strong>
          </p>
        </div>
      </div>

      <div className="flex items-center bg-[rgba(0,0,0,0.05)] dark:bg-[rgba(255,255,255,0.05)] rounded-lg p-2.5 mb-4 border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
        <div className="flex-1 font-mono text-xs text-[#1d1d1f] dark:text-[#f5f5f7] whitespace-nowrap overflow-hidden text-ellipsis text-left" id="referral-link">
          {referralLink}
        </div>
        <button
          onClick={handleCopy}
          className="bg-[#0066FF] hover:bg-[#0052cc] active:scale-95 text-white border-none py-1.5 px-3 rounded-md font-semibold cursor-pointer ml-2 transition-all duration-200 text-xs flex items-center gap-1"
          id="copy-btn"
        >
          {copied ? <><Check size={14}/> Copied!</> : <><Copy size={14}/> Copy</>}
        </button>
      </div>

      <div className="flex gap-2">
        <button
          onClick={handleShareTwitter}
          className="flex-1 flex items-center justify-center p-2 rounded-lg font-semibold text-sm cursor-pointer border-none text-white bg-black hover:opacity-90 active:scale-[0.98] transition-all duration-200"
          id="share-twitter"
        >
          Share on X
        </button>
        <button
          onClick={handleShareWhatsApp}
          className="flex-1 flex items-center justify-center p-2 rounded-lg font-semibold text-sm cursor-pointer border-none text-white bg-[#25D366] hover:opacity-90 active:scale-[0.98] transition-all duration-200"
          id="share-whatsapp"
        >
          WhatsApp
        </button>
      </div>
    </motion.div>
  );
}
