"use client";
import { useSyncGateway } from "../../hooks/useSyncGateway";

import React, { useState, useEffect, Suspense } from "react";
import { useRouter, useSearchParams } from "next/navigation";
import { WithTooltip } from "../../components/TooltipRegistry";
import { PoweredByOHC } from "../components/PoweredByOHC";
import { OneTapReferral } from "../components/OneTapReferral";
import { PostPurchaseShareWidget } from "../components/PostPurchaseShareWidget";
import { ShareAndSaveWidget } from "../components/ShareAndSaveWidget";

function CheckoutContent() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const tier = searchParams?.get("tier");
  const productId = searchParams?.get("product_id") || "prod_123";
  const quantity = parseInt(searchParams?.get("quantity") || "1", 10);
  const discountParam = searchParams?.get("discount");
  const [isProcessing, setIsProcessing] = useState(false);
  const [isReserving, setIsReserving] = useState(false);
  const [showSuccessModal, setShowSuccessModal] = useState(false);
  const [tenant, setTenant] = useState("my-store");
  const [checkoutStatus, setCheckoutStatus] = useState("");
  const [isMercadoPagoProcessing, setIsMercadoPagoProcessing] = useState(false);
  const [shareDiscountApplied, setShareDiscountApplied] = useState(false);
  const [isSoldOut, setIsSoldOut] = useState(false);
  const { lastMessage } = useSyncGateway({
    topics: ["inventory"],
    enabled: !!tenant && !!productId,
  });

  useEffect(() => {
    if (
      lastMessage &&
      lastMessage.product_id === productId &&
      lastMessage.action === "reserve"
    ) {
      setIsSoldOut(true);
      setCheckoutStatus("Oops! Item just sold out.");
      setIsSoldOut(true);
    }
  }, [lastMessage, productId]);

  useEffect(() => {
    if (typeof localStorage !== "undefined") {
      setTenant(localStorage.getItem("tenant") || "my-store");
    }
  }, []);

  const [deliveryAddress, setDeliveryAddress] = useState("");
  const [deliveryFee, setDeliveryFee] = useState<number | null>(null);
  const [isCheckingDelivery, setIsCheckingDelivery] = useState(false);
  const [deliveryError, setDeliveryError] = useState<string | null>(null);
  const [useLoyaltyPoints, setUseLoyaltyPoints] = useState(false);
  const [availablePoints, setAvailablePoints] = useState(0);
  const [loyaltyDiscount, setLoyaltyDiscount] = useState<number | null>(null);
  const [isLoyaltyReady, setIsLoyaltyReady] = useState(false);
  const [productData, setProductData] = useState<any>(null);

  useEffect(() => {
    if (productId && tenant && !tier) {
      fetch(`/api/v1/pos/inventory?tenant_id=${tenant}`)
        .then((res) => res.json())
        .then((data) => {
          if (data && data.inventory) {
            const product = data.inventory.find((p: any) => p.id === productId);
            if (product) {
              setProductData(product);
            }
          }
        })
        .catch(console.error);
    }
  }, [productId, tenant, tier]);

  useEffect(() => {
    if (typeof localStorage !== "undefined") {
      const customerId = localStorage.getItem("customer_id") || "guest";
      const currentTenant = localStorage.getItem("tenant") || "my-store";
      fetch(
        `/api/v1/growth/loyalty?tenant_id=${currentTenant}&customer_id=${customerId}`,
      )
        .then((res) => {
          if (res.ok) {
            return res.json();
          }
          return { points_balance: 50 }; // Fallback to 50 for guests/failures
        })
        .then((data) => {
          setAvailablePoints(data.points_balance || 50);
          if ((data.points_balance || 50) >= 50) {
            setLoyaltyDiscount(0.1); // 10%
          }
          setIsLoyaltyReady(true);
        })
        .catch(() => {
          setAvailablePoints(50);
          setLoyaltyDiscount(0.1);
          setIsLoyaltyReady(true);
        });
    } else {
      setIsLoyaltyReady(true);
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
        body: JSON.stringify({ tenant_id: tenant, deliveryAddress }),
      });
      const data = await response.json();
      if (data.success && data.fee !== undefined) {
        setDeliveryFee(data.fee);
      } else {
        setDeliveryError(
          data.message || "Delivery not available for this address.",
        );
        setDeliveryFee(null);
      }
    } catch (e) {
      setDeliveryError("Error checking delivery.");
    } finally {
      setIsCheckingDelivery(false);
    }
  };

  const [isSubscription, setIsSubscription] = useState(false);
  const isSuccess = searchParams.get("success") === "true";

  const handlePayment = async (isSub = false) => {
    setIsProcessing(true);
    setCheckoutStatus("Preparing Checkout...");
    setIsSubscription(isSub);

    try {
      const response = await fetch("/api/billing/create-checkout-session", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          ...(typeof localStorage !== "undefined" &&
          localStorage.getItem("token")
            ? { Authorization: `Bearer ${localStorage.getItem("token")}` }
            : {}),
        },
        body: JSON.stringify({
          tier,
          is_subscription: tier ? true : isSub,
          product_id: tier ? undefined : productId,
          quantity: tier ? undefined : quantity,
        }),
      });
      if (response.status === 409) {
        setCheckoutStatus("Oops! Item just sold out.");
        setIsProcessing(false);
        return;
      }

      const data = await response.json();
      if (data.error_message && data.error_message.includes("sold out")) {
        setCheckoutStatus("Oops! Item just sold out.");
        setIsProcessing(false);
        return;
      }
      if (!response.ok || !data.checkout_url) {
        throw new Error(
          data.message || data.error || "Failed to create checkout session",
        );
      }

      setCheckoutStatus("Redirecting to checkout...");
      window.location.assign(data.checkout_url);
    } catch (e: any) {
      setCheckoutStatus("Checkout is temporarily unavailable.");
      setIsProcessing(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F8F9FA] text-gray-900 overflow-x-hidden">
      <header className="px-4 md:px-6 py-4 flex flex-col md:flex-row items-center justify-between gap-4 sticky top-0 z-50 app-panel-header shadow-sm">
        <WithTooltip
          id="checkout-title-tooltip"
          defaultText="Review your order or plan details before securely completing your purchase."
        >
          <h1 className="text-2xl font-bold font-outfit text-center md:text-left text-gray-900 tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-gray-900 to-gray-600">
            {isSuccess
              ? "Order Successful"
              : tier
                ? "Plan Upgrade"
                : "Secure Checkout"}
          </h1>
        </WithTooltip>
      </header>

      <main
        id="checkout-screen"
        className="p-4 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col justify-center"
      >
        {isSuccess ? (
          <div className="flex flex-col gap-6">
            <div className="p-8 flex flex-col justify-center items-center bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] shadow-lg rounded-[24px] text-center">
              <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mb-4">
                <svg
                  className="w-8 h-8 text-green-600"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">
                Thank you for your order!
              </h2>
              <p className="text-gray-600 mb-6">
                Your payment was successful and your order is being processed.
              </p>

              <button
                onClick={() => router.push("/")}
                className="w-full px-4 py-3 bg-gray-100 text-gray-800 rounded-lg font-medium hover:bg-gray-200 transition-colors"
              >
                Return to Store
              </button>
            </div>

            <PostPurchaseShareWidget
              tenantId={tenant || "default-store"}
              orderId={searchParams.get("orderId") || "success"}
            />
          </div>
        ) : tier ? (
          <div className="p-6 md:p-8 flex flex-col justify-between bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] shadow-lg rounded-[24px]">
            <div className="mb-6">
              <h3 className="text-2xl font-bold font-outfit mb-2 text-gray-900">
                OHC {tier} Plan
              </h3>
              <p className="text-gray-600 text-sm">
                You are upgrading to the {tier} tier. Your card will be charged
                based on your region's pricing.
              </p>
            </div>

            {checkoutStatus && (
              <p
                className="text-sm font-medium text-indigo-700 mb-4"
                role="status"
              >
                {checkoutStatus}
              </p>
            )}

            <WithTooltip
              id="checkout-plan-upgrade-tooltip"
              defaultText={
                "Click here to securely subscribe to the " + tier + " plan."
              }
            >
              <button
                onClick={() => handlePayment(true)}
                disabled={isProcessing || isSoldOut}
                className={
                  "w-full mb-3 px-4 py-3 text-white rounded-lg font-medium transition-colors shadow-sm " +
                  (isProcessing
                    ? "bg-indigo-400 cursor-not-allowed"
                    : "bg-indigo-600 hover:bg-indigo-700")
                }
              >
                {isProcessing ? "Processing..." : "Upgrade"}
              </button>
            </WithTooltip>

            <WithTooltip
              id="checkout-cancel-tooltip"
              defaultText="Go back to the previous screen without subscribing."
            >
              <button
                onClick={() => router.push("/pricing")}
                className="w-full px-4 py-3 bg-gray-200 text-gray-800 rounded-lg font-medium hover:bg-gray-300 transition-colors"
              >
                Cancel
              </button>
            </WithTooltip>
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
                <span className="font-semibold text-gray-700">
                  Service Deposit
                </span>
                <span className="text-xl font-bold font-outfit text-gray-900">
                  $45.00
                </span>
              </div>

              <div className="bg-white/60 backdrop-blur-[30px] saturate-[210%] border border-indigo-200/50 shadow-sm rounded-xl p-4 my-2">
                <p className="text-indigo-800 text-xs font-medium">
                  A 20% discount (
                  {discountParam && !isNaN(Number(discountParam))
                    ? discountParam
                    : "20"}
                  %) has been applied to this order automatically.
                </p>
              </div>

              {true && (
                <div
                  className="bg-indigo-50/50 border border-indigo-100/50 rounded-xl p-4 my-2 backdrop-blur-[30px] saturate-[210%] cursor-pointer hover:bg-indigo-50/80 transition-colors"
                  onClick={() => setUseLoyaltyPoints(!useLoyaltyPoints)}
                >
                  <div className="flex justify-between items-center">
                    <div>
                      <p className="text-indigo-900 text-sm font-bold font-outfit">
                        Neighborhood Collective Points
                      </p>
                      <p className="text-indigo-800 text-xs font-medium">
                        You have {availablePoints} points available
                      </p>
                    </div>
                    <div className="flex items-center gap-3">
                      <span className="text-indigo-600 text-sm font-bold">
                        {loyaltyDiscount
                          ? `-${loyaltyDiscount * 100}% off`
                          : "0% off"}
                      </span>
                      <div
                        className={`w-10 h-6 rounded-full flex items-center p-1 transition-colors ${useLoyaltyPoints ? "bg-indigo-600" : "bg-gray-300"}`}
                      >
                        <div
                          className={`bg-white w-4 h-4 rounded-full shadow-sm transform transition-transform ${useLoyaltyPoints ? "translate-x-4" : "translate-x-0"}`}
                        ></div>
                      </div>
                    </div>
                  </div>
                </div>
              )}

              <div className="flex flex-col gap-2">
                <label className="text-sm font-semibold text-gray-700">
                  Delivery Address (Optional)
                </label>
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
                {deliveryError && (
                  <p className="text-xs text-[#FF3B30] mt-1">{deliveryError}</p>
                )}
                {deliveryFee !== null && (
                  <p className="text-xs text-green-600 mt-1 font-medium">
                    Delivery available: +${deliveryFee.toFixed(2)}
                  </p>
                )}
              </div>

              {!tier && productData && productData.is_subscribable && (
                <div className="flex items-center mb-4">
                  <label
                    htmlFor="subscribe"
                    className="flex items-center cursor-pointer group"
                  >
                    <div className="relative">
                      <input
                        type="checkbox"
                        id="subscribe"
                        className="sr-only"
                        checked={isSubscription}
                        onChange={(e) => setIsSubscription(e.target.checked)}
                      />
                      <div
                        className={`block w-10 h-6 rounded-full transition-colors duration-300 ease-in-out ${isSubscription ? "bg-indigo-500 shadow-[0_0_10px_rgba(99,102,241,0.5)]" : "bg-gray-300"}`}
                      ></div>
                      <div
                        className={`dot absolute left-1 top-1 bg-white w-4 h-4 rounded-full transition-transform duration-300 ease-in-out shadow-sm ${isSubscription ? "transform translate-x-4" : ""}`}
                      ></div>
                    </div>
                    <div className="ml-3 text-sm font-medium text-gray-700 group-hover:text-gray-900 transition-colors">
                      Subscribe & Save{" "}
                      {productData.subscription_discount_percent || 10}%
                    </div>
                  </label>
                </div>
              )}

              <div className="bg-green-50 border border-green-200/50 shadow-sm rounded-xl p-4 my-2 mb-4">
                <div className="flex justify-between items-center">
                  <span className="text-green-800 text-sm font-bold">
                    Available Rewards
                  </span>
                  <span className="text-green-800 text-sm font-bold">
                    1 Reward Available
                  </span>
                </div>
                <p className="text-green-700 text-xs font-medium mt-1">
                  You have earned a free coffee! Tap 'Pay' to automatically
                  apply your reward at checkout.
                </p>
              </div>

              <ShareAndSaveWidget
                tenantId={tenant}
                discountPercentage={10}
                onShareComplete={() => setShareDiscountApplied(true)}
              />

              <div className="flex justify-between items-center pt-2 border-t border-gray-100">
                <span className="font-semibold text-gray-700">
                  Taxes and Fees
                </span>
                <span className="text-sm font-medium text-gray-500">
                  Calculated at checkout
                </span>
              </div>

              {deliveryFee !== null && (
                <div className="flex justify-between items-center pt-2 border-t border-gray-100">
                  <span className="font-semibold text-gray-700">
                    Total with Delivery
                  </span>
                  <span className="text-xl font-bold font-outfit text-gray-900">
                    $
                    {(
                      (45.0 + deliveryFee) *
                      (useLoyaltyPoints && loyaltyDiscount
                        ? 1 - loyaltyDiscount
                        : 1) *
                      (shareDiscountApplied ? 0.9 : 1)
                    ).toFixed(2)}
                  </span>
                </div>
              )}
              {deliveryFee === null && (
                <div className="flex justify-between items-center pt-2 border-t border-gray-100">
                  <span className="font-semibold text-gray-700">Total</span>
                  <span className="text-xl font-bold font-outfit text-gray-900">
                    $
                    {(
                      45.0 *
                      (useLoyaltyPoints && loyaltyDiscount
                        ? 1 - loyaltyDiscount
                        : 1) *
                      (shareDiscountApplied ? 0.9 : 1)
                    ).toFixed(2)}
                  </span>
                </div>
              )}

              <WithTooltip
                id="checkout-pay-tooltip"
                defaultText="Tap to quickly and securely pay for your order."
              >
                <button
                  onClick={() => handlePayment(isSubscription)}
                  disabled={isProcessing}
                  className="w-full px-4 py-3 bg-black text-white rounded-lg font-medium hover:bg-gray-900 transition-colors shadow-sm flex items-center justify-center gap-2"
                >
                  {isSoldOut
                    ? "Sold Out"
                    : isProcessing
                      ? "Processing..."
                      : "Pay"}
                </button>
              </WithTooltip>

              {checkoutStatus && checkoutStatus !== "Oops! Item just sold out." && (
                <p
                  className="text-sm font-medium text-indigo-700"
                  role="status"
                >
                  {checkoutStatus}
                </p>
              )}
              {checkoutStatus === "Oops! Item just sold out." && (
                <div className="fixed bottom-0 left-0 right-0 p-6 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] z-[100] shadow-2xl rounded-t-[24px]">
                  <div className="flex items-start gap-4 max-w-lg mx-auto">
                    <div className="w-12 h-12 bg-red-100 rounded-full flex-shrink-0 flex items-center justify-center text-xl text-red-600">
                      <svg
                        className="w-6 h-6"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                        />
                      </svg>
                    </div>
                    <div>
                      <h3 className="text-lg font-bold text-gray-900 mb-1 font-outfit">
                        Oops! Item just sold out.
                      </h3>
                      <p className="text-gray-600 text-sm leading-relaxed mb-4">
                        Someone at our physical store is buying the last one
                        right now. Check back in 15 minutes or browse similar
                        items.
                      </p>
                      <button
                        onClick={() => setCheckoutStatus("")}
                        className="px-4 py-2 bg-gray-900 text-white rounded-lg text-sm font-medium hover:bg-gray-800 transition-colors"
                      >
                        Got it
                      </button>
                    </div>
                  </div>
                </div>
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
            </div>
          </>
        )}

        {/* Powered By OHC Footer */}
        <div className="flex justify-center mt-6 w-full relative z-[9999] pb-8">
          <PoweredByOHC tenantId={tenant} />
        </div>
      </main>

      {/* Post-Purchase Referral Modal */}
      {showSuccessModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="app-card w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter border border-green-200/50 shadow-sm">
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

            <div className="bg-white/60 backdrop-blur-[30px] saturate-[210%] border border-indigo-200/50 shadow-sm rounded-xl p-4 mb-6">
              <div className="flex items-center gap-3 mb-2">
                <span className="text-xl">💰</span>
                <h3 className="font-bold text-indigo-900 font-outfit text-sm">
                  Become an Affiliate
                </h3>
              </div>
              <p className="text-indigo-800 text-xs font-medium">
                Give a 20% discount to friends and get a 10% commission when
                they make their first purchase!{" "}
                <a
                  href={`/onboarding?ref=${tenant}&source=checkout_affiliate`}
                  target="_blank"
                  className="font-bold hover:underline"
                  onClick={(e) => {
                    fetch("/api/v1/growth/referrals/click", {
                      method: "POST",
                      headers: { "Content-Type": "application/json" },
                      body: JSON.stringify({
                        referrer_id: tenant,
                        source: "checkout_affiliate",
                      }),
                    }).catch(() => {
                      /* ignore */
                    });
                  }}
                >
                  ⚡ Powered by OHC
                </a>
              </p>
            </div>

            <div className="space-y-4">
              <div className="mb-6">
                <OneTapReferral tenantId={tenant} source="checkout_affiliate" />
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
    <Suspense
      fallback={
        <div className="min-h-screen flex items-center justify-center font-inter text-gray-500">
          Loading Checkout...
        </div>
      }
    >
      <CheckoutContent />
    </Suspense>
  );
}
