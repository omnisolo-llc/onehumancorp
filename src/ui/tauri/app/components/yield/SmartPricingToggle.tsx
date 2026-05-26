import React, { useState } from 'react';

interface Props {
  enabled: boolean;
  minPrice: number;
  maxPrice: number;
  onChange: (enabled: boolean, minPrice: number, maxPrice: number) => void;
}

export function SmartPricingToggle({ enabled, minPrice, maxPrice, onChange }: Props) {
  const [isOpen, setIsOpen] = useState(false);
  const [localEnabled, setLocalEnabled] = useState(enabled);
  const [localMin, setLocalMin] = useState(minPrice);
  const [localMax, setLocalMax] = useState(maxPrice);

  const handleToggle = () => {
    const nextState = !localEnabled;
    setLocalEnabled(nextState);
    if (nextState) {
      setIsOpen(true);
    } else {
      onChange(false, localMin, localMax);
    }
  };

  const handleSave = () => {
    onChange(localEnabled, localMin, localMax);
    setIsOpen(false);
  };

  return (
    <div className="smart-pricing-container" style={{ maxWidth: '375px', margin: '0 auto', fontFamily: 'system-ui' }}>
      <div className="toggle-row" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', padding: '16px', background: 'rgba(255,255,255,0.8)', backdropFilter: 'blur(10px)', borderRadius: '12px', border: '1px solid rgba(0,0,0,0.1)' }}>
        <div>
          <h3 style={{ margin: 0, fontSize: '16px', fontWeight: 600 }}>Enable AI Smart Pricing</h3>
          <p style={{ margin: '4px 0 0', fontSize: '12px', color: '#666' }}>Dynamically adjust prices to maximize yield</p>
        </div>
        <button
          onClick={handleToggle}
          style={{
            width: '50px',
            height: '30px',
            borderRadius: '15px',
            background: localEnabled ? '#34C759' : '#E5E5EA',
            border: 'none',
            position: 'relative',
            cursor: 'pointer',
            transition: 'background 0.3s'
          }}
        >
          <div style={{
            width: '26px',
            height: '26px',
            borderRadius: '50%',
            background: 'white',
            position: 'absolute',
            top: '2px',
            left: localEnabled ? '22px' : '2px',
            transition: 'left 0.3s',
            boxShadow: '0 2px 4px rgba(0,0,0,0.2)'
          }} />
        </button>
      </div>

      {isOpen && (
        <div style={{
          position: 'fixed',
          bottom: 0,
          left: 0,
          right: 0,
          background: 'rgba(255,255,255,0.95)',
          backdropFilter: 'blur(20px) saturate(180%)',
          borderTopLeftRadius: '24px',
          borderTopRightRadius: '24px',
          padding: '24px',
          boxShadow: '0 -4px 20px rgba(0,0,0,0.1)',
          zIndex: 1000,
          transition: 'transform 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275)'
        }}>
          <div style={{ width: '40px', height: '4px', background: '#CCC', borderRadius: '2px', margin: '0 auto 20px' }} />
          <h2 style={{ margin: '0 0 16px', fontSize: '20px', fontWeight: 600 }}>Smart Pricing Limits</h2>
          <p style={{ color: '#666', fontSize: '14px', marginBottom: '24px' }}>Set your floor and ceiling. The AI will never go outside this range.</p>

          <div style={{ display: 'flex', gap: '16px', marginBottom: '24px' }}>
            <div style={{ flex: 1 }}>
              <label style={{ display: 'block', fontSize: '12px', color: '#666', marginBottom: '8px' }}>Min Price ($)</label>
              <input
                type="number"
                value={localMin}
                onChange={e => setLocalMin(Number(e.target.value))}
                style={{ width: '100%', padding: '12px', borderRadius: '8px', border: '1px solid #DDD', fontSize: '16px', boxSizing: 'border-box' }}
              />
            </div>
            <div style={{ flex: 1 }}>
              <label style={{ display: 'block', fontSize: '12px', color: '#666', marginBottom: '8px' }}>Max Price ($)</label>
              <input
                type="number"
                value={localMax}
                onChange={e => setLocalMax(Number(e.target.value))}
                style={{ width: '100%', padding: '12px', borderRadius: '8px', border: '1px solid #DDD', fontSize: '16px', boxSizing: 'border-box' }}
              />
            </div>
          </div>

          <button
            onClick={handleSave}
            style={{ width: '100%', padding: '16px', background: '#007AFF', color: 'white', border: 'none', borderRadius: '12px', fontSize: '16px', fontWeight: 600, cursor: 'pointer' }}
          >
            Save & Enable
          </button>
        </div>
      )}
    </div>
  );
}
