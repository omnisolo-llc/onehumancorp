import React, { useState } from 'react';

export const Wizard = () => {
  const [step, setStep] = useState(0);
  const [name, setName] = useState('');
  const [type, setType] = useState('');
  const [goal, setGoal] = useState('');
  const [link, setLink] = useState('');

  const handleNext = async () => {
    if (step === 2) {
      const response = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, type, goal }),
      });
      const data = await response.json();
      setLink(data.public_link);
    }
    setStep(step + 1);
  };

  return (
    <div className="wizard-container" style={{ backdropFilter: 'blur(20px) saturate(200%)', fontFamily: 'Outfit, Inter, sans-serif' }}>
      {step === 0 && (
        <div>
          <h2>What is your Business Name?</h2>
          <input value={name} onChange={(e) => setName(e.target.value)} />
        </div>
      )}
      {step === 1 && (
        <div>
          <h2>What is your Business Type?</h2>
          <input value={type} onChange={(e) => setType(e.target.value)} />
        </div>
      )}
      {step === 2 && (
        <div>
          <h2>What is your primary goal?</h2>
          <input value={goal} onChange={(e) => setGoal(e.target.value)} />
        </div>
      )}
      {step === 3 && (
        <div>
          <h2>Success! 🎉</h2>
          <p>Your business is live at: {link}</p>
        </div>
      )}
      {step < 3 && <button onClick={handleNext}>Next</button>}
    </div>
  );
};
