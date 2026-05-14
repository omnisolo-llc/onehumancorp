'use client';
import { useState } from 'react';
import { TooltipWrapper } from '../components/TooltipWrapper';
import { WalkthroughOverlay } from '../components/WalkthroughOverlay';

export default function Home() {
  const [showWalkthrough, setShowWalkthrough] = useState(false);
  const [walkthroughStep, setWalkthroughStep] = useState(0);

  const steps = [
    {
      selector: '#nav-store-settings',
      title: 'Welcome to your Store!',
      content: 'Let\'s get your business online. First, click here to open your store settings.'
    },
    {
      selector: '#input-store-name',
      title: 'Name your business',
      content: 'Type the name your customers know you by.'
    }
  ];

  const handleNext = () => {
    if (walkthroughStep < steps.length - 1) {
      setWalkthroughStep(walkthroughStep + 1);
    } else {
      setShowWalkthrough(false);
      setWalkthroughStep(0);
    }
  };

  return (
    <main style={{ padding: '40px', maxWidth: '800px', margin: '0 auto' }}>
      <h1>One Human Corp Dashboard</h1>
      <p>Welcome to your control panel.</p>

      <div style={{ margin: '40px 0', padding: '20px', background: 'white', borderRadius: '8px', boxShadow: '0 2px 4px rgba(0,0,0,0.1)' }}>
        <h2>Quick Actions</h2>

        <TooltipWrapper registryKey="marketing.campaign.start_button">
          <button style={{ padding: '10px 20px', background: '#0070f3', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer', marginRight: '10px' }}>
            Start Marketing Campaign
          </button>
        </TooltipWrapper>

        <button
          onClick={() => { setShowWalkthrough(true); setWalkthroughStep(0); }}
          style={{ padding: '10px 20px', background: '#eee', color: '#333', border: '1px solid #ccc', borderRadius: '4px', cursor: 'pointer' }}
        >
          Take a Tour
        </button>
      </div>

      <div style={{ display: 'flex', gap: '20px' }}>
        <div id="nav-store-settings" style={{ padding: '20px', background: 'white', borderRadius: '8px', flex: 1, border: '1px solid #eee' }}>
          <h3>Store Settings</h3>
          <p>Configure your shop here.</p>
        </div>

        <div id="input-store-name" style={{ padding: '20px', background: 'white', borderRadius: '8px', flex: 1, border: '1px solid #eee' }}>
          <h3>Business Info</h3>
          <input type="text" placeholder="Your Business Name" style={{ padding: '8px', width: '100%', boxSizing: 'border-box' }} />
        </div>
      </div>

      <WalkthroughOverlay
        isActive={showWalkthrough}
        targetSelector={steps[walkthroughStep]?.selector || ''}
        title={steps[walkthroughStep]?.title || ''}
        content={steps[walkthroughStep]?.content || ''}
        onDismiss={() => setShowWalkthrough(false)}
        onNext={handleNext}
      />
    </main>
  );
}
