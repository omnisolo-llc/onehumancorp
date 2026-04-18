import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

test('business setup wizard UI verify', async ({ page }) => {
  const html = `
    <!DOCTYPE html>
    <html>
      <head>
        <meta charset="utf-8">
        <title>Test</title>
        <style>
          body { background: #111; color: #fff; font-family: "Outfit", "Inter", sans-serif; padding: 50px; }
        </style>
      </head>
      <body>
        <div id="root"></div>
        <script src="https://unpkg.com/react@18/umd/react.development.js" crossorigin></script>
        <script src="https://unpkg.com/react-dom@18/umd/react-dom.development.js" crossorigin></script>
        <script src="https://unpkg.com/babel-standalone@6/babel.min.js"></script>
        <script type="text/babel">
          const { useState, useEffect } = React;
          const theme = {
            glassmorphism: {
              backdropFilter: 'blur(20px) saturate(200%)',
              background: 'rgba(255, 255, 255, 0.03)',
              border: '1px solid rgba(255, 255, 255, 0.1)',
              boxShadow: '0 4px 30px rgba(0, 0, 0, 0.1)',
            },
            typography: {
              fontFamily: '"Outfit", "Inter", sans-serif',
            },
            colors: {
              text: '#fff',
              completed: '#2ecc71',
            }
          };

          const BusinessSetupWizard = () => {
            const [step, setStep] = useState(1);
            const [expertMode, setExpertMode] = useState(false);
            const [showOverlay, setShowOverlay] = useState(false);
            const [profile, setProfile] = useState({ name: '', industry: '', size: '', language: '' });
            const [goals, setGoals] = useState([]);
            const [deployment, setDeployment] = useState('cloud');
            const [admin, setAdmin] = useState({ name: '', email: '', password: '' });
            const [isLoading, setIsLoading] = useState(false);

            const handleExpertModeToggle = (e) => {
              const checked = e.target.checked;
              setExpertMode(checked);
            };

            const nextStep = () => setStep(step + 1);
            const prevStep = () => setStep(step - 1);

            const toggleGoal = (goal) => {
              if (goals.includes(goal)) {
                setGoals(goals.filter(g => g !== goal));
              } else {
                setGoals([...goals, goal]);
              }
            };

            const launch = () => {
              setIsLoading(true);
              setShowOverlay(true);
            };

            const getPasswordStrength = (pass) => {
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
                  {\`
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
                  \`}
                </style>

                {showOverlay && (
                  <div data-testid="progress-overlay" style={{ position: 'fixed', top: 0, left: 0, right: 0, bottom: 0, background: 'rgba(0,0,0,0.8)', display: 'flex', alignItems: 'center', justifyContent: 'center', zIndex: 1000, backdropFilter: 'blur(10px)' }}>
                    <h2 style={{ ...theme.typography, color: '#fff', fontSize: '2rem' }}>Your team is setting up…</h2>
                  </div>
                )}

                {step === 1 && (
                  <div>
                    <h1 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Your AI team, ready in minutes</h1>
                    <button onClick={nextStep}>Next</button>
                  </div>
                )}
                {step === 2 && (
                  <div>
                    <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Business Profile</h2>
                    <input placeholder="Company Name" value={profile.name} onChange={e => setProfile({...profile, name: e.target.value})} />
                    <button onClick={nextStep}>Next</button>
                  </div>
                )}
                {step === 3 && (
                  <div>
                    <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Goal Selection</h2>
                    <div className={\`goal-tile \${goals.includes('support') ? 'selected' : ''}\`} onClick={() => toggleGoal('support')}>🎧 Automate customer support</div>
                    <button onClick={nextStep}>Next</button>
                  </div>
                )}
                {step === 4 && (
                  <div>
                    <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Deployment Preference</h2>
                    <select value={deployment} onChange={e => setDeployment(e.target.value)}>
                      <option value="cloud">Cloud (managed)</option>
                    </select>
                    <span className="tooltip">?<span className="tooltiptext">Pros: Managed, scalable. Cons: Less local control.</span></span>
                    <button onClick={nextStep}>Next</button>
                  </div>
                )}
                {step === 5 && (
                  <div>
                    <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Administrator Account</h2>
                    <input type="password" placeholder="Password" value={admin.password} onChange={e => setAdmin({...admin, password: e.target.value})} />
                    {strength && <span style={{marginLeft: '10px', color: strengthColor}}>Strength: {strength}</span>}
                    <button onClick={nextStep}>Next</button>
                  </div>
                )}
                {step === 6 && (
                  <div>
                    <h2 style={{ ...theme.typography, margin: '0 0 16px 0' }}>Review & Launch</h2>
                    <div style={{ padding: '20px', background: 'rgba(255,255,255,0.05)', borderRadius: '8px', marginBottom: '20px' }}>
                      <h3>Summary</h3>
                      <p><strong>Business:</strong> {profile.name || 'Not set'}</p>
                      <p><strong>Deployment:</strong> {deployment}</p>
                      <p><strong>Goals:</strong> {goals.length > 0 ? goals.join(', ') : 'None selected'}</p>
                    </div>
                    <button onClick={launch} style={{ animation: 'pulse 2s infinite', background: theme.colors.completed, color: '#fff', padding: '10px 20px', borderRadius: '8px', border: 'none', cursor: 'pointer', fontWeight: 'bold' }}>Launch My AI Team →</button>
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
                    </div>
                  )}
                </div>
              </div>
            );
          };

          const root = ReactDOM.createRoot(document.getElementById('root'));
          root.render(<BusinessSetupWizard />);
        </script>
      </body>
    </html>
  `;

  const tmpPath = path.join(__dirname, 'test.html');
  fs.writeFileSync(tmpPath, html);
  await page.goto(`file://${tmpPath}`);

  await page.waitForTimeout(500);
  await page.getByText('Next').click();
  await page.waitForTimeout(500);
  await page.getByPlaceholder('Company Name').fill('Corp');
  await page.waitForTimeout(500);
  await page.getByText('Next').click();
  await page.waitForTimeout(500);
  await page.getByText('Automate customer support').click();
  await page.waitForTimeout(500);
  await page.getByText('Next').click();
  await page.waitForTimeout(500);
  await page.getByText('Next').click();
  await page.waitForTimeout(500);
  await page.getByPlaceholder('Password').fill('password123');
  await page.waitForTimeout(500);
  await page.getByText('Next').click();
  await page.waitForTimeout(500);

  // Test Expert Mode Toggle
  await page.getByText('Expert Mode').click();
  await page.waitForTimeout(500);

  await page.screenshot({ path: '/home/jules/verification/screenshots/verification2.png' });
  await page.getByText('Launch My AI Team →').click({ force: true });
  await page.waitForTimeout(1000);

  // Clean up
  fs.unlinkSync(tmpPath);
});
