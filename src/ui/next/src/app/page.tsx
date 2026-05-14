"use client";
import React, { useState } from 'react';

export default function BusinessSetup() {
  const [step, setStep] = useState(0);
  const [businessType, setBusinessType] = useState('');
  const [businessName, setBusinessName] = useState('');
  const [product, setProduct] = useState('');
  const [price, setPrice] = useState('');

  // Back button handler
  const handleBack = () => setStep(prev => Math.max(0, prev - 1));

  if (step === 0) {
    return (
      <div className="glass">
        <h1>Your business, live in minutes.</h1>
        <p>Zero tech skills needed. We do the heavy lifting.</p>
        <button onClick={() => setStep(1)}>🚀 Start My Business</button>
        <button onClick={() => setStep(100)}>⚡ Instant Build (AI) →</button>
        <button onClick={() => setStep(200)}>Guided Setup</button>
      </div>
    );
  }

  if (step === 1) {
    return (
      <div className="glass">
        <h2>What kind of business are you building?</h2>
        <button onClick={() => { setBusinessType('Online Store'); setStep(2); }}>🛒 Online Store</button>
        <button onClick={() => { setBusinessType('Service Business'); setStep(2); }}>Service Business</button>
        <button onClick={() => { setBusinessType('Restaurant / Food'); setStep(2); }}>Restaurant / Food</button>
        <button onClick={() => { setBusinessType('Creative'); setStep(2); }}>Creative</button>
        <button onClick={() => { setBusinessType('Local Business'); setStep(2); }}>Local Business</button>
        <button onClick={handleBack}>Back</button>
      </div>
    );
  }

  if (step === 2) {
    return (
      <div className="glass">
        <h2>Give your business a name</h2>
        <input
          placeholder="e.g. Maya's Cakes"
          value={businessName}
          onChange={e => setBusinessName(e.target.value)}
        />
        <button onClick={() => setStep(3)}>Next →</button>
        <button onClick={handleBack}>Back</button>
      </div>
    );
  }

  if (step === 3) {
    return (
      <div className="glass">
        <h2>What do you sell?</h2>
        <button>📦 Physical products</button>
        <button>📅 Services / appointments</button>
        <button>🍕 Food & beverages</button>
        <button>🖼️ Portfolios / Galleries</button>
        <button>🔁 Subscriptions</button>
        <button onClick={() => setStep(4)}>Next →</button>
      </div>
    );
  }

  if (step === 4) {
    return (
      <div className="glass">
        <h2>How do you want to get paid?</h2>
        <button onClick={() => setStep(5)}>🌐 Online only</button>
        <button onClick={() => setStep(5)}>🤝 In-person (Take payments on your phone)</button>
        <button onClick={() => setStep(5)}>🌍 Both Online & In-person</button>
        <button onClick={() => setStep(5)}>⏭️ Skip for now</button>
        <button onClick={() => setStep(5)}>Next →</button>
      </div>
    );
  }

  if (step === 5) {
    return (
      <div className="glass">
        <h2>Admin Account</h2>
        <input placeholder="e.g. Maya Smith" />
        <input placeholder="you@email.com" />
        <input type="password" placeholder="Password" />
        <button onClick={() => setStep(6)}>Next</button>
      </div>
    );
  }

  if (step === 6) {
    return (
      <div className="glass">
        <h2>Choose a Template</h2>
        <button>✨ Modern</button>
        <button>🔥 Bold</button>
        <button>Classic</button>
        <button onClick={() => setStep(7)}>Next →</button>
      </div>
    );
  }

  if (step === 7) {
    return (
      <div className="glass">
        <h2>Add your first product</h2>
        <input placeholder="e.g. Custom Birthday Cake" value={product} onChange={e => setProduct(e.target.value)} />
        <input placeholder="e.g. 50.00" value={price} onChange={e => setPrice(e.target.value)} />
        <button onClick={() => setStep(8)}>Next →</button>
      </div>
    );
  }

  if (step === 8) {
    return (
      <div className="glass">
        <h2>Choose a domain</h2>
        <button>🌐 Free OHC Domain</button>
        <button>🔗 Connect Custom Domain</button>
        <button>🌍 Connect Custom Domain</button>
        <button onClick={() => setStep(9)}>Next →</button>
      </div>
    );
  }

  if (step === 9) {
    return (
      <div className="glass">
        <h2>Go Live</h2>
        <button onClick={() => setStep(10)}>Publish my business →</button>
      </div>
    );
  }

  if (step === 10) {
    return (
      <div className="glass">
        <h2>CONFETTI SUCCESS</h2>
      </div>
    );
  }

  // Instant Build (AI) Flow
  if (step === 100) {
    return (
      <div className="glass">
        <h2>Describe your business in a sentence</h2>
        <input placeholder="e.g. I run a local bakery called Maya's Cakes..." />
        <button onClick={() => setStep(101)}>Generate Storefront →</button>
        <button onClick={() => setStep(0)}>Back</button>
      </div>
    );
  }

  if (step === 101) {
    return (
      <div className="glass">
        <h2>Designing your storefront...</h2>
        <button onClick={() => setStep(102)}>Launch My Business →</button>
        <button onClick={() => setStep(100)}>Back</button>
      </div>
    );
  }

  if (step === 102) {
    return (
      <div className="glass">
        <h2>Your live storefront!</h2>
        <p>AI Store</p>
        <button onClick={() => setStep(103)}>Continue to Dashboard →</button>
      </div>
    );
  }

  if (step === 103) {
    return (
      <div className="glass">
        <h2>Dashboard</h2>
      </div>
    );
  }

  // Guided Setup Flow (from Creative Portfolio test)
  if (step === 200) {
    return (
      <div className="glass">
        <h2>What kind of business are you building?</h2>
        <button onClick={() => { setBusinessType('Online Store'); setStep(2); }}>🛒 Online Store</button>
        <button onClick={() => { setBusinessType('Service Business'); setStep(2); }}>Service Business</button>
        <button onClick={() => { setBusinessType('Restaurant / Food'); setStep(2); }}>Restaurant / Food</button>
        <button onClick={() => { setBusinessType('Creative'); setStep(2); }}>Creative</button>
        <button onClick={() => { setBusinessType('Local Business'); setStep(2); }}>Local Business</button>
        <button onClick={() => setStep(0)}>Back</button>
      </div>
    );
  }

  return null;
}
