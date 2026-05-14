'use client';
import React, { useState, useEffect } from 'react';

export default function Wizard() {
  const [step, setStep] = useState(0);
  const [advancedMode, setAdvancedMode] = useState(false);
  const [businessName, setBusinessName] = useState('');
  const [businessType, setBusinessType] = useState('');
  const [category, setCategory] = useState('');
  const [productName, setProductName] = useState('');
  const [paymentChoice, setPaymentChoice] = useState('');
  const [template, setTemplate] = useState('');
  const [domain, setDomain] = useState('');
  const [adminName, setAdminName] = useState('');
  const [adminEmail, setAdminEmail] = useState('');
  const [adminPassword, setAdminPassword] = useState('');

  // Premium design tokens
  const containerStyle = {
    backdropFilter: 'blur(20px) saturate(200%)',
    background: 'rgba(255, 255, 255, 0.03)',
    fontFamily: "'Inter', sans-serif",
    padding: '24px',
    borderRadius: '16px',
    border: '1px solid rgba(255, 255, 255, 0.1)',
    boxShadow: '0 4px 6px rgba(0, 0, 0, 0.1)',
    color: '#fff',
    maxWidth: '500px',
    margin: '40px auto'
  };

  const headerStyle = {
    fontFamily: "'Outfit', sans-serif",
    fontSize: '32px',
    fontWeight: 700,
    marginBottom: '10px'
  };

  const buttonStyle = {
    padding: '12px 24px',
    background: '#00FFCC',
    color: '#0A0A0A',
    border: 'none',
    borderRadius: '8px',
    cursor: 'pointer',
    fontWeight: 600,
    marginTop: '10px',
    marginRight: '10px',
    transition: 'all 0.3s'
  };

  const inputStyle = {
    width: '100%',
    padding: '12px',
    margin: '10px 0',
    background: 'rgba(255, 255, 255, 0.05)',
    border: '1px solid rgba(255, 255, 255, 0.1)',
    borderRadius: '8px',
    color: '#fff'
  };

  const nextStep = () => {
    // In a real implementation this would trigger an API call to save state
    setStep((s) => s + 1);
  };

  return (
    <div style={{ minHeight: '100vh', background: '#0A0A0A', padding: '20px' }}>
      <div style={containerStyle}>
        <h2 style={headerStyle}>OHC Interactive Setup</h2>
        <p style={{ color: '#A0A0A0', marginBottom: '30px' }}>Your business, live in minutes.</p>

        {step === 0 && (
          <div>
            <button onClick={() => setStep(1)} style={buttonStyle}>Start My Business</button>
          </div>
        )}

        {step === 1 && (
          <div>
            <h3 style={{ fontSize: '20px', marginBottom: '15px' }}>What kind of business are you building?</h3>
            <button style={buttonStyle} onClick={() => { setBusinessType('Online Store'); }}>Online Store</button>
            <button style={{...buttonStyle, background: 'rgba(255,255,255,0.1)', color: '#fff'}} onClick={nextStep}>Next</button>
          </div>
        )}

        {step === 2 && (
          <div>
            <h3 style={{ fontSize: '20px', marginBottom: '15px' }}>What is your business called?</h3>
            <input type="text" style={inputStyle} placeholder="e.g. Maya's Cakes" value={businessName} onChange={(e) => setBusinessName(e.target.value)} />
            <button style={{...buttonStyle, background: 'rgba(255,255,255,0.1)', color: '#fff'}} onClick={() => {}}>Auto-suggest Description</button>
            <button style={buttonStyle} onClick={nextStep}>Next</button>

            {advancedMode && (
              <div style={{ marginTop: '20px', fontSize: '12px', color: '#A0A0A0' }}>
                <p>Config.slug: {businessName.toLowerCase().replace(/\s+/g, '-')}</p>
              </div>
            )}
          </div>
        )}

        {step === 3 && (
          <div>
            <h3 style={{ fontSize: '20px', marginBottom: '15px' }}>What do you sell?</h3>
            <button style={buttonStyle} onClick={() => { setCategory('Physical products'); }}>Physical products</button>
            <button style={{...buttonStyle, background: 'rgba(255,255,255,0.1)', color: '#fff'}} onClick={nextStep}>Next</button>
          </div>
        )}

        {step === 4 && (
          <div>
            <h3 style={{ fontSize: '20px', marginBottom: '15px' }}>Add your first product</h3>
            <input type="text" style={inputStyle} placeholder="What is the name of this product?" value={productName} onChange={(e) => setProductName(e.target.value)} />
            <input type="text" style={inputStyle} placeholder="0.00" />
            <button style={buttonStyle} onClick={nextStep}>Next</button>
          </div>
        )}

        {step === 5 && (
          <div>
            <h3 style={{ fontSize: '20px', marginBottom: '15px' }}>How do you want to receive payments?</h3>
            <button style={buttonStyle} onClick={() => { setPaymentChoice('Online only'); }}>Online only</button>
            <button style={{...buttonStyle, background: 'rgba(255,255,255,0.1)', color: '#fff'}} onClick={nextStep}>Next</button>
          </div>
        )}

        {step === 6 && (
          <div>
            <h3 style={{ fontSize: '20px', marginBottom: '15px' }}>Choose a Template</h3>
            <button style={buttonStyle} onClick={() => { setTemplate('Modern'); }}>Modern</button>
            <button style={{...buttonStyle, background: 'rgba(255,255,255,0.1)', color: '#fff'}} onClick={nextStep}>Next</button>
          </div>
        )}

        {step === 7 && (
          <div>
            <h3 style={{ fontSize: '20px', marginBottom: '15px' }}>Choose a Domain</h3>
            <button style={buttonStyle} onClick={() => { setDomain('Free OHC Domain'); }}>Free OHC Domain</button>
            <button style={{...buttonStyle, background: 'rgba(255,255,255,0.1)', color: '#fff'}} onClick={nextStep}>Next</button>
          </div>
        )}

        {step === 8 && (
          <div>
            <h3 style={{ fontSize: '20px', marginBottom: '15px' }}>Administrator account</h3>
            <input type="text" style={inputStyle} placeholder="e.g. Maya Smith" value={adminName} onChange={(e) => setAdminName(e.target.value)} />
            <input type="email" style={inputStyle} placeholder="you@email.com" value={adminEmail} onChange={(e) => setAdminEmail(e.target.value)} />
            <input type="password" style={inputStyle} placeholder="Password" value={adminPassword} onChange={(e) => setAdminPassword(e.target.value)} />
            <button style={buttonStyle} onClick={nextStep}>Review & Launch</button>
          </div>
        )}

        {step === 9 && (
          <div>
            <h3 style={{ fontSize: '20px', marginBottom: '15px' }}>Almost there</h3>
            <button style={buttonStyle} onClick={nextStep}>Launch!</button>
          </div>
        )}

        {step === 10 && (
          <div style={{ textAlign: 'center' }}>
            <h3 style={{ fontSize: '28px', color: '#00FFCC', marginBottom: '15px' }}>Onboarding Complete!</h3>
            <p style={{ color: '#A0A0A0' }}>Welcome to your new business.</p>
          </div>
        )}

        <div style={{ marginTop: '40px', borderTop: '1px solid rgba(255,255,255,0.1)', paddingTop: '20px' }}>
          <label style={{ display: 'flex', alignItems: 'center', cursor: 'pointer', fontSize: '14px', color: '#A0A0A0' }}>
            <input type="checkbox" checked={advancedMode} onChange={(e) => setAdvancedMode(e.target.checked)} style={{ marginRight: '10px' }} />
            Advanced Mode
          </label>
        </div>
      </div>
    </div>
  );
}
