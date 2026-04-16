import React, { useState } from 'react';

const BusinessSetupWizard = () => {
  const [step, setStep] = useState(1);
      const [expertMode, setExpertMode] = useState(false);

  React.useEffect(() => {
    if (typeof window !== 'undefined') {
      const saved = localStorage.getItem('expertMode');
      if (saved === 'true') {
        setExpertMode(true);
      }
    }
  }, []);

  const handleExpertModeToggle = (e: React.ChangeEvent<HTMLInputElement>) => {
    const checked = e.target.checked;
    setExpertMode(checked);
    if (typeof window !== 'undefined') {
      localStorage.setItem('expertMode', checked.toString());
    }
  };
  const [profile, setProfile] = useState({ name: '', industry: '', size: '', language: '' });
  const [goals, setGoals] = useState<string[]>([]);
  const [deployment, setDeployment] = useState('cloud');
  const [admin, setAdmin] = useState({ name: '', email: '', password: '' });
  const [isLoading, setIsLoading] = useState(false);

  const nextStep = () => setStep(step + 1);
  const prevStep = () => setStep(step - 1);

  const toggleGoal = (goal: string) => {
    if (goals.includes(goal)) {
      setGoals(goals.filter(g => g !== goal));
    } else {
      setGoals([...goals, goal]);
    }
  };

  const launch = async () => {
    setIsLoading(true);
    try {
      const response = await fetch('/api/provision', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ profile, goals, deployment, admin })
      });
      if (response.ok) {
        alert('Launched!');
      } else {
        alert('Failed to launch');
      }
    } catch (e) {
      alert('Error launching');
    }
    setIsLoading(false);
  };

  return (
    <>

      <style>{`
        @keyframes fadeIn {
          from { opacity: 0; transform: translateY(10px); }
          to { opacity: 1; transform: translateY(0); }
        }
        @keyframes pulse {
          0% { box-shadow: 0 0 0 0 rgba(0, 123, 255, 0.7); }
          70% { box-shadow: 0 0 0 10px rgba(0, 123, 255, 0); }
          100% { box-shadow: 0 0 0 0 rgba(0, 123, 255, 0); }
        }
        .pulse-btn {
          animation: pulse 2s infinite;
          background: #007BFF;
          color: white;
          border: none;
          padding: 10px 20px;
          border-radius: 8px;
          cursor: pointer;
          font-family: 'Outfit', sans-serif;
          font-weight: bold;
        }
        .tile {
          display: inline-block;
          padding: 15px;
          margin: 10px;
          border-radius: 8px;
          background: rgba(255,255,255,0.1);
          cursor: pointer;
          border: 1px solid transparent;
          text-align: center;
          width: 120px;
        }
        .tile.selected {
          border-color: #007BFF;
          background: rgba(0,123,255,0.2);
        }
        .tile input { display: none; }
      `}</style>

    <div style={{ backdropFilter: 'blur(20px) saturate(200%)', background: 'rgba(255, 255, 255, 0.03)', borderRadius: '12px', padding: '30px', color: '#fff', fontFamily: 'Inter, sans-serif', boxShadow: '0 0 20px rgba(255,255,255,0.1)' }}>
      {step === 1 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <div style={{ textAlign: 'center', marginBottom: '20px' }}>
            <h1 style={{ fontFamily: 'Outfit, sans-serif', fontSize: '2.5rem', margin: '0' }}>Your AI team, ready in minutes.</h1>
            <p style={{ fontFamily: 'Inter, sans-serif', fontSize: '1.2rem', color: '#ccc' }}>Zero friction. Maximum visual delight.</p>
          </div>
          <button onClick={nextStep}>Next</button>
        </div>
      )}
      {step === 2 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ fontFamily: 'Outfit, sans-serif' }}>Business Profile</h2>
          <input placeholder="Company Name" value={profile.name} onChange={e => setProfile({...profile, name: e.target.value})} />
          <select value={profile.industry} onChange={e => setProfile({...profile, industry: e.target.value})}>
            <option value="">Select Industry</option>
            <option value="tech">Tech</option>
            <option value="finance">Finance</option>
          </select>
          <select value={profile.size} onChange={e => setProfile({...profile, size: e.target.value})}>
            <option value="">Select Size</option>
            <option value="S">Small</option>
            <option value="M">Medium</option>
            <option value="L">Large</option>
            <option value="Enterprise">Enterprise</option>
          </select>
          <input placeholder="Language" value={profile.language} onChange={e => setProfile({...profile, language: e.target.value})} />
          <button onClick={prevStep}>Back</button>
          <button onClick={nextStep}>Next</button>
        </div>
      )}
      {step === 3 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ fontFamily: 'Outfit, sans-serif' }}>Goal Selection</h2>
          <div style={{ display: 'flex', flexWrap: 'wrap', justifyContent: 'center' }}>
            {[
              { id: 'support', label: 'Automate customer support', icon: '🎧' },
              { id: 'software', label: 'Build software faster', icon: '💻' },
              { id: 'marketing', label: 'Generate marketing content', icon: '📈' },
              { id: 'data', label: 'Analyze data', icon: '📊' },
              { id: 'custom', label: 'Custom', icon: '⚙️' }
            ].map(g => (
              <div key={g.id} className={`tile ${goals.includes(g.id) ? 'selected' : ''}`} onClick={() => toggleGoal(g.id)}>
                <div style={{ fontSize: '2rem' }}>{g.icon}</div>
                <div style={{ fontFamily: 'Inter, sans-serif', fontSize: '0.9rem', marginTop: '10px' }}>{g.label}</div>
              </div>
            ))}
          </div>
          <button onClick={prevStep}>Back</button>
          <button onClick={nextStep}>Next</button>
        </div>
      )}
      {step === 4 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ fontFamily: 'Outfit, sans-serif' }}>Deployment Preference</h2>
          <select value={deployment} onChange={e => setDeployment(e.target.value)}>
            <option value="cloud">Cloud (managed)</option>
            <option value="desktop">Self-hosted Desktop</option>
            <option value="mobile">Mobile-only</option>
          </select>
          <button onClick={prevStep}>Back</button>
          <button onClick={nextStep}>Next</button>
        </div>
      )}
      {step === 5 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ fontFamily: 'Outfit, sans-serif' }}>Administrator Account</h2>
          <input placeholder="Name" value={admin.name} onChange={e => setAdmin({...admin, name: e.target.value})} />
          <input type="email" placeholder="Email" value={admin.email} onChange={e => setAdmin({...admin, email: e.target.value})} />
          <input type="password" placeholder="Password" value={admin.password} onChange={e => setAdmin({...admin, password: e.target.value})} />
          <button onClick={prevStep}>Back</button>
          <button onClick={nextStep}>Next</button>
        </div>
      )}
      {step === 6 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ fontFamily: 'Outfit, sans-serif' }}>Review & Launch</h2>
          <button onClick={prevStep}>Back</button>
          <button className="pulse-btn" onClick={launch} disabled={isLoading}>{isLoading ? 'Launching...' : 'Launch My AI Team →'}</button>
        </div>
      )}

      <div style={{ marginTop: '20px' }}>
        <label>
          <input type="checkbox" checked={expertMode} onChange={handleExpertModeToggle} />
          Expert Mode
        </label>
        {expertMode && (
          <div style={{ background: '#222', padding: '10px', marginTop: '10px', fontFamily: 'monospace' }}>
            <p>Raw Config Fields Revealed</p>
            <pre>{JSON.stringify({ profile, goals, deployment, admin }, null, 2)}</pre>
          </div>
        )}
      </div>
    </div>
    </>
  );
};

export default BusinessSetupWizard;
