'use client';
import React, { useState } from 'react';

export default function CodeNativeExecutionPage() {
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleExecute = async () => {
    setLoading(true);
    setError(null);
    setResult(null);

    try {
      const response = await fetch('/api/agents/code-native', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({}),
      });

      const data = await response.json();

      if (!response.ok) {
        throw new Error(data.error || 'Failed to execute code-native pipeline');
      }

      setResult(JSON.stringify(data.results, null, 2));
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-8 max-w-4xl mx-auto font-sans">
      <h1 className="text-3xl font-bold mb-4">Code-Native Execution Pipeline</h1>
      <p className="text-gray-600 mb-8">
        SOTA Harness Patterns (2025-2026): 2. Code-native execution - preserving execution state and rich data structures.
      </p>

      <div className="space-y-6 bg-white p-8 shadow rounded-2xl">
        <button
          onClick={handleExecute}
          disabled={loading}
          className="w-full py-3 bg-blue-600 text-white rounded-xl hover:bg-blue-700 disabled:opacity-50"
        >
          {loading ? 'Executing pipeline...' : 'Run Pipeline'}
        </button>
      </div>

      {error && (
        <div className="mt-8 p-6 bg-red-50 text-red-800 rounded-xl" data-testid="error-message">
          {error}
        </div>
      )}

      {result && (
        <div className="mt-8 p-6 bg-green-50 text-green-800 rounded-xl" data-testid="success-message">
          <pre className="whitespace-pre-wrap font-mono text-sm">{result}</pre>
        </div>
      )}
    </div>
  );
}
