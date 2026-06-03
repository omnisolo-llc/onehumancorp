"use client";

import React, { useState } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';
import { WithTooltip } from '../../components/TooltipRegistry';

export default function CheckoutPage() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const [isProcessing, setIsProcessing] = useState(false);
  const [showSuccessModal, setShowSuccessModal] = useState(false);
  const [referralLink, setReferralLink] = useState("");
  const [copied, setCopied] = useState(false);

  const [deliveryAddress, setDeliveryAddress] = useState("");
  const [deliveryFee, setDeliveryFee] = useState<number | null>(null);
  const [deliveryRadius] = useState(5); // Fixed for demo, would come from business settings
  const [isCheckingDelivery, setIsCheckingDelivery] = useState(false);

  const isSubscription = searchParams?.get('type') === 'subscription';
  const interval = searchParams?.get('interval') || 'Month';
  const productId = searchParams?.get('product') || 'e2e-product-cake';
  const priceStr = searchParams?.get('price') || '3999';
  const price = parseInt(priceStr, 10);

  const checkDeliveryEligibility = async () => {
    if (!deliveryAddress) return;
    setIsCheckingDelivery(true);
    // Simulate an API call that returns a dynamic delivery fee based on distance
    setTimeout(() => {
      setDeliveryFee(4.99); // Mock dynamic fee
      setIsCheckingDelivery(false);
    }, 800);
  };

  const handlePayment = async () => {
    setIsProcessing(true);

    if (isSubscription) {
      try {
        const response = await fetch("/api/v1/billing/subscriptions", {
          method: "POST",
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            tenant_id: 'e2e-tenant',
            customer_id: 'e2e-customer-ava',
            product_id: productId,
            interval: interval,
            price_cents: price
          })
        });
        const data = await response.json();
        if (data.success) {
           setShowSuccessModal(true);
        }
      } catch (e) {
        console.error("Subscription payment failed", e);
      }
    } else {
      // Fetch dynamic referral link
      try {
        const response = await fetch("/api/v1/growth/referrals/generate", {
          method: "POST",
        });
        const data = await response.json();
        if (data && data.referral_link) {
          setReferralLink(data.referral_link);
        } else {
          const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
          setReferralLink(`https://ohc.store/join?ref=${tenant}`);
        }
      } catch (e) {
        console.error("Failed to generate dynamic referral link", e);
        const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
        setReferralLink(`https://ohc.store/join?ref=${tenant}`);
      }
      setShowSuccessModal(true);
    }

    setIsProcessing(false);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Checkout</h1>
      </header>

      <main id="checkout-screen" className="p-6 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col gap-6">

        <div className="bg-white rounded-2xl p-4 shadow-sm border border-gray-100 flex items-center gap-4 mb-4">
           <div className="w-16 h-16 bg-gray-100 rounded-xl overflow-hidden flex items-center justify-center text-3xl">
              🧁
           </div>
           <div className="flex-1">
              <h2 className="font-bold text-gray-900">Vegan Celebration Cake</h2>
              {isSubscription ? (
                <p className="text-sm font-semibold text-gray-600">${(price / 100).toFixed(2)} / {interval}</p>
              ) : (
                <p className="text-sm text-gray-500">Qty: 1</p>
              )}
           </div>
           {!isSubscription && <div className="font-bold text-gray-900">$39.99</div>}
        </div>

        {!isSubscription && (
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
            {deliveryFee !== null && (
              <div className="mt-2 p-3 bg-indigo-50 border border-indigo-100 rounded-lg flex items-center justify-between">
                <span className="text-sm text-indigo-900 font-medium flex items-center gap-2">
                  <svg className="w-4 h-4 text-indigo-600" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                  Delivery Available!
                </span>
                <span className="text-sm font-bold text-indigo-900">+${deliveryFee.toFixed(2)}</span>
              </div>
            )}
          </div>
        )}

        <p className="text-gray-700 font-medium">Payment Details</p>

        <div className="p-6 shadow-sm flex flex-col gap-4" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
          <p className="text-sm text-gray-600">100% money back guarantee. Secure SSL payments.</p>
          {deliveryFee !== null && !isSubscription && (
            <div className="flex justify-between py-2 border-b border-gray-200">
               <span className="text-sm text-gray-600">Delivery Fee</span>
               <span className="text-sm font-medium text-gray-900">${deliveryFee.toFixed(2)}</span>
            </div>
          )}

          <WithTooltip id="checkout-pay-now-tooltip" defaultText="Click here to securely finish your purchase and process your payment.">
            <button
              onClick={handlePayment}
              disabled={isProcessing}
              className={`w-full px-4 py-3 text-white rounded-lg font-medium transition-colors shadow-sm ${isProcessing ? 'bg-indigo-400 cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700'}`}
            >
              {isProcessing ? 'Processing...' : 'Pay Now'}
            </button>
          </WithTooltip>

          {!isSubscription && (
            <WithTooltip id="checkout-tap-to-pay-tooltip" defaultText="Tap your card or phone on the reader to pay in person.">
              <button
                onClick={() => {
                  const amount = prompt("Enter amount to charge:");
                  if (!amount) return;

                  if (navigator.onLine) {
                    alert(`Payment of ${amount} successful!`);
                    router.push('/dashboard');
                  } else {
                    let queue = [];
                    try {
                      queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
                    } catch (e) {}

                    queue.push({
                      id: 'txn_' + Date.now(),
                      amount: parseFloat(amount),
                      timestamp: new Date().toISOString(),
                      type: 'tap_to_pay',
                      idempotency_key: 'idempotency_' + Date.now() + Math.random().toString(36).substring(7)
                    });
                    localStorage.setItem('ohc_offline_queue', JSON.stringify(queue));
                    alert(`You are offline. Payment of ${amount} saved locally and will process when reconnected.`);
                    router.push('/dashboard');
                  }
                }}
                className="w-full px-4 py-3 bg-blue-500 text-white rounded-lg font-medium hover:bg-blue-600 transition-colors shadow-sm"
              >
                Tap to Pay (Stripe Terminal)
              </button>
            </WithTooltip>
          )}

          {!isSubscription && (
            <WithTooltip id="checkout-mercadopago-tooltip" defaultText="Pay securely using Mercado Pago.">
              <button
                onClick={() => {
                  alert("Redirecting to Mercado Pago...");
                  setShowSuccessModal(true);
                }}
                className="w-full px-4 py-3 bg-[#009EE3] text-white rounded-lg font-medium hover:bg-[#007ebd] transition-colors shadow-sm flex items-center justify-center gap-2"
              >
                Pay with Mercado Pago
              </button>
            </WithTooltip>
          )}

          <WithTooltip id="checkout-cancel-tooltip" defaultText="Go back to the previous screen without buying anything.">
            <button
              onClick={() => router.push('/pricing')}
              className="w-full px-4 py-3 bg-gray-200 text-gray-800 rounded-lg font-medium hover:bg-gray-300 transition-colors"
            >
              Cancel
            </button>
          </WithTooltip>
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
            {isSubscription ? (
               <p className="text-gray-600 mb-6 text-sm leading-relaxed">
                 You're subscribed! We've sent you an SMS with a magic link to manage your subscription without needing a password.
               </p>
            ) : (
               <p className="text-gray-600 mb-6 text-sm leading-relaxed">
                 Your order is confirmed. Love what you bought? Share with your friends! When they buy, they get 10% off and you earn a <strong className="text-gray-900">$10 credit</strong>.
               </p>
            )}

            <div className="space-y-4">
              {!isSubscription && (
                <div>
                  <label className="block text-xs font-semibold text-gray-700 uppercase tracking-wide mb-2">Your Unique Link</label>
                  <div className="flex gap-2">
                    <input
                      type="text"
                      readOnly
                      value={referralLink}
                      className="flex-1 bg-gray-50 border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-600 focus:outline-none"
                    />
                    <button
                      onClick={() => {
                        navigator.clipboard.writeText(referralLink);
                        setCopied(true);
                        setTimeout(() => setCopied(false), 2000);
                      }}
                      className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
                    >
                      {copied ? 'Copied!' : 'Copy'}
                    </button>
                  </div>
                </div>
              )}

              {!isSubscription && (
                <div className="relative py-3">
                  <div className="absolute inset-0 flex items-center"><div className="w-full border-t border-gray-200"></div></div>
                  <div className="relative flex justify-center"><span className="bg-white px-2 text-xs text-gray-500 uppercase font-semibold tracking-wide">Or Share Via</span></div>
                </div>
              )}

              {!isSubscription && (
                <div className="grid grid-cols-2 gap-3 mb-6">
                  <a
                    href={`https://wa.me/?text=${encodeURIComponent(`I just bought an amazing product from this store! Use my link to get 10% off your first order: ${referralLink} ⚡ Powered by OHC`)}`}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="flex items-center justify-center gap-2 bg-[#25D366] text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all"
                  >
                    WhatsApp
                  </a>
                  <a
                    href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(`I just bought an amazing product from this store! Use my link to get 10% off your first order: ${referralLink} ⚡ Powered by OHC`)}`}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="flex items-center justify-center gap-2 bg-black text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all"
                  >
                    X (Twitter)
                  </a>
                </div>
              )}

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
