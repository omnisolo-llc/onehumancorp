'use client';
import React, { useState } from 'react';

export default function ActorModelPage() {
  const [message, setMessage] = useState('');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleExecute = async () => {
    setLoading(true);
    setError(null);
    setResult(null);

    try {
      const response = await fetch('/api/agents/actor-model', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message }),
      });

      if (!response.ok) {
        const err = await response.json();
        throw new Error(err.error || 'Failed to execute Actor Model');
      }

      const data = await response.json();
      setResult(data.result);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-8 max-w-4xl mx-auto font-sans">
      <h1 className="text-3xl font-bold mb-4">Actor-Model Message Passing</h1>
      <p className="text-gray-600 mb-8">
        SOTA Harness Patterns (2025-2026): 1. Actor-model message passing - replacing classic ReAct loops.
        Communicate with the agent swarm using an Actor-Model design.
      </p>

      <div className="mb-6 space-y-4">
        <div>
          <label htmlFor="message" className="block text-sm font-medium text-gray-700 mb-2">
            Message to the Swarm
          </label>
          <textarea
            id="message"
            className="w-full p-4 border rounded-lg shadow-sm focus:ring-[#0066FF] focus:border-[#0066FF] glassmorphism bg-white backdrop-blur-[30px] saturate-[210%] border-white/40 bg-transparent"
            rows={4}
            value={message}
            onChange={(e) => setMessage(e.target.value)}
            placeholder="e.g. Ask the researcher to find latest trends, and have the summarizer condense it."
          />
        </div>
      </div>

      <button
        onClick={handleExecute}
        disabled={loading || !message}
        className="px-6 py-3 bg-[#0071E3] text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 font-medium transition-colors"
      >
        {loading ? 'Actors are executing...' : 'Send Message to Swarm'}
      </button>

      {error && (
        <div className="mt-8 p-4 bg-red-50/70 backdrop-blur-[30px] saturate-[210%] text-red-700 rounded-lg border border-red-200 shadow-sm" data-testid="error-message">
          <h3 className="font-bold mb-2">Execution Error:</h3>
          <p>{error}</p>
        </div>
      )}

      {result && (
        <div className="mt-8 p-6 glassmorphism bg-white backdrop-blur-[30px] saturate-[210%] border border-white/40 shadow-sm" data-testid="success-message">
          <h2 className="text-xl font-bold mb-4 border-b pb-2">Swarm Result</h2>
          <pre className="whitespace-pre-wrap text-sm text-gray-800 font-mono bg-gray-50 p-4 rounded">
            {result}
          </pre>
        </div>
      )}
    </div>
  );
}
