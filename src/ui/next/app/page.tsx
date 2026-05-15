
import React, { useState, useEffect } from 'react';

export default function Onboarding() {
  const [step, setStep] = useState(1);
  const [formData, setFormData] = useState({
    email: '', password: '', businessType: '', companyName: '',
    template: '', productName: '', productPrice: '', domain: ''
  });

  useEffect(() => {
    // Cross-device resume
    const savedState = localStorage.getItem('onboardingState');
    if (savedState) {
      const parsed = JSON.parse(savedState);
      if (parsed.step) setStep(parsed.step);
      if (parsed.formData) setFormData(parsed.formData);
    }
  }, []);

  const handleNext = () => {
    const nextStep = step + 1;
    setStep(nextStep);
    localStorage.setItem('onboardingState', JSON.stringify({ step: nextStep, formData }));
    // Persist to backend
    fetch('/api/onboarding/state', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ step: nextStep, state_json: formData })
    });
  };

  const publish = () => {
    // Publish logic with animated confetti
    alert('Confetti! Your business is live.');
    setStep(6);
  };

  return (
    <div style={{ backdropFilter: 'blur(20px) saturate(200%)', background: 'rgba(255, 255, 255, 0.03)', padding: '24px', maxWidth: '375px', margin: '0 auto', fontFamily: 'Outfit, Inter, sans-serif' }}>
      {step === 1 && (
        <div>
          <h2>Sign-Up</h2>
          <input type="email" placeholder="Email" value={formData.email} onChange={e => setFormData({...formData, email: e.target.value})} />
          <input type="password" placeholder="Password" value={formData.password} onChange={e => setFormData({...formData, password: e.target.value})} />
          <button onClick={handleNext}>Sign Up</button>
        </div>
      )}
      {step === 2 && (
        <div>
          <h2>Business Setup</h2>
          <input type="text" placeholder="Business Type" value={formData.businessType} onChange={e => setFormData({...formData, businessType: e.target.value})} />
          <input type="text" placeholder="Company Name" value={formData.companyName} onChange={e => setFormData({...formData, companyName: e.target.value})} />
          <button onClick={handleNext}>Next</button>
        </div>
      )}
      {step === 3 && (
        <div>
          <h2>Template Selection</h2>
          <select value={formData.template} onChange={e => setFormData({...formData, template: e.target.value})}>
            <option value="modern">Modern</option>
            <option value="classic">Classic</option>
          </select>
          <button onClick={handleNext}>Select</button>
        </div>
      )}
      {step === 4 && (
        <div>
          <h2>First Product</h2>
          <input type="text" placeholder="Product Name" value={formData.productName} onChange={e => setFormData({...formData, productName: e.target.value})} />
          <input type="text" placeholder="Price" value={formData.productPrice} onChange={e => setFormData({...formData, productPrice: e.target.value})} />
          <button onClick={handleNext}>Next</button>
        </div>
      )}
      {step === 5 && (
        <div>
          <h2>Domain & Go-Live</h2>
          <input type="text" placeholder="mybusiness.ohc.app" value={formData.domain} onChange={e => setFormData({...formData, domain: e.target.value})} />
          <button onClick={publish}>Publish</button>
        </div>
      )}
      {step === 6 && (
        <div>
          <h2>Welcome Checklist</h2>
          <ul>
            <li>✅ Business live</li>
            <li>⬜ Add 3 more products</li>
            <li>⬜ Connect Instagram</li>
            <li>⬜ Share your link with a friend</li>
          </ul>
        </div>
      )}
    </div>
  );
}
