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
    <div style={{ backdropFilter: 'blur(20px)', background: 'rgba(255, 255, 255, 0.05)', borderRadius: '12px', padding: '30px', color: '#fff', fontFamily: 'Inter, sans-serif', boxShadow: '0 0 20px rgba(255,255,255,0.1)' }}>
      {step === 1 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h1 style={{ fontFamily: 'Outfit, sans-serif' }}>Your AI team, ready in minutes</h1>
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
          <label><input type="checkbox" checked={goals.includes('support')} onChange={() => toggleGoal('support')} /> Automate customer support</label><br />
          <label><input type="checkbox" checked={goals.includes('software')} onChange={() => toggleGoal('software')} /> Build software faster</label><br />
          <label><input type="checkbox" checked={goals.includes('marketing')} onChange={() => toggleGoal('marketing')} /> Generate marketing content</label><br />
          <label><input type="checkbox" checked={goals.includes('data')} onChange={() => toggleGoal('data')} /> Analyze data</label><br />
          <label><input type="checkbox" checked={goals.includes('custom')} onChange={() => toggleGoal('custom')} /> Custom</label><br />
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
          <button onClick={launch} disabled={isLoading}>{isLoading ? 'Launching...' : 'Launch My AI Team &gt;'}</button>
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
  );
};

export default BusinessSetupWizard;
