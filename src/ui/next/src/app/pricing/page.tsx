"use client";
import React, { useEffect, useState } from 'react';

export default function PricingPage() {
  const [checkoutOpen, setCheckoutOpen] = useState(false);
  const [myPlan, setMyPlan] = useState<any>(null);

  useEffect(() => {
    fetch('http://localhost:18789/my-plan')
      .then(res => res.json())
      .then(data => setMyPlan(data))
      .catch(e => console.error(e));
  }, []);

  return (
    <div className="p-8">
      <h1>Pricing Plans</h1>
      <p>Secure SSL payments powered by Stripe.</p>

      <div className="grid grid-cols-4 gap-4 mt-8">
        <div className="border p-4 rounded">
          <h2>Free</h2>
          <p>100 AI actions / month</p>
          <button>Current Plan</button>
        </div>
        <div className="border p-4 rounded">
          <h2>Starter</h2>
          <p>1,000 AI actions / month</p>
          <button onClick={() => setCheckoutOpen(true)}>Upgrade to Starter</button>
        </div>
        <div className="border p-4 rounded">
          <h2>Pro</h2>
          <p>Unlimited AI actions</p>
          <button onClick={() => setCheckoutOpen(true)}>Upgrade to Pro via Stripe</button>
        </div>
        <div className="border p-4 rounded">
          <h2>Business</h2>
          <p>Unlimited AI actions + Custom Domain</p>
          <button onClick={() => setCheckoutOpen(true)}>Upgrade to Business</button>
        </div>
      </div>

      {checkoutOpen && (
        <div id="checkout-screen" className="mt-8 p-4 border bg-gray-50">
          <h2>Checkout</h2>
          <p>Secure SSL payments.</p>
        </div>
      )}
    </div>
  );
}
