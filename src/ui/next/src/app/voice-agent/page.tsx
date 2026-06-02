'use client';
import { useState, useEffect } from 'react';

interface VoiceAgentConfig {
  tenant_id: string;
  phone_number: string;
  is_enabled: boolean;
  primary_language: string;
  custom_instructions: string;
}

export default function VoiceAgentPage() {
  const [config, setConfig] = useState<VoiceAgentConfig | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [saveMessage, setSaveMessage] = useState('');

  useEffect(() => {
    const fetchConfig = async () => {
      try {
        const response = await fetch('/api/settings/voice-agent');
        if (response.ok) {
          const data = await response.json();
          setConfig(data);
        }
      } catch (error) {
        console.error('Error fetching Voice Agent Config', error);
      } finally {
        setLoading(false);
      }
    };

    fetchConfig();
  }, []);

  const handleSave = async () => {
    if (!config) return;
    setSaving(true);
    try {
      const response = await fetch('/api/settings/voice-agent', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(config),
      });
      if (response.ok) {
        setSaveMessage('Voice settings saved successfully.');
        setTimeout(() => setSaveMessage(''), 3000);
      }
    } catch (error) {
      console.error('Error saving Voice Agent Config', error);
    } finally {
      setSaving(false);
    }
  };

  if (loading) return <div>Loading...</div>;

  return (
    <div style={{ maxWidth: '600px', margin: '0 auto', padding: '20px' }}>
      <h1>Voice AI Receptionist</h1>

      <div className="card glass" style={{ padding: '20px', borderRadius: '16px', backdropFilter: 'blur(20px) saturate(200%)', backgroundColor: 'rgba(255, 255, 255, 0.1)', border: '1px solid rgba(255, 255, 255, 0.2)' }}>
        <h2 style={{ fontSize: '18px', marginBottom: '16px' }}>Settings</h2>

        <div style={{ marginBottom: '16px' }}>
          <label style={{ display: 'block', fontWeight: 'bold', marginBottom: '4px' }}>Your Business Phone Number</label>
          <div style={{ padding: '8px 12px', background: 'rgba(0,0,0,0.05)', borderRadius: '8px' }}>
            {config?.phone_number || '(Not Assigned)'}
          </div>
        </div>

        <div style={{ marginBottom: '16px', display: 'flex', alignItems: 'center' }}>
          <label style={{ fontWeight: 'bold', marginRight: '16px' }}>AI Receptionist (On/Off)</label>
          <input
            type="checkbox"
            checked={config?.is_enabled || false}
            onChange={(e) => setConfig(config ? { ...config, is_enabled: e.target.checked } : null)}
            style={{ width: '20px', height: '20px' }}
          />
        </div>

        <div style={{ marginBottom: '16px' }}>
          <label style={{ display: 'block', fontWeight: 'bold', marginBottom: '4px' }}>Primary Language</label>
          <select
            value={config?.primary_language || 'English'}
            onChange={(e) => setConfig(config ? { ...config, primary_language: e.target.value } : null)}
            style={{ width: '100%', padding: '10px', borderRadius: '8px', border: '1px solid rgba(0,0,0,0.1)' }}
          >
            <option value="English">English</option>
            <option value="Arabic">Arabic</option>
            <option value="Spanish">Spanish</option>
          </select>
        </div>

        <div style={{ marginBottom: '16px' }}>
          <label style={{ display: 'block', fontWeight: 'bold', marginBottom: '4px' }}>What should the agent know?</label>
          <textarea
            value={config?.custom_instructions || ''}
            onChange={(e) => setConfig(config ? { ...config, custom_instructions: e.target.value } : null)}
            placeholder="e.g., 'Tell callers to park in the back'"
            style={{ width: '100%', padding: '10px', borderRadius: '8px', border: '1px solid rgba(0,0,0,0.1)', minHeight: '100px' }}
          />
        </div>

        <button
          onClick={handleSave}
          disabled={saving}
          style={{ width: '100%', padding: '12px', background: 'black', color: 'white', border: 'none', borderRadius: '8px', fontWeight: 'bold', cursor: 'pointer' }}
        >
          {saving ? 'Saving...' : 'Save Settings'}
        </button>
        {saveMessage && <div style={{ marginTop: '12px', color: 'green', textAlign: 'center' }}>{saveMessage}</div>}
      </div>

      {/* Call History Section */}
      <div style={{ marginTop: '40px' }}>
        <h2>Recent Calls & Transcripts</h2>

        <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
          {[
            { id: 1, name: 'Maya Baker', duration: '3m 12s', summary: 'Asked about vegan cakes. Advised on menu.', transcript: 'AI: Hello! How can I help you?\nUser: Do you have vegan cakes?\nAI: Yes, we do! They need a 48-hour advance notice.' },
            { id: 2, name: 'Carlos (Handyman)', duration: '1m 45s', summary: 'Booked plumbing estimate for Tuesday.', transcript: 'User: My sink is leaking.\nAI: I can book an estimate. How about Tuesday?' },
          ].map(call => (
            <details key={call.id} className="card glass" style={{ padding: '16px', borderRadius: '12px', background: 'rgba(255, 255, 255, 0.4)' }}>
              <summary style={{ cursor: 'pointer', fontWeight: 'bold', display: 'flex', justifyContent: 'space-between', listStyle: 'none' }}>
                <span>{call.name}</span>
                <span style={{ color: '#666', fontSize: '14px' }}>{call.duration}</span>
              </summary>
              <div style={{ marginTop: '12px', paddingTop: '12px', borderTop: '1px solid rgba(0,0,0,0.1)' }}>
                <p style={{ fontStyle: 'italic', marginBottom: '8px' }}>{call.summary}</p>
                <div style={{ background: 'rgba(0,0,0,0.03)', padding: '12px', borderRadius: '8px', fontSize: '14px', whiteSpace: 'pre-wrap', fontFamily: 'monospace' }}>
                  {call.transcript}
                </div>
              </div>
            </details>
          ))}
        </div>
      </div>

    </div>
  );
}
