'use client';
import React, { useState } from 'react';

export default function GuardrailsPage() {
  const [projectTrusted, setProjectTrusted] = useState(false);
  const [allowedTools, setAllowedTools] = useState('read_file, execute_bash');
  const [highRiskTools, setHighRiskTools] = useState('execute_bash, delete_database');
  const [toolToRun, setToolToRun] = useState('execute_bash');

  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);

  const handleTestGuardrails = async () => {
    setLoading(true);
    setError(null);
    setResult(null);

    try {
      const response = await fetch('/api/guardrails', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          project_trusted: projectTrusted,
          allowed_tools: allowedTools.split(',').map(s => s.trim()).filter(Boolean),
          high_risk_tools: highRiskTools.split(',').map(s => s.trim()).filter(Boolean),
          tool_to_run: toolToRun,
        }),
      });

      if (!response.ok) {
        const err = await response.json();
        throw new Error(err.error || 'Failed to test guardrails');
      }

      const data = await response.json();
      setResult(data);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-8 max-w-4xl mx-auto font-sans">
      <h1 className="text-3xl font-bold mb-4">Guardrails & Safety</h1>
      <p className="text-gray-600 mb-8">
        Test Anthropic 3-Stage Tool Gating: Trust establishment at project load {'->'} Permission check before each tool call {'->'} Explicit user confirmation for high-risk operations.
      </p>

      <div className="bg-white p-6 rounded-lg shadow-sm border mb-8 space-y-6">
        <div>
          <label className="flex items-center space-x-3 cursor-pointer">
            <input
              type="checkbox"
              checked={projectTrusted}
              onChange={(e) => setProjectTrusted(e.target.checked)}
              className="h-5 w-5 text-blue-600 rounded border-gray-300 focus:ring-blue-500"
            />
            <span className="text-gray-700 font-medium">Project is Trusted (Stage 1)</span>
          </label>
          <p className="text-sm text-gray-500 mt-1 ml-8">If untrusted, only safe-for-untrusted tools are allowed.</p>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Session Allowed Tools (Stage 2)
          </label>
          <input
            type="text"
            value={allowedTools}
            onChange={(e) => setAllowedTools(e.target.value)}
            className="w-full p-2 border rounded shadow-sm focus:ring-blue-500 focus:border-blue-500"
            placeholder="Comma separated tools, e.g. read_file, execute_bash"
          />
          <p className="text-sm text-gray-500 mt-1">If set, tools must be in this list.</p>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            High-Risk Tools (Stage 3)
          </label>
          <input
            type="text"
            value={highRiskTools}
            onChange={(e) => setHighRiskTools(e.target.value)}
            className="w-full p-2 border rounded shadow-sm focus:ring-blue-500 focus:border-blue-500"
            placeholder="Comma separated tools, e.g. delete_database"
          />
          <p className="text-sm text-gray-500 mt-1">These tools require explicit user confirmation.</p>
        </div>

        <div className="pt-4 border-t">
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Tool to run
          </label>
          <div className="flex space-x-4">
            <input
              type="text"
              value={toolToRun}
              onChange={(e) => setToolToRun(e.target.value)}
              className="flex-1 p-2 border rounded shadow-sm focus:ring-blue-500 focus:border-blue-500"
              placeholder="e.g. execute_bash"
            />
            <button
              onClick={handleTestGuardrails}
              disabled={loading || !toolToRun}
              className="px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50 transition-colors"
            >
              {loading ? 'Testing...' : 'Test Guardrails'}
            </button>
          </div>
        </div>
      </div>

      {error && (
        <div className="p-4 bg-red-50 text-red-700 rounded-lg border border-red-200 mb-4" data-testid="error-message">
          <h3 className="font-bold mb-1">Execution Error:</h3>
          <p>{error}</p>
        </div>
      )}

      {result && (
        <div className={`p-4 rounded-lg border ${result.status === 'allowed' ? 'bg-green-50 border-green-200 text-green-800' : 'bg-yellow-50 border-yellow-200 text-yellow-800'}`} data-testid="success-message">
          <h3 className="font-bold mb-1 flex items-center">
            {result.status === 'allowed' ? '✅ Tool Allowed' : '⚠️ Guardrail Tripped'}
          </h3>
          <p className="font-mono text-sm mt-2">{result.message}</p>
        </div>
      )}
    </div>
  );
}
