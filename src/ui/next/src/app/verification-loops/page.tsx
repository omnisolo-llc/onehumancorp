'use client';

import React, { useState } from 'react';

export default function VerificationLoopsPage() {
  const [taskContext, setTaskContext] = useState('');
  const [outputText, setOutputText] = useState('');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleVerify = async (verificationType: 'computational' | 'visual' | 'inferential') => {
    setLoading(true);
    setError(null);
    setResult(null);

    try {
      const response = await fetch('/api/verification-loops', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          task_context: taskContext,
          output_text: outputText,
          verification_type: verificationType
        }),
      });

      if (!response.ok) {
        const err = await response.json();
        throw new Error(err.error || `Failed to run ${verificationType} verification`);
      }

      const data = await response.json();
      setResult(data.result?.message || 'Verification passed successfully.');
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-6xl mx-auto p-8 font-sans">
      <h1 className="text-3xl font-bold mb-4 text-gray-900">Verification Loops</h1>
      <p className="text-gray-600 mb-8">
        Test agent output against distinct verification loops: Computational Guides (bash/linters), Visual Verifiers (Playwright), or Inferential Sensors (LLM Judge).
      </p>

      <div className="glassmorphism bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 p-6 shadow-sm rounded-2xl space-y-6">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Task Context
          </label>
          <textarea
            className="w-full p-4 border border-gray-300 rounded-xl shadow-sm focus:ring-[#0066FF] focus:border-[#0066FF] transition-colors bg-white/80 backdrop-blur-[30px] saturate-[210%]"
            rows={4}
            value={taskContext}
            onChange={(e) => setTaskContext(e.target.value)}
            placeholder="e.g. Write a bash script that echoes 'ok'."
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Agent Output / Command / Path
          </label>
          <textarea
            className="w-full p-4 border border-gray-300 rounded-xl shadow-sm focus:ring-[#0066FF] focus:border-[#0066FF] transition-colors bg-white/80 backdrop-blur-[30px] saturate-[210%] font-mono text-sm"
            rows={4}
            value={outputText}
            onChange={(e) => setOutputText(e.target.value)}
            placeholder="e.g. echo 'ok'; e\x78it 0"
          />
        </div>

        <div className="flex flex-wrap gap-4 pt-4 border-t">
          <button
            onClick={() => handleVerify('computational')}
            disabled={loading || !outputText}
            className="px-6 py-2.5 bg-[#0071E3] text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 font-medium transition-colors shadow-sm"
          >
            {loading ? 'Verifying...' : 'Run Computational Guide'}
          </button>

          <button
            onClick={() => handleVerify('visual')}
            disabled={loading || !outputText}
            className="px-6 py-2.5 bg-purple-600 text-white rounded-lg hover:bg-purple-700 disabled:opacity-50 font-medium transition-colors shadow-sm"
          >
            {loading ? 'Verifying...' : 'Run Visual Verifier'}
          </button>

          <button
            onClick={() => handleVerify('inferential')}
            disabled={loading || !outputText}
            className="px-6 py-2.5 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 disabled:opacity-50 font-medium transition-colors shadow-sm"
          >
            {loading ? 'Verifying...' : 'Run Inferential Sensor'}
          </button>
        </div>
      </div>

      {error && (
        <div className="mt-8 p-4 bg-red-50/80 backdrop-blur-[30px] saturate-[210%] text-red-700 rounded-xl border border-red-200 shadow-sm">
          <h3 className="font-bold mb-2 flex items-center">
            <svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
            Verification Failed
          </h3>
          <p className="whitespace-pre-wrap font-mono text-sm">{error}</p>
        </div>
      )}

      {result && (
        <div className="mt-8 p-6 bg-green-50/80 backdrop-blur-[30px] saturate-[210%] text-green-800 border border-green-200 rounded-xl shadow-sm">
          <h3 className="font-bold mb-2 flex items-center text-green-900">
            <svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
            Verification Passed
          </h3>
          <p>{result}</p>
        </div>
      )}
    </div>
  );
}
