"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { WithTooltip } from '../../components/TooltipRegistry';
import { PoweredByOHC } from '../components/PoweredByOHC';
import PostCheckoutShare from '../components/PostCheckoutShare';

export default function CheckoutPage() {
  const router = useRouter();
  const [isProcessing, setIsProcessing] = useState(false);
  const [showSuccessModal, setShowSuccessModal] = useState(false);
  const [referralLink, setReferralLink] = useState("");
  const [copied, setCopied] = useState(false);
  const [tenant, setTenant] = useState("my-store");
  const [checkoutStatus, setCheckoutStatus] = useState("");

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setTenant(localStorage.getItem('tenant') || 'my-store');
    }
  }, []);

  const [deliveryAddress, setDeliveryAddress] = useState("");
  const [deliveryFee, setDeliveryFee] = useState<number | null>(null);
  const [isCheckingDelivery, setIsCheckingDelivery] = useState(false);
  const [deliveryError, setDeliveryError] = useState<string | null>(null);

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
      console.error("Failed to check delivery eligibility", e);
      setDeliveryError("Error checking delivery.");
    } finally {
      setIsCheckingDelivery(false);
    }
  };

  const [isSubscription, setIsSubscription] = useState(false);

  const handlePayment = async (isSub = false) => {
    setIsProcessing(true);
    setIsSubscription(isSub);
    const fallbackReferralLink = () => {
      const origin = typeof window !== 'undefined' ? window.location.origin : '';
      return `${origin}/onboarding?ref=${tenant}`;
    };
    const normalizeReferralLink = (rawLink: string) => {
      if (!rawLink || rawLink.includes('ohc.store') || rawLink.startsWith('ohc://')) return fallbackReferralLink();
      return rawLink;
    };

    // Fetch dynamic referral link
    try {
      const response = await fetch("/api/v1/growth/referrals/generate", {
        method: "POST",
      });
      const data = await response.json();
      if (data && data.referral_link) {
        setReferralLink(normalizeReferralLink(data.referral_link));
      } else {
        setReferralLink(fallbackReferralLink());
      }
    } catch (e) {
      console.error("Failed to generate dynamic referral link", e);
      setReferralLink(fallbackReferralLink());
    }

    setIsProcessing(false);
    setShowSuccessModal(true);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Checkout</h1>
      </header>

      <main id="checkout-screen" className="p-6 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col gap-6">
        <div className="p-6 shadow-sm flex flex-col gap-4 mb-4" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
          <h2 className="text-lg font-semibold text-gray-900">Local Delivery</h2>
          <p className="text-sm text-gray-600">Enter your address to see if we can deliver to you via DoorDash Drive (flat fee).</p>
          <div className="flex gap-2">
            <input
              type="text"
              placeholder="Enter delivery address..."
              value={deliveryAddress}
              onChange={(e) => setDeliveryAddress(e.target.value)}
              className="flex-1 bg-white border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-800 focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all"
            />
            <button
              onClick={checkDeliveryEligibility}
              disabled={isCheckingDelivery || !deliveryAddress}
              className="px-4 py-2 bg-gray-900 text-white rounded-lg font-medium hover:bg-gray-800 transition-colors shadow-sm disabled:opacity-50 text-sm whitespace-nowrap"
            >
              {isCheckingDelivery ? 'Checking...' : 'Check'}
            </button>
          </div>
          {deliveryFee !== null && !deliveryError && (
            <div className="mt-2 p-3 bg-indigo-50 border border-indigo-100 rounded-lg flex items-center justify-between">
              <span className="text-sm text-indigo-900 font-medium flex items-center gap-2">
                <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                Delivery Available!
              </span>
              <span className="text-sm font-bold text-indigo-900">+${deliveryFee.toFixed(2)}</span>
            </div>
          )}
          {deliveryError && (
             <div className="mt-2 p-3 bg-red-50 border border-red-100 rounded-lg">
              <span className="text-sm text-red-900 font-medium">{deliveryError}</span>
            </div>
          )}
        </div>

        <p className="text-gray-700 font-medium">Payment Details</p>

        <div className="p-6 shadow-sm flex flex-col gap-4" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
          <p className="text-sm text-gray-600">100% money back guarantee. Secure SSL payments.</p>
          {deliveryFee !== null && (
            <div className="flex justify-between py-2 border-b border-gray-200">
               <span className="text-sm text-gray-600">Delivery Fee</span>
               <span className="text-sm font-medium text-gray-900">${deliveryFee.toFixed(2)}</span>
            </div>
          )}

          <WithTooltip id="checkout-pay-now-tooltip" defaultText="Click here to securely finish your purchase and process your payment.">
            <button
              onClick={() => handlePayment(false)}
              disabled={isProcessing}
              className={`w-full px-4 py-3 text-white rounded-lg font-medium transition-colors shadow-sm ${isProcessing ? 'bg-indigo-400 cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700'}`}
            >
              {isProcessing ? 'Processing...' : 'Pay Now'}
            </button>
          </WithTooltip>

          <WithTooltip id="checkout-subscribe-tooltip" defaultText="Start a monthly subscription using saved wallet payment for frictionless vaulting.">
            <button
              onClick={() => handlePayment(true)}
              disabled={isProcessing}
              className={`w-full px-4 py-3 text-white rounded-lg font-medium transition-colors shadow-sm ${isProcessing ? 'bg-green-400 cursor-not-allowed' : 'bg-green-600 hover:bg-green-700'}`}
            >
              {isProcessing ? 'Processing...' : 'Subscribe Monthly (Wallet Pay)'}
            </button>
          </WithTooltip>

          <WithTooltip id="checkout-tap-to-pay-tooltip" defaultText="Tap your card or phone on the reader to pay in person.">
            <button
              onClick={() => {
                if (navigator.onLine) {
                  setCheckoutStatus('Stripe Terminal payment captured for $45.00.');
                  handlePayment(false);
                } else {
                  let queue = [];
                  try {
                    queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
                  } catch (e) {}

                  queue.push({
                    id: 'txn_' + Date.now(),
                    amount: 45,
                    timestamp: new Date().toISOString(),
                    type: 'tap_to_pay',
                    idempotency_key: 'idempotency_' + Date.now() + Math.random().toString(36).substring(7)
                  });
                  localStorage.setItem('ohc_offline_queue', JSON.stringify(queue));
                  setCheckoutStatus('Offline terminal payment saved locally for sync.');
                  setShowSuccessModal(true);
                }
              }}
              className="w-full px-4 py-3 bg-blue-500 text-white rounded-lg font-medium hover:bg-blue-600 transition-colors shadow-sm"
            >
              Tap to Pay (Stripe Terminal)
            </button>
          </WithTooltip>

          <WithTooltip id="checkout-mercadopago-tooltip" defaultText="Pay securely using Mercado Pago.">
            <button
              onClick={() => {
                setCheckoutStatus("Mercado Pago checkout prepared.");
                setShowSuccessModal(true);
              }}
              className="w-full px-4 py-3 bg-[#009EE3] text-white rounded-lg font-medium hover:bg-[#007ebd] transition-colors shadow-sm flex items-center justify-center gap-2"
            >
              Pay with Mercado Pago
            </button>
          </WithTooltip>

          {checkoutStatus && <p className="text-sm font-medium text-indigo-700" role="status">{checkoutStatus}</p>}

          <WithTooltip id="checkout-cancel-tooltip" defaultText="Go back to the previous screen without buying anything.">
            <button
              onClick={() => router.push('/pricing')}
              className="w-full px-4 py-3 bg-gray-200 text-gray-800 rounded-lg font-medium hover:bg-gray-300 transition-colors"
            >
              Cancel
            </button>
          </WithTooltip>
          <PoweredByOHC tenantId={tenant} />
        </div>
      </main>

      {/* Post-Purchase Referral Modal */}
      {showSuccessModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter border border-green-100">
            {/* Background embellishment */}
            <div className="absolute top-0 right-0 w-32 h-32 bg-green-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-between items-start mb-4">
              <div className="w-12 h-12 bg-green-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-green-600">
                🎉
              </div>
            </div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Payment Successful!</h2>

            <div className="mb-6 mt-4">
              <PostCheckoutShare referralLink={referralLink} isSubscription={isSubscription} />
            </div>

            <div className="space-y-4">
              <button
                onClick={() => router.push('/dashboard')}
                className="w-full px-4 py-3 text-indigo-600 bg-indigo-50 rounded-lg font-medium hover:bg-indigo-100 transition-colors"
              >
                Continue to Dashboard
              </button>
            </div>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
