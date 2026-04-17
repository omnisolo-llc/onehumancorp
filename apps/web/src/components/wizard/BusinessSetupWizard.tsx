import React, { useState } from 'react';
import { theme } from '../../styles/theme';

const pulseStyle = `\n  @keyframes pulse {\n    0% { transform: scale(1); }\n    50% { transform: scale(1.05); }\n    100% { transform: scale(1); }\n  }\n`;

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
      <style>{pulseStyle}</style>
    <div style={{ ...theme.glassmorphism, ...theme.typography, borderRadius: '12px', padding: '30px', color: theme.colors.text }}>
      {step === 1 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h1 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Your AI team, ready in minutes</h1>
          <button onClick={nextStep}>Next</button>
        </div>
      )}
      {step === 2 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Business Profile</h2>
          <input placeholder="Company Name" value={profile.name} onChange={e => setProfile({...profile, name: e.target.value})} style={{ display: 'block', margin: '10px 0', padding: '10px', width: '100%', borderRadius: '4px' }} />
          <select value={profile.industry} onChange={e => setProfile({...profile, industry: e.target.value})} style={{ display: 'block', margin: '10px 0', padding: '10px', width: '100%', borderRadius: '4px' }}>
            <option value="">Select Industry</option>
            <option value="tech">Tech (💻)</option>
            <option value="healthcare">Healthcare (🏥)</option>
            <option value="finance">Finance (💰)</option>
            <option value="retail">Retail (🏪)</option>
            <option value="other">Other (🏢)</option>
          </select>
          <select value={profile.size} onChange={e => setProfile({...profile, size: e.target.value})} style={{ display: 'block', margin: '10px 0', padding: '10px', width: '100%', borderRadius: '4px' }}>
            <option value="">Select Company Size</option>
            <option value="S">Small (1-50)</option>
            <option value="M">Medium (51-200)</option>
            <option value="L">Large (201-1000)</option>
            <option value="Enterprise">Enterprise (1000+)</option>
          </select>
          <select value={profile.language} onChange={e => setProfile({...profile, language: e.target.value})} style={{ display: 'block', margin: '10px 0', padding: '10px', width: '100%', borderRadius: '4px' }}>
            <option value="">Select Primary Language</option>
            <option value="english">English</option>
            <option value="spanish">Spanish</option>
            <option value="french">French</option>
            <option value="german">German</option>
          </select>
          <button onClick={prevStep}>Back</button>
          <button onClick={nextStep}>Next</button>
        </div>
      )}
      {step === 3 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Goal Selection</h2>
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
          <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Deployment Preference</h2>
          <select value={deployment} onChange={e => setDeployment(e.target.value)} title={deployment === 'cloud' ? 'Managed securely by OHC. Best for teams.' : deployment === 'desktop' ? 'Runs locally on your machine. Maximum privacy.' : 'Connects to APIs without a local database.'}>
            <option value="cloud" title="Managed securely by OHC. Best for teams.">Cloud (managed)</option>
            <option value="desktop" title="Runs locally on your machine. Maximum privacy.">Self-hosted Desktop</option>
            <option value="mobile" title="Connects to APIs without a local database.">Mobile-only</option>
          </select>
          <button onClick={prevStep}>Back</button>
          <button onClick={nextStep}>Next</button>
        </div>
      )}
      {step === 5 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Administrator Account</h2>
          <input placeholder="Name" value={admin.name} onChange={e => setAdmin({...admin, name: e.target.value})} style={{ display: 'block', margin: '10px 0', padding: '10px', width: '100%', borderRadius: '4px' }} />
          <input type="email" placeholder="Email" value={admin.email} onChange={e => setAdmin({...admin, email: e.target.value})} style={{ display: 'block', margin: '10px 0', padding: '10px', width: '100%', borderRadius: '4px' }} />
          <input type="password" placeholder="Password" value={admin.password} onChange={e => setAdmin({...admin, password: e.target.value})} style={{ display: 'block', margin: '10px 0', padding: '10px', width: '100%', borderRadius: '4px' }} />
          <div style={{ marginTop: '8px', marginBottom: '8px' }}>
            <div style={{ height: '4px', background: 'grey', width: '100%' }}>
              <div style={{ height: '4px', background: 'green', width: '50%' }}></div>
            </div>
            <p style={{ fontFamily: 'Inter', fontSize: '12px', marginTop: '4px' }}>Password Strength: Medium</p>
          </div>
          <div style={{ marginTop: '16px' }}>
            <button style={{ marginRight: '8px', padding: '8px 16px', background: '#DB4437', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>Sign in with Google</button>
            <button style={{ padding: '8px 16px', background: '#333', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>Sign in with GitHub</button>
          </div>
          <button onClick={prevStep}>Back</button>
          <button onClick={nextStep}>Next</button>
        </div>
      )}
      {step === 6 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Review & Launch</h2>
          <button onClick={prevStep}>Back</button>
                    <button
            onClick={launch}
            disabled={isLoading}
            style={{ animation: 'pulse 1.5s infinite ease-in-out', background: '#007AFF', color: '#FFFFFF', border: 'none', padding: '10px 20px', borderRadius: '8px', cursor: 'pointer', fontFamily: 'Inter', fontSize: '18px' }}
          >
            {isLoading ? 'Launching...' : 'Launch My AI Team →'}
          </button>

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
