"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { WithTooltip } from '../../components/TooltipRegistry';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function CheckoutPage() {
  const router = useRouter();
  const [isProcessing, setIsProcessing] = useState(false);
  const [showSuccessModal, setShowSuccessModal] = useState(false);
  const [referralLink, setReferralLink] = useState("");
  const [copied, setCopied] = useState(false);
  const [tenant, setTenant] = useState("my-store");
  const [checkoutStatus, setCheckoutStatus] = useState("");
  const [isOffline, setIsOffline] = useState(false);
  const [isSubscription, setIsSubscription] = useState(false);

  // Delivery state
  const [deliveryAddress, setDeliveryAddress] = useState("");
  const [deliveryFee, setDeliveryFee] = useState<number | null>(null);
  const [isCheckingDelivery, setIsCheckingDelivery] = useState(false);
  const [deliveryError, setDeliveryError] = useState<string | null>(null);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setIsOffline(!navigator.onLine);
      const handleOnline = () => setIsOffline(false);
      const handleOffline = () => setIsOffline(true);
      window.addEventListener('online', handleOnline);
      window.addEventListener('offline', handleOffline);
      return () => {
        window.removeEventListener('online', handleOnline);
        window.removeEventListener('offline', handleOffline);
      };
    }
  }, []);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setTenant(localStorage.getItem('tenant') || 'my-store');
    }
  }, []);

  const checkDeliveryEligibility = async () => {
    if (!deliveryAddress) return;
    setIsCheckingDelivery(true);
    setDeliveryError(null);
    try {
      const response = await fetch("/api/checkout/delivery-quote", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ deliveryAddress }),
      });
      const data = await response.json();
      if (data.success) {
        setDeliveryFee(data.fee);
      } else {
        setDeliveryFee(null);
        setDeliveryError(data.message || "Delivery is not available.");
      }
    } catch (e) {
      setDeliveryError("Error checking delivery.");
    } finally {
      setIsCheckingDelivery(false);
    }
  };

  const handlePayment = async (isSub = false) => {
    setIsProcessing(true);
    setIsSubscription(isSub);
    const fallbackLink = `${typeof window !== 'undefined' ? window.location.origin : ''}/onboarding?ref=${tenant}`;
    try {
      const response = await fetch("/api/v1/growth/referrals/generate", { method: "POST" });
      const data = await response.json();
      setReferralLink(data?.referral_link || fallbackLink);
    } catch (e) {
      setReferralLink(fallbackLink);
    }
    setIsProcessing(false);
    setShowSuccessModal(true);
  };

  // SUCCESS VIEW
  if (showSuccessModal) {
    return (
      <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
        <header className="px-6 py-4 flex items-center justify-between border-b bg-white/80 backdrop-blur-md">
          <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F' }}>Order Recorded</h1>
          {isOffline && <span className="bg-orange-100 text-orange-700 text-[10px] uppercase font-bold px-2 py-0.5 rounded-full">Offline</span>}
        </header>

        <main className="flex-1 flex items-center justify-center p-4 md:p-8">
          <div className="bg-white w-full max-w-md rounded-2xl p-6 md:p-8 shadow-2xl relative overflow-hidden border border-green-100">
            <div className="absolute top-0 right-0 w-32 h-32 bg-green-50 rounded-bl-full -z-10"></div>
            <div className="w-12 h-12 bg-green-100 rounded-xl flex items-center justify-center text-2xl mb-6 shadow-inner text-green-600">🎉</div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">
              {isOffline ? "Payment Saved Offline" : "Payment Successful!"}
            </h2>
            <p className="text-gray-600 mb-8 text-sm leading-relaxed">
              {isOffline
                ? "Your payment was saved securely on this device. We'll automatically sync it as soon as you're back online!"
                : "Your order is confirmed! Love what you bought? Share with your friends to earn rewards."}
            </p>

            <div className="space-y-6">
              <div>
                <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">Referral Link</label>
                <div className="flex gap-2">
                  <input type="text" readOnly value={referralLink} className="flex-1 bg-gray-50 border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-600 focus:outline-none" />
                  <button onClick={() => { navigator.clipboard.writeText(referralLink); setCopied(true); setTimeout(() => setCopied(false), 2000); }} className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}>
                    {copied ? 'Copied!' : 'Copy'}
                  </button>
                </div>
              </div>

              <div className="flex flex-col gap-3">
                <a href={`https://wa.me/?text=${encodeURIComponent(`Check this out! ${referralLink}`)}`} target="_blank" rel="noopener noreferrer" className="flex items-center justify-center gap-2 bg-[#25D366] text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all">
                  <svg width="20" height="20" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                  WhatsApp
                </a>
                <button onClick={() => router.push('/dashboard')} className="w-full px-4 py-3 text-indigo-600 bg-indigo-50 rounded-lg font-semibold hover:bg-indigo-100 transition-colors mt-4">
                  Continue to Dashboard
                </button>
              </div>
            </div>
          </div>
        </main>
        <PoweredByOHC tenantId={tenant} />
      </div>
    );
  }

  // CHECKOUT FORM VIEW
  return (
    <div className="flex flex-col min-h-screen font-inter relative" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b bg-white/65 backdrop-blur-xl sticky top-0 z-40">
        <div className="flex items-center gap-3">
          <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F' }}>Checkout</h1>
          {isOffline && <span className="bg-orange-100 text-orange-700 text-[10px] uppercase font-bold px-2 py-0.5 rounded-full animate-pulse border border-orange-200">Offline</span>}
        </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col gap-6">
        {/* Delivery Card */}
        <div className="p-6 shadow-sm flex flex-col gap-4 bg-white/65 backdrop-blur-xl border border-white/40 rounded-2xl">
          <h2 className="text-lg font-semibold text-gray-900">Local Delivery</h2>
          <div className="flex gap-2">
            <input type="text" placeholder="Delivery address..." value={deliveryAddress} onChange={(e) => setDeliveryAddress(e.target.value)} className="flex-1 border border-gray-200 rounded-lg px-3 py-2 text-sm focus:ring-2 focus:ring-indigo-500 outline-none" />
            <button onClick={checkDeliveryEligibility} disabled={isCheckingDelivery || !deliveryAddress} className="px-4 py-2 bg-gray-900 text-white rounded-lg font-medium text-sm disabled:opacity-50">
              {isCheckingDelivery ? 'Checking...' : 'Check'}
            </button>
          </div>
          {deliveryFee !== null && <div className="p-3 bg-indigo-50 border border-indigo-100 rounded-lg text-sm text-indigo-900 font-medium flex justify-between"><span>Delivery Available!</span><span>+${deliveryFee.toFixed(2)}</span></div>}
          {deliveryError && <div className="p-3 bg-red-50 border border-red-100 rounded-lg text-sm text-red-900 font-medium">{deliveryError}</div>}
        </div>

        {/* Payment Card */}
        <div className="p-6 shadow-sm flex flex-col gap-4 bg-white/65 backdrop-blur-xl border border-white/40 rounded-2xl">
          <p className="text-sm text-gray-600 mb-2">100% money back guarantee. Secure SSL payments.</p>

          <button onClick={() => handlePayment(false)} disabled={isProcessing} className="w-full px-4 py-3 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg font-semibold shadow-sm transition-all disabled:bg-indigo-400">
            {isProcessing ? 'Processing...' : 'Pay Now'}
          </button>

          <button
            onClick={() => {
              if (navigator.onLine) {
                setCheckoutStatus('Online payment captured.');
                handlePayment(false);
              } else {
                const { SyncManager } = require('../../lib/sync/SyncManager');
                SyncManager.getInstance().enqueue({
                  id: 'txn_' + Date.now(),
                  amount: 45,
                  timestamp: new Date().toISOString(),
                  type: 'tap_to_pay',
                  idempotency_key: 'id_' + Date.now(),
                  product_id: 'checkout_tap_to_pay',
                  quantity_deducted: 1
                });
                setCheckoutStatus('Offline payment saved.');
                setShowSuccessModal(true);
              }
            }}
            className="w-full px-4 py-3 min-h-[44px] bg-blue-600 text-white rounded-lg font-semibold hover:bg-blue-700 transition-all shadow-md flex items-center justify-center gap-2 active:scale-95"
          >
            <svg width="20" height="20" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2"><path d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" /></svg>
            Tap to Pay
          </button>

          {checkoutStatus && <p className="text-center text-sm font-medium text-indigo-700 mt-2">{checkoutStatus}</p>}
        </div>
      </main>
      <PoweredByOHC tenantId={tenant} />
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
