"use client";

import React, { useState, useEffect, Suspense } from "react";
import { useRouter, useSearchParams } from "next/navigation";
import { WithTooltip } from "../../components/TooltipRegistry";
import { PoweredByOHC } from "../components/PoweredByOHC";

function CheckoutContent() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const tier = searchParams?.get("tier");
  const discountParam = searchParams?.get("discount");
  const [isProcessing, setIsProcessing] = useState(false);
  const [showSuccessModal, setShowSuccessModal] = useState(false);
  const [referralLink, setReferralLink] = useState("");
  const [copied, setCopied] = useState(false);
  const [tenant, setTenant] = useState("my-store");
  const [checkoutStatus, setCheckoutStatus] = useState("");
  const [isMercadoPagoProcessing, setIsMercadoPagoProcessing] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== "undefined") {
      setTenant(localStorage.getItem("tenant") || "my-store");
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
        body: JSON.stringify({ tenant_id: tenant, deliveryAddress }),
      });
      const data = await response.json();
      if (data.success && data.fee !== undefined) {
        setDeliveryFee(data.fee);
      } else {
        setDeliveryError(data.message || "Delivery not available for this address.");
        setDeliveryFee(null);
      }
    } catch (e) {
      console.error("Failed to check delivery eligibility", e);
      setDeliveryError("Error checking delivery.");
    } finally {
      setIsCheckingDelivery(false);
    }
  };

  const [isSubscription, setIsSubscription] = useState(false);


  const handlePlanUpgrade = async () => {
    setIsProcessing(true);
    setCheckoutStatus("Preparing Stripe Checkout...");
    try {
      const response = await fetch("/api/billing/create-checkout-session", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          ...(typeof localStorage !== "undefined" && localStorage.getItem('token') ? { "Authorization": `Bearer ${localStorage.getItem('token')}` } : {})
        },
        body: JSON.stringify({ tier, is_subscription: isSubscription }),
      });
      const data = await response.json();
      if (!response.ok || !data.checkout_url) {
        throw new Error(data.message || "Failed to create checkout session");
      }
      window.location.assign(data.checkout_url);
    } catch (e) {
      console.error("Failed to start checkout", e);
      setCheckoutStatus("Stripe Checkout is temporarily unavailable.");
      setIsProcessing(false);
    }
  };

  const startMercadoPagoCheckout = async () => {

    setIsMercadoPagoProcessing(true);
    setCheckoutStatus("Preparing Mercado Pago checkout...");

    let amount_cents = 4500;
    if (tier) {
        if (tier.toLowerCase() === 'starter') amount_cents = 2900;
        else if (tier.toLowerCase() === 'pro') amount_cents = 7900;
        else if (tier.toLowerCase() === 'business') amount_cents = 29900;
    }

    try {
      const response = await fetch("/api/checkout/mercadopago", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tenant_id: tenant,
          amount_cents: amount_cents,
          currency: "MXN",
        }),
      });
      const data = await response.json();
      if (!response.ok || !data.checkout_url) {
        throw new Error(data.error || "Mercado Pago checkout unavailable.");
      }

      setCheckoutStatus("Redirecting to Mercado Pago...");
      window.location.assign(data.checkout_url);
    } catch (e) {
      console.error("Failed to start Mercado Pago checkout", e);
      setCheckoutStatus("Mercado Pago checkout is temporarily unavailable.");
    } finally {
      setIsMercadoPagoProcessing(false);
    }
  };

  const handlePayment = async (isSub = false) => {
    setIsProcessing(true);
    setCheckoutStatus("Simulating local POS / Tap to Pay processing...");
    setIsSubscription(isSub);
    // Simulate API delay for tap to pay
    setTimeout(() => {
      setReferralLink("https://ohc.inc/ref/" + Math.random().toString(36).substring(7));
      setShowSuccessModal(true);
      setIsProcessing(false);
      setCheckoutStatus("");
    }, 1500);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F8F9FA] text-gray-900 overflow-x-hidden">
      <header className="px-4 md:px-6 py-4 flex flex-col md:flex-row items-center justify-between gap-4 sticky top-0 z-50 bg-white/70 backdrop-blur-xl saturate-200 border-b border-white/40 shadow-sm">
        <WithTooltip
          id="checkout-title-tooltip"
          defaultText="Review your order or plan details before securely completing your purchase."
        >
          <h1 className="text-2xl font-bold font-outfit text-center md:text-left text-gray-900 tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-gray-900 to-gray-600">
            {tier ? "Plan Upgrade" : "Secure Checkout"}
          </h1>
        </WithTooltip>
      </header>

      <main
        id="checkout-screen"
        className="p-4 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col justify-center"
      >
        {tier ? (
            <div className="p-6 md:p-8 flex flex-col justify-between bg-white/60 backdrop-blur-2xl saturate-200 border border-white/40 shadow-lg rounded-[24px]">
              <div className="mb-6">
                <h3 className="text-2xl font-bold font-outfit mb-2 text-gray-900">OHC {tier} Plan</h3>
                <p className="text-gray-600 text-sm">You are upgrading to the {tier} tier. Your card will be charged based on your region's pricing.</p>
              </div>

              {checkoutStatus && (
                <p className="text-sm font-medium text-indigo-700 mb-4" role="status">
                  {checkoutStatus}
                </p>
              )}

              <WithTooltip id="checkout-plan-upgrade-tooltip" defaultText={"Click here to securely subscribe to the " + tier + " plan."}>
                <button
                  onClick={handlePlanUpgrade}
                  disabled={isProcessing || isMercadoPagoProcessing}
                  className={"w-full mb-3 px-4 py-3 text-white rounded-lg font-medium transition-colors shadow-sm " + (isProcessing ? 'bg-indigo-400 cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700')}
                >
                  {isProcessing ? 'Processing...' : 'Upgrade via Stripe'}
                </button>
              </WithTooltip>

              <WithTooltip id="checkout-mercadopago-plan-upgrade-tooltip" defaultText={"Click here to securely subscribe to the " + tier + " plan via Mercado Pago."}>
                <button
                  onClick={startMercadoPagoCheckout}
                  disabled={isProcessing || isMercadoPagoProcessing}
                  className={"w-full mb-4 px-4 py-3 bg-[#009EE3] text-white rounded-lg font-medium hover:bg-[#007ebd] transition-colors shadow-sm flex items-center justify-center gap-2 " + (isMercadoPagoProcessing ? 'opacity-70 cursor-not-allowed' : '')}
                >
                  {isMercadoPagoProcessing ? 'Preparing Mercado Pago...' : 'Upgrade via Mercado Pago'}
                </button>
              </WithTooltip>

              <WithTooltip id="checkout-cancel-tooltip" defaultText="Go back to the previous screen without subscribing.">
                <button
                  onClick={() => router.push('/pricing')}
                  className="w-full px-4 py-3 bg-gray-200 text-gray-800 rounded-lg font-medium hover:bg-gray-300 transition-colors"
                >
                  Cancel
                </button>
              </WithTooltip>
              <PoweredByOHC tenantId={tenant} />
            </div>
        ) : (
          <>
        <div
          className="p-6 shadow-sm flex flex-col gap-4 mb-4"
          style={{
            background: "rgba(255, 255, 255, 0.65)",
            backdropFilter: "blur(30px) saturate(210%)",
            borderRadius: "24px",
            border: "1px solid rgba(255, 255, 255, 0.4)",
          }}
        >
          <div className="flex justify-between items-center pb-4 border-b border-gray-100">
            <span className="font-semibold text-gray-700">Service Deposit</span>
            <span className="text-xl font-bold font-outfit text-gray-900">
              $45.00
            </span>
          </div>

          <div className="bg-indigo-50 border border-indigo-100 rounded-xl p-4 my-2">
            <p className="text-indigo-800 text-xs font-medium">
              A 20% discount ({(discountParam && !isNaN(Number(discountParam))) ? discountParam : "20"}%) has been applied to this order automatically.
            </p>
          </div>

          <div className="flex flex-col gap-2">
            <label className="text-sm font-semibold text-gray-700">Delivery Address (Optional)</label>
            <div className="flex gap-2">
              <input
                type="text"
                value={deliveryAddress}
                onChange={(e) => setDeliveryAddress(e.target.value)}
                placeholder="Enter address for delivery quote"
                className="flex-1 px-3 py-2 bg-white border border-gray-200 rounded-lg text-sm text-gray-800 focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
              <button
                onClick={checkDeliveryEligibility}
                disabled={!deliveryAddress || isCheckingDelivery}
                className="px-3 py-2 bg-gray-100 text-gray-700 text-sm font-medium rounded-lg hover:bg-gray-200 disabled:opacity-50 transition-colors"
              >
                {isCheckingDelivery ? "Checking..." : "Check"}
              </button>
            </div>
            {deliveryError && <p className="text-xs text-red-500 mt-1">{deliveryError}</p>}
            {deliveryFee !== null && <p className="text-xs text-green-600 mt-1 font-medium">Delivery available: +${deliveryFee.toFixed(2)}</p>}
          </div>

          <div className="flex items-center gap-2 mb-4">
            <input type="checkbox" id="subscribe" checked={isSubscription} onChange={(e) => setIsSubscription(e.target.checked)} className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500" />
            <label htmlFor="subscribe" className="text-sm font-medium text-gray-700">Subscribe & Save 10%</label>
          </div>

          {deliveryFee !== null && (
             <div className="flex justify-between items-center pt-2 border-t border-gray-100">
               <span className="font-semibold text-gray-700">Total with Delivery</span>
               <span className="text-xl font-bold font-outfit text-gray-900">
                 ${(45.00 + deliveryFee).toFixed(2)}
               </span>
             </div>
          )}

          <WithTooltip
            id="checkout-apple-pay-tooltip"
            defaultText="Tap to quickly and securely pay using Apple Pay."
          >
            <button
              onClick={() => handlePayment(isSubscription)}
              disabled={isProcessing}
              className="w-full px-4 py-3 bg-black text-white rounded-lg font-medium hover:bg-gray-900 transition-colors shadow-sm flex items-center justify-center gap-2"
            >
              <svg className="w-5 h-5" viewBox="0 0 384 512" fill="white">
                <path d="M318.7 268.7c-.2-36.7 16.4-64.4 50-84.8-18.8-26.9-47.2-41.7-84.7-44.6-35.5-2.8-74.3 20.7-88.5 20.7-15 0-49.4-19.7-76.4-19.7C63.3 141.2 4 184.8 4 273.5q0 39.3 14.4 81.2c12.8 36.7 59 126.7 107.2 125.2 25.2-.6 43-17.9 75.8-17.9 31.8 0 48.3 17.9 76.4 17.9 48.6-.7 90.4-82.5 102.6-119.3-65.2-30.7-61.7-90-61.7-91.9zm-56.6-164.2c27.3-32.4 24.8-61.9 24-72.5-24.1 1.4-52 16.4-67.9 34.9-17.5 19.8-27.8 44.3-25.6 71.9 26.1 2 49.9-11.4 69.5-34.3z" />
              </svg>
              {isProcessing ? "Processing..." : "Pay"}
            </button>
          </WithTooltip>

          <WithTooltip
            id="checkout-tap-to-pay-tooltip"
            defaultText="Use your mobile device as a terminal to accept contactless payments."
          >
            <button
              onClick={() => handlePayment(isSubscription)}
              disabled={isProcessing}
              className="w-full px-4 py-3 bg-indigo-50 text-indigo-700 rounded-lg font-medium hover:bg-indigo-100 transition-colors border border-indigo-100 shadow-sm flex items-center justify-center gap-2"
            >
              Pay with Stripe
            </button>
          </WithTooltip>

          <WithTooltip
            id="checkout-mercadopago-tooltip"
            defaultText="Pay securely using Mercado Pago."
          >
            <button
              onClick={startMercadoPagoCheckout}
              disabled={isMercadoPagoProcessing}
              className="w-full px-4 py-3 bg-[#009EE3] text-white rounded-lg font-medium hover:bg-[#007ebd] transition-colors shadow-sm flex items-center justify-center gap-2"
            >
              {isMercadoPagoProcessing
                ? "Preparing Mercado Pago..."
                : "Pay with Mercado Pago"}
            </button>
          </WithTooltip>

          {checkoutStatus && (
            <p className="text-sm font-medium text-indigo-700" role="status">
              {checkoutStatus}
            </p>
          )}

          <WithTooltip
            id="checkout-cancel-tooltip"
            defaultText="Go back to the previous screen without buying anything."
          >
            <button
              onClick={() => router.push("/pricing")}
              className="w-full px-4 py-3 bg-gray-200 text-gray-800 rounded-lg font-medium hover:bg-gray-300 transition-colors"
            >
              Cancel
            </button>
          </WithTooltip>
          <PoweredByOHC tenantId={tenant} />
        </div>
        </>
        )}
      </main>

      {/* Post-Purchase Referral Modal */}
      {showSuccessModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="app-card w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter border border-green-100">
            {/* Background embellishment */}
            <div className="absolute top-0 right-0 w-32 h-32 bg-green-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-between items-start mb-4">
              <div className="w-12 h-12 bg-green-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-green-600">
                🎉
              </div>
            </div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">
              Payment Successful!
            </h2>
            {isSubscription ? (
              <p className="text-gray-600 mb-6 text-sm leading-relaxed">
                You're in! We'll text you a magic link to manage your
                subscription anytime. Love what you bought? Share with your
                friends! When they buy, they get 20% off and you earn a{" "}
                <strong className="text-gray-900">10% commission</strong>.
              </p>
            ) : (
              <p className="text-gray-600 mb-6 text-sm leading-relaxed">
                Your order is confirmed. Love what you bought? Share with your
                friends! When they buy, they get 20% off and you earn a{" "}
                <strong className="text-gray-900">10% commission</strong>.
              </p>
            )}

            <div className="bg-indigo-50 border border-indigo-100 rounded-xl p-4 mb-6">
              <div className="flex items-center gap-3 mb-2">
                <span className="text-xl">💰</span>
                <h3 className="font-bold text-indigo-900 font-outfit text-sm">
                  Become an Affiliate
                </h3>
              </div>
              <p className="text-indigo-800 text-xs font-medium">
                Give a 20% discount to friends and get a 10% commission when they
                make their first purchase! ⚡ Powered by OHC
              </p>
            </div>

            <div className="space-y-4">
              <div>
                <label className="block text-xs font-semibold text-gray-700 uppercase tracking-wide mb-2">
                  Your Unique Link
                </label>
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
                    className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all ${copied ? "bg-green-100 text-green-700" : "bg-gray-900 text-white hover:bg-black"}`}
                  >
                    {copied ? "Copied!" : "Copy"}
                  </button>
                </div>
              </div>

              <div className="relative py-3">
                <div className="absolute inset-0 flex items-center">
                  <div className="w-full border-t border-gray-200"></div>
                </div>
                <div className="relative flex justify-center">
                  <span className="bg-white px-2 text-xs text-gray-500 uppercase font-semibold tracking-wide">
                    Or Share Via
                  </span>
                </div>
              </div>

              <div className="flex flex-col gap-3 mb-6">
                <a
                  href={`https://wa.me/?text=${encodeURIComponent(`I just bought an amazing product from this store! Use my link to get 20% off your first order: ${referralLink} ⚡ Powered by OHC`)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-[#25D366] text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all"
                >
                  <svg
                    className="w-5 h-5"
                    fill="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z" />
                  </svg>
                  WhatsApp
                </a>
                <a
                  href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(`I just bought an amazing product from this store! Use my link to get 20% off your first order: ${referralLink} ⚡ Powered by OHC`)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-black text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all"
                >
                  <svg
                    className="w-4 h-4"
                    fill="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z" />
                  </svg>
                  X (Twitter)
                </a>
                <a
                  href={`https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(referralLink)}&quote=${encodeURIComponent(`I just bought an amazing product from this store! Use my link to get 20% off your first order: ${referralLink} ⚡ Powered by OHC`)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-[#1877F2]/80 text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#166fe5] transition-all"
                >
                  <svg
                    className="w-5 h-5"
                    fill="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.469h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.469h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z" />
                  </svg>
                  Share on Facebook
                </a>
              </div>

              <button
                onClick={() => router.push("/dashboard")}
                className="w-full px-4 py-3 text-indigo-600 bg-indigo-50 rounded-lg font-medium hover:bg-indigo-100 transition-colors"
              >
                Continue to Dashboard
              </button>
            </div>
          </div>
        </div>
      )}

      <style
        dangerouslySetInnerHTML={{
          __html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `,
        }}
      />
    </div>
  );
}

export default function CheckoutPage() {
  return (
    <Suspense fallback={<div className="min-h-screen flex items-center justify-center font-inter text-gray-500">Loading Checkout...</div>}>
      <CheckoutContent />
    </Suspense>
  );
}
