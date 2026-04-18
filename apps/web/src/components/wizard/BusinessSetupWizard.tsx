import React, { useState, useEffect } from 'react';
import { theme } from '../../styles/theme';

const BusinessSetupWizard = () => {
  const [step, setStep] = useState(1);
  const [expertMode, setExpertMode] = useState(false);
  const [showOverlay, setShowOverlay] = useState(false);

  React.useEffect(() => {
    if (typeof window !== 'undefined') {
      const saved = localStorage.getItem('expertMode');
      if (saved === 'true') {
        setExpertMode(true);
      }
    }
    const fetchState = async () => {
      try {
        const res = await fetch('/api/wizard/state');
        if (res.ok) {
          const data = await res.json();
          if (data && data.step) {
            setStep(data.step);
            if (data.profile) setProfile(data.profile);
            if (data.goals) setGoals(data.goals);
            if (data.deployment) setDeployment(data.deployment);
            if (data.admin) setAdmin(data.admin);
          }
        }
      } catch (e) {
        console.error("Failed to load wizard state", e);
      }
    };
    fetchState();
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

  const saveState = async (newStep: number) => {
    try {
      await fetch('/api/wizard/state/save', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ step: newStep, profile, goals, deployment, admin })
      });
    } catch(e) {
      console.error("Failed to save wizard state", e);
    }
  };

  const nextStep = () => {
    const n = step + 1;
    setStep(n);
    saveState(n);
  };
  const prevStep = () => {
    const p = step - 1;
    setStep(p);
    saveState(p);
  };

  const toggleGoal = (goal: string) => {
    if (goals.includes(goal)) {
      setGoals(goals.filter(g => g !== goal));
    } else {
      setGoals([...goals, goal]);
    }
  };

  const launch = async () => {
    setIsLoading(true);
    setShowOverlay(true);
    try {
      const response = await fetch('/api/provision', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ profile, goals, deployment, admin })
      });
      if (response.ok) {
        // success
      } else {
        alert('Failed to launch');
        setShowOverlay(false);
      }
    } catch (e) {
      console.error("Failed to provision AI team", e);
      alert('Error launching');
      setShowOverlay(false);
    }
    setIsLoading(false);
  };

  const getPasswordStrength = (pass: string) => {
    if (pass.length === 0) return '';
    if (pass.length < 6) return 'Weak';
    if (pass.length < 10) return 'Medium';
    return 'Strong';
  };

  const strength = getPasswordStrength(admin.password);
  const strengthColor = strength === 'Weak' ? 'red' : strength === 'Medium' ? 'orange' : 'green';

  return (
    <div style={{ ...theme.glassmorphism, ...theme.typography, borderRadius: '12px', padding: '30px', color: theme.colors.text }}>
      <style>
        {`
          @keyframes pulse {
            0% { transform: scale(1); box-shadow: 0 0 0 0 rgba(46, 204, 113, 0.7); }
            70% { transform: scale(1.05); box-shadow: 0 0 0 10px rgba(46, 204, 113, 0); }
            100% { transform: scale(1); box-shadow: 0 0 0 0 rgba(46, 204, 113, 0); }
          }
          .goal-tile { padding: 10px; margin: 5px 0; border: 1px solid #555; border-radius: 8px; cursor: pointer; display: flex; align-items: center; gap: 10px; }
          .goal-tile.selected { background: rgba(255,255,255,0.2); border-color: #fff; }
          .tooltip { position: relative; display: inline-block; cursor: help; border-bottom: 1px dotted #ccc; margin-left: 8px; }
          .tooltip .tooltiptext { visibility: hidden; width: 200px; background-color: #333; color: #fff; text-align: center; border-radius: 6px; padding: 5px; position: absolute; z-index: 1; bottom: 125%; left: 50%; margin-left: -100px; opacity: 0; transition: opacity 0.3s; }
          .tooltip:hover .tooltiptext { visibility: visible; opacity: 1; }
        `}
      </style>

      {showOverlay && (
        <div data-testid="progress-overlay" style={{ position: 'fixed', top: 0, left: 0, right: 0, bottom: 0, background: 'rgba(0,0,0,0.8)', display: 'flex', alignItems: 'center', justifyContent: 'center', zIndex: 1000, backdropFilter: 'blur(10px)' }}>
          <h2 style={{ ...theme.typography, color: '#fff', fontSize: '2rem' }}>Your team is setting up…</h2>
        </div>
      )}

      {step === 1 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h1 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Your AI team, ready in minutes</h1>
          <button onClick={nextStep}>Next</button>
        </div>
      )}
      {step === 2 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Business Profile</h2>
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
          <div style={{marginTop: '20px'}}>
            <button onClick={prevStep}>Back</button>
            <button onClick={nextStep}>Next</button>
          </div>
        </div>
      )}
      {step === 3 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Goal Selection</h2>
          <div className={`goal-tile ${goals.includes('support') ? 'selected' : ''}`} onClick={() => toggleGoal('support')}>🎧 Automate customer support</div>
          <div className={`goal-tile ${goals.includes('software') ? 'selected' : ''}`} onClick={() => toggleGoal('software')}>💻 Build software faster</div>
          <div className={`goal-tile ${goals.includes('marketing') ? 'selected' : ''}`} onClick={() => toggleGoal('marketing')}>📝 Generate marketing content</div>
          <div className={`goal-tile ${goals.includes('data') ? 'selected' : ''}`} onClick={() => toggleGoal('data')}>📊 Analyze data</div>
          <div className={`goal-tile ${goals.includes('custom') ? 'selected' : ''}`} onClick={() => toggleGoal('custom')}>⚙️ Custom</div>
          <div style={{marginTop: '20px'}}>
            <button onClick={prevStep}>Back</button>
            <button onClick={nextStep}>Next</button>
          </div>
        </div>
      )}
      {step === 4 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Deployment Preference</h2>
          <select value={deployment} onChange={e => setDeployment(e.target.value)}>
            <option value="cloud">Cloud (managed)</option>
            <option value="desktop">Self-hosted Desktop</option>
            <option value="mobile">Mobile-only</option>
          </select>
          {deployment === 'cloud' && <span className="tooltip">?<span className="tooltiptext">Pros: Managed, scalable. Cons: Less local control.</span></span>}
          {deployment === 'desktop' && <span className="tooltip">?<span className="tooltiptext">Pros: Local control. Cons: Needs hardware.</span></span>}
          {deployment === 'mobile' && <span className="tooltip">?<span className="tooltiptext">Pros: Portable. Cons: Limited features.</span></span>}
          <div style={{marginTop: '20px'}}>
            <button onClick={prevStep}>Back</button>
            <button onClick={nextStep}>Next</button>
          </div>
        </div>
      )}
      {step === 5 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Administrator Account</h2>
          <input placeholder="Name" value={admin.name} onChange={e => setAdmin({...admin, name: e.target.value})} /><br/>
          <input type="email" placeholder="Email" value={admin.email} onChange={e => setAdmin({...admin, email: e.target.value})} /><br/>
          <input type="password" placeholder="Password" value={admin.password} onChange={e => setAdmin({...admin, password: e.target.value})} />
          {strength && <span style={{marginLeft: '10px', color: strengthColor}}>Strength: {strength}</span>}
          <div style={{marginTop: '20px'}}>
            <button onClick={prevStep}>Back</button>
            <button onClick={nextStep}>Next</button>
          </div>
        </div>
      )}
      {step === 6 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Review & Launch</h2>
          <div style={{ padding: '20px', background: 'rgba(255,255,255,0.05)', borderRadius: '8px', marginBottom: '20px' }}>
            <h3>Summary</h3>
            <p><strong>Business:</strong> {profile.name || 'Not set'}</p>
            <p><strong>Deployment:</strong> {deployment}</p>
            <p><strong>Goals:</strong> {goals.length > 0 ? goals.join(', ') : 'None selected'}</p>
          </div>
          <div style={{marginTop: '20px', display: 'flex', gap: '10px', alignItems: 'center'}}>
            <button onClick={prevStep}>Back</button>
            <button
              onClick={launch}
              disabled={isLoading}
              style={{ animation: 'pulse 2s infinite', background: theme.colors.completed || '#2ecc71', color: '#fff', padding: '10px 20px', borderRadius: '8px', border: 'none', cursor: 'pointer', fontWeight: 'bold' }}
            >
              {isLoading ? 'Launching...' : 'Launch My AI Team →'}
            </button>
          </div>
        </div>
      )}

      <div style={{ marginTop: '40px', borderTop: '1px solid rgba(255,255,255,0.1)', paddingTop: '10px' }}>
        <label>
          <input type="checkbox" checked={expertMode} onChange={handleExpertModeToggle} />
          Expert Mode
        </label>
        {expertMode && (
          <div style={{ background: '#222', padding: '10px', marginTop: '10px', fontFamily: 'monospace' }}>
            <p>Raw Config Fields Revealed</p>
            <pre>{JSON.stringify({ profile, goals, deployment, admin }, null, 2)}</pre>
            <p><strong>API Endpoint:</strong> /api/provision</p>
            <p><strong>CLI Command:</strong> curl -X POST /api/provision -H "Content-Type: application/json" -d '{JSON.stringify({ profile, goals, deployment, admin })}'</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default BusinessSetupWizard;
