'use client';
import React, { useState } from 'react';

export default function LangGraphPage() {
  const [message, setMessage] = useState('');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleExecute = async () => {
    setLoading(true);
    setError(null);
    setResult(null);

    try {
      const response = await fetch('/api/v1/agents/langgraph', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message }),
      });

      if (!response.ok) {
        const err = await response.json();
        throw new Error(err.error || 'Failed to execute LangGraph state machine');
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
      <h1 className="text-3xl font-bold mb-4">LangGraph State Machine</h1>
      <p className="text-gray-600 mb-8">
        LangChain/LangGraph: Models the harness as an explicit state graph. Mechanically: uses llm_call and tool_node connected by conditional edges (if tool calls present {'->'} route to tool_node; if absent {'->'} route to END). State flows as typed dictionaries with reducer functions.
      </p>

      <div className="mb-6 space-y-4">
        <div>
          <label htmlFor="message" className="block text-sm font-medium text-gray-700 mb-2">
            Message
          </label>
          <textarea
            id="message"
            className="w-full p-4 border rounded-lg shadow-sm focus:ring-[#0066FF] focus:border-[#0066FF]"
            rows={4}
            value={message}
            onChange={(e) => setMessage(e.target.value)}
            placeholder="e.g. Write a quick poem about a cake."
          />
        </div>
      </div>

      <button
        onClick={handleExecute}
        disabled={loading || !message}
        className="px-6 py-3 bg-[#0071E3] text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 font-medium transition-colors"
      >
        {loading ? 'Executing State Graph...' : 'Run LangGraph'}
      </button>

      {error && (
        <div className="mt-8 p-4 bg-red-50 text-red-700 rounded-lg border border-red-200" data-testid="error-message">
          <h3 className="font-bold mb-2">Execution Error:</h3>
          <p>{error}</p>
        </div>
      )}

      {result && (
        <div className="mt-8 p-6 bg-white border rounded-lg shadow-sm" data-testid="success-message">
          <h2 className="text-xl font-bold mb-4 border-b pb-2">LangGraph Output</h2>
          <pre className="whitespace-pre-wrap text-sm text-gray-800 font-mono bg-gray-50 p-4 rounded">
            {result}
          </pre>
        </div>
      )}
    </div>
  );
}
