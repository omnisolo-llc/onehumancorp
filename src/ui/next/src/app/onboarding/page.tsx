'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function OnboardingPage() {
  const [description, setDescription] = useState('');
  const [status, setStatus] = useState<'idle' | 'loading' | 'success'>('idle');
  const [setup, setSetup] = useState<any>(null);
  const router = useRouter();

  const handleProcess = async () => {
    if (!description.trim()) return;
    setStatus('loading');

    // Simulate agentic call
    setTimeout(() => {
      setSetup({
        business_name: 'Custom Bakery',
        products: [
          { name: 'Custom Birthday Cake', price: 50 },
          { name: 'Wedding Cake Tier', price: 150 }
        ]
      });
      setStatus('success');
    }, 2000);
  };

  const handleApprove = async () => {
    setStatus('loading');

    // Attempt actual provision
    try {
      const res = await fetch('/api/growth/zero-click-generate', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ prompt: description })
      });
      if (res.ok) {
        router.push('/dashboard');
      } else {
        setStatus('idle');
      }
    } catch (e) {
      console.error(e);
      setStatus('idle');
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 p-4">
      <div className="glass-panel w-full max-w-md p-6 rounded-2xl shadow-xl flex flex-col space-y-6">
        <h1 className="text-2xl font-bold text-center">Tell me about your business...</h1>

        {status === 'idle' && (
          <>
            <textarea
              value={description}
              onChange={e => setDescription(e.target.value)}
              placeholder="I bake custom cakes in Austin and sell them on Instagram."
              className="w-full h-32 p-3 border rounded-lg resize-none"
              data-testid="business-description-input"
            />
            <button
              onClick={handleProcess}
              className="w-full h-[44px] bg-black text-white rounded-lg font-semibold min-h-[44px]"
              data-testid="process-button"
            >
              Analyze
            </button>
          </>
        )}

        {status === 'loading' && (
          <div className="flex flex-col items-center py-8">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-black mb-4"></div>
            <p>Agent is designing your workspace...</p>
          </div>
        )}

        {status === 'success' && setup && (
          <div className="flex flex-col space-y-4" data-testid="proposed-setup-card">
            <h2 className="text-xl font-semibold">Proposed Setup</h2>
            <div className="bg-white p-4 rounded-lg border">
              <p><strong>Business:</strong> {setup.business_name}</p>
              <ul className="mt-2 list-disc pl-5">
                {setup.products.map((p: any, i: number) => (
                  <li key={i}>{p.name} - ${p.price}</li>
                ))}
              </ul>
            </div>
            <button
              onClick={handleApprove}
              className="w-full h-[44px] bg-black text-white rounded-lg font-semibold min-h-[44px]"
              data-testid="approve-button"
            >
              Approve & Launch
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
