import React, { useState, useEffect } from 'react';

const Toast = ({ message, onClose }: { message: string, onClose: () => void }) => {
  useEffect(() => {
    const timer = setTimeout(onClose, 3000);
    return () => clearTimeout(timer);
  }, [onClose]);

  return (
    <div style={{
      position: 'fixed', bottom: '20px', right: '20px',
      background: 'rgba(0, 255, 0, 0.2)', backdropFilter: 'blur(10px)',
      border: '1px solid rgba(0, 255, 0, 0.5)', borderRadius: '8px',
      padding: '12px 24px', color: '#fff', zIndex: 1000,
      animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)'
    }}>
      {message}
    </div>
  );
};

const PromptTuningWizard = () => {
  const [step, setStep] = useState(1);
  const [personality, setPersonality] = useState('Friendly');
  const [focus, setFocus] = useState<string[]>([]);
  const [examples, setExamples] = useState([{ q: '', a: '' }]);
  const [isLoading, setIsLoading] = useState(false);
  const [expertMode, setExpertMode] = useState(false);
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  // Chat sandbox state
  const [chatInput, setChatInput] = useState('');
  const [chatLog, setChatLog] = useState<{role: string, content: string}[]>([]);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const saved = localStorage.getItem('expertMode');
      if (saved === 'true') setExpertMode(true);
    }
  }, []);

  const nextStep = () => setStep(step + 1);
  const prevStep = () => setStep(step - 1);

  const toggleFocus = (tag: string) => {
    if (focus.includes(tag)) setFocus(focus.filter(t => t !== tag));
    else setFocus([...focus, tag]);
  };

  const addExample = () => {
    if (examples.length < 3) setExamples([...examples, { q: '', a: '' }]);
  };

  const updateExample = (index: number, field: 'q'|'a', value: string) => {
    const newEx = [...examples];
    newEx[index][field] = value;
    setExamples(newEx);
  };

  const handleExpertModeToggle = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const checked = e.target.checked;
    setExpertMode(checked);
    if (typeof window !== 'undefined') {
      localStorage.setItem('expertMode', checked.toString());
    }
    // Mock syncing to user profile via API
    try {
      await fetch('/api/user/profile', {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ expertMode: checked })
      });
    } catch (err) {
      console.warn("API mock error", err);
    }
  };

  const save = async () => {
    setIsLoading(true);
    try {
      const response = await fetch('/api/agents/tune', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ personality, focus, examples })
      });
      if (response.ok) {
        setToastMessage('Your agent has been updated ✓');
      } else {
        setToastMessage('Failed to save');
      }
    } catch (e) {
      setToastMessage('Error saving');
    }
    setIsLoading(false);
  };

  const handleChatSubmit = () => {
    if (!chatInput.trim()) return;
    setChatLog([...chatLog, { role: 'user', content: chatInput }]);
    const currentInput = chatInput;
    setChatInput('');

    // Simple mock response based on examples or personality
    setTimeout(() => {
      const matchedExample = examples.find(ex => ex.q.toLowerCase() === currentInput.toLowerCase());
      const reply = matchedExample ? matchedExample.a : `[${personality} reply]: I understand you said "${currentInput}". Let's discuss further.`;
      setChatLog(prev => [...prev, { role: 'agent', content: reply }]);
    }, 500);
  };

  const generatedPrompt = `You are a helpful AI assistant. Your personality is: ${personality}.\n` +
    (focus.length > 0 ? `Please focus on the following domains: ${focus.join(', ')}.\n` : '') +
    (examples.length > 0 && examples[0].q ? `Here are some examples of how to respond:\n${examples.map(e => `Q: ${e.q}\nA: ${e.a}`).join('\n\n')}` : '');

  return (
    <div style={{ backdropFilter: 'blur(20px) saturate(200%)', background: 'rgba(255, 255, 255, 0.03)', borderRadius: '12px', padding: '30px', color: '#fff', fontFamily: 'Inter, sans-serif', boxShadow: '0 0 20px rgba(255,255,255,0.1)' }}>
      {toastMessage && <Toast message={toastMessage} onClose={() => setToastMessage(null)} />}

      {step === 1 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ fontFamily: 'Outfit, sans-serif' }}>Personality & Tone</h2>
          {['Formal', 'Friendly', 'Concise', 'Detailed', 'Custom'].map(t => (
            <label key={t} style={{ display: 'block' }}>
              <input type="radio" name="tone" checked={personality === t} onChange={() => setPersonality(t)} /> {t}
            </label>
          ))}
          <button onClick={nextStep}>Next</button>
        </div>
      )}
      {step === 2 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ fontFamily: 'Outfit, sans-serif' }}>Domain Focus</h2>
          {['Only discuss topics related to my business', 'Avoid competitor mentions', 'Always reply in Spanish'].map(tag => (
            <label key={tag} style={{ display: 'block' }}>
              <input type="checkbox" checked={focus.includes(tag)} onChange={() => toggleFocus(tag)} /> {tag}
            </label>
          ))}
          <button onClick={prevStep}>Back</button>
          <button onClick={nextStep}>Next</button>
        </div>
      )}
      {step === 3 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <h2 style={{ fontFamily: 'Outfit, sans-serif' }}>Example Interactions</h2>
          {examples.map((ex, i) => (
            <div key={i} style={{ marginBottom: '10px' }}>
              <input placeholder="User question" value={ex.q} onChange={e => updateExample(i, 'q', e.target.value)} /><br/>
              <input placeholder="Agent response" value={ex.a} onChange={e => updateExample(i, 'a', e.target.value)} />
            </div>
          ))}
          {examples.length < 3 && <button onClick={addExample}>Add another example</button>}
          <br/>
          <button onClick={prevStep}>Back</button>
          <button onClick={nextStep}>Next</button>
        </div>
      )}
      {step === 4 && (
        <div style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
          <div style={{ display: 'flex', gap: '20px', flexWrap: 'wrap' }}>
            <div style={{ flex: '1 1 45%', minWidth: '300px' }}>
              <h2 style={{ fontFamily: 'Outfit, sans-serif' }}>Generated Prompt</h2>
              <pre style={{ whiteSpace: 'pre-wrap', fontFamily: 'monospace', background: 'rgba(0,0,0,0.2)', padding: '10px', borderRadius: '8px' }}>
                {expertMode ? generatedPrompt : 'Prompt preview is hidden in standard mode.'}
              </pre>
            </div>
            <div style={{ flex: '1 1 45%', minWidth: '300px' }}>
              <h2 style={{ fontFamily: 'Outfit, sans-serif' }}>Live Sandbox</h2>
              <div style={{ background: '#000', padding: '10px', height: '200px', display: 'flex', flexDirection: 'column', borderRadius: '8px' }}>
                <div style={{ flex: 1, overflowY: 'auto', marginBottom: '10px' }}>
                  {chatLog.map((msg, i) => (
                    <div key={i} style={{ textAlign: msg.role === 'user' ? 'right' : 'left', marginBottom: '5px' }}>
                      <span style={{ background: msg.role === 'user' ? '#007BFF' : '#444', padding: '5px 10px', borderRadius: '12px', display: 'inline-block' }}>
                        {msg.content}
                      </span>
                    </div>
                  ))}
                </div>
                <div style={{ display: 'flex' }}>
                  <input
                    type="text"
                    value={chatInput}
                    onChange={e => setChatInput(e.target.value)}
                    onKeyDown={e => e.key === 'Enter' && handleChatSubmit()}
                    placeholder="Test your agent..."
                    style={{ flex: 1, padding: '5px', color: '#000' }}
                  />
                  <button onClick={handleChatSubmit} style={{ marginLeft: '5px' }}>Send</button>
                </div>
              </div>
            </div>
          </div>
          <div style={{ marginTop: '20px' }}>
            <button onClick={prevStep}>Back</button>
            <button onClick={save} disabled={isLoading}>{isLoading ? 'Saving...' : 'Save Agent ✓'}</button>
          </div>
        </div>
      )}
      <div style={{ marginTop: '20px' }}>
        <label>
          <input type="checkbox" checked={expertMode} onChange={handleExpertModeToggle} />
          Expert Mode
        </label>
      </div>
    </div>
  );
};
export default PromptTuningWizard;
