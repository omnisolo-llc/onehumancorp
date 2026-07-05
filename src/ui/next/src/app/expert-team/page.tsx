'use client';
import React, { useState } from 'react';

export default function ExpertTeamPage() {
  const [task, setTask] = useState('');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleExecute = async () => {
    setLoading(true);
    setError(null);
    setResult(null);

    try {
      const response = await fetch('/api/expert-team', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ task }),
      });

      if (!response.ok) {
        const err = await response.json();
        throw new Error(err.error || 'Failed to execute expert team');
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
    <div className="max-w-6xl mx-auto p-8 font-sans">
      <h1 className="text-3xl font-bold mb-4">Collaborative Expert Team</h1>
      <p className="text-gray-600 mb-8">
        Enter a complex task. The Lead Agent will coordinate 5 domain experts (Industry Researcher, Financial Analyst, Strategic Analyst, Process Supervisor, Quality Auditor) to execute it in parallel, strictly passing through code-enforced quality gates (Pre-flight, Pre-merge, Pre-deliver). (Tencent Workbuddy (Expert Team) Feature)
      </p>

      <div className="glassmorphism bg-white backdrop-blur-[30px] saturate-[210%] border border-white/40 p-6 shadow-sm rounded-2xl mb-6">
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Business Task Context
        </label>
        <textarea
          className="w-full p-4 border border-gray-300 rounded-xl shadow-sm focus:ring-[#0066FF] focus:border-[#0066FF] bg-white/80 backdrop-blur-[30px] saturate-[210%]"
          rows={5}
          value={task}
          onChange={(e) => setTask(e.target.value)}
          placeholder="e.g. Write a comprehensive business plan for a new vegan bakery... Chart: Required. Analysis: Deep."
        />
      </div>

      <button
        onClick={handleExecute}
        disabled={loading || !task}
        className="px-6 py-3 bg-[#0071E3] text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 font-medium shadow-sm transition-colors"
      >
        {loading ? 'Orchestrating Expert Team...' : 'Execute Task via Expert Team'}
      </button>

      {error && (
        <div className="mt-8 p-4 bg-red-50/80 backdrop-blur-[30px] saturate-[210%] text-red-700 rounded-xl border border-red-200 shadow-sm expert-error-content">
          <h3 className="font-bold mb-2">Quality Gate or Execution Error:</h3>
          <p>{error}</p>
        </div>
      )}

      {result && (
        <div className="glassmorphism mt-8 p-6 bg-white backdrop-blur-[30px] saturate-[210%] border border-white/40 shadow-sm rounded-2xl expert-output-content">
          <h2 className="text-xl font-bold mb-4 border-b pb-2">Final Delivered Output</h2>
          <pre className="whitespace-pre-wrap text-sm text-gray-800 font-mono overflow-auto max-h-[600px]">
            {result}
          </pre>
        </div>
      )}
    </div>
  );
}
