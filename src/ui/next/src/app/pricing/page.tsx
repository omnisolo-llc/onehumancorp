'use client';
import React, { useState, useEffect } from 'react';

export default function PricingPage() {
  const [plans, setPlans] = useState<any[]>([]);

  useEffect(() => {
    // In a real application, we would fetch data from the actual backend
    const fetchPricing = async () => {
      try {
        const response = await fetch('/api/billing/pricing-plans');
        if (response.ok) {
          const data = await response.json();
          setPlans(data);
        } else {
          throw new Error('Failed to fetch from backend');
        }
      } catch (err) {
        // Fallback stub data for testing UI requirements
        setPlans([
          { name: 'Free', price: '$0 / month', agents: 1, storage: '500MB' },
          { name: 'Starter', price: '$10 / month', agents: 3, storage: '5GB' },
          { name: 'Pro', price: '$29 / month', agents: 10, storage: '50GB' },
          { name: 'Business', price: '$99 / month', agents: 'Unlimited', storage: 'Unlimited' }
        ]);
      }
    };
    fetchPricing();
  }, []);

  return (
    <div className="pricing-page" style={{ padding: '20px', fontFamily: 'Outfit, sans-serif' }}>
      <h1>Pricing Plan Comparison</h1>
      <p>Recommended plan: Pro</p>

      <div style={{ display: 'flex', gap: '20px', flexWrap: 'wrap' }}>
        {plans.map((plan: any) => (
          <div key={plan.name} className="plan-card" style={{ border: '1px solid #ccc', padding: '20px', borderRadius: '8px' }}>
            <h2>{plan.name} Plan</h2>
            <p style={{ fontSize: '24px', fontWeight: 'bold' }}>{plan.price}</p>
            <ul>
              <li>Number of agents: {plan.agents}</li>
              <li>Storage limit: {plan.storage}</li>
              <li>Community support</li>
              <li>Basic features</li>
              <li>Professional features</li>
              <li>Enterprise features</li>
            </ul>
            <button style={{ padding: '10px 20px', cursor: 'pointer' }}>
              {plan.name === 'Free' ? 'Start Free' : plan.name === 'Business' ? 'Contact Sales' : 'Choose ' + plan.name}
            </button>
          </div>
        ))}
      </div>

      <div style={{ marginTop: '20px' }}>
        <label>
          <input type="checkbox" /> Annual billing discount (20% savings)
        </label>
      </div>

      <div className="faq" style={{ marginTop: '40px' }}>
        <h2>FAQ</h2>
        <div className="question">
          <h3 style={{ cursor: 'pointer' }}>Question 1?</h3>
          <p className="answer">Answer 1 description</p>
        </div>
      </div>

      <div style={{ marginTop: '40px', display: 'flex', gap: '20px' }}>
        <span>100% money back guarantee</span>
        <span>SSL secure checkout</span>
      </div>
    </div>
  );
}
