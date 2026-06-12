'use client';

import React, { useState } from 'react';

export default function AgenticSeekPage() {
  const [task, setTask] = useState('');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleExecute = async () => {
    if (!task) return;
    setLoading(true);
    setError(null);
    setResult(null);

    try {
      const response = await fetch('/api/agents/agentic-seek', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ task }),
      });

      if (!response.ok) {
        const err = await response.json();
        throw new Error(err.error || 'Failed to execute local agent task');
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
    <div className="max-w-4xl mx-auto p-8 font-sans bg-gray-50 min-h-screen">
      <h1 className="text-3xl font-bold mb-4 text-gray-900">AgenticSeek Local Agent</h1>
      <p className="text-gray-600 mb-8 text-sm">
        Run tasks purely on local compute with no API costs using the AgenticSeek provider.
      </p>

      <div className="bg-white shadow p-6 rounded-lg border border-gray-200 mb-8">
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Local Task Description
        </label>
        <textarea
          className="w-full p-4 border rounded-lg shadow-sm focus:ring-blue-500 focus:border-blue-500 mb-4 bg-gray-50"
          rows={4}
          value={task}
          onChange={(e) => setTask(e.target.value)}
          placeholder="e.g. Analyze the local log files and summarize errors..."
        />
        <button
          onClick={handleExecute}
          disabled={loading || !task}
          className="px-6 py-2 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 disabled:opacity-50 transition-colors shadow-sm"
        >
          {loading ? 'Running Locally...' : 'Execute Local Task'}
        </button>
      </div>

      {error && (
        <div className="mb-8 p-4 bg-red-50 text-red-700 rounded-lg border border-red-200">
          <h3 className="font-bold mb-2">Execution Error:</h3>
          <p>{error}</p>
        </div>
      )}

      {result && (
        <div className="bg-white border border-gray-200 rounded-lg p-6 shadow-sm">
          <h2 className="text-xl font-bold mb-4 text-gray-900">Local Execution Result</h2>
          <pre className="whitespace-pre-wrap text-sm text-gray-800 font-mono overflow-auto max-h-[500px] bg-gray-50 p-4 rounded-lg border border-gray-100">
            {result}
          </pre>
        </div>
      )}
    </div>
  );
}
