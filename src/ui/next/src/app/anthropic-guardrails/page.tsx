'use client';
import React, { useState } from 'react';

export default function AnthropicGuardrailsPage() {
  const [toolName, setToolName] = useState('execute_bash');
  const [projectTrusted, setProjectTrusted] = useState(false);
  const [sessionAllowedTools, setSessionAllowedTools] = useState('read_file, list_files');
  const [highRiskTools, setHighRiskTools] = useState('execute_bash, delete_database');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleValidate = async () => {
    setLoading(true);
    setError(null);
    setResult(null);

    try {
      const allowedTools = sessionAllowedTools.split(',').map(s => s.trim()).filter(Boolean);
      const riskTools = highRiskTools.split(',').map(s => s.trim()).filter(Boolean);

      const response = await fetch('/api/agents/guardrails/anthropic', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          toolName,
          projectTrusted,
          sessionAllowedTools: allowedTools,
          highRiskTools: riskTools
        }),
      });

      const data = await response.json();

      if (!response.ok) {
        throw new Error(data.error || 'Failed to validate tool');
      }

      setResult(data.result || 'Validation passed successfully');
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-8 max-w-4xl mx-auto font-sans">
      <h1 className="text-3xl font-bold mb-4">Anthropic 3-Stage Tool Gating</h1>
      <p className="text-gray-600 mb-8">
        SOTA Harness Patterns: Guardrails & Safety. Test the Anthropic Mechanic with 3 distinct stages:
        Trust establishment, Session permissions, and High-risk user confirmation.
      </p>

      <div className="space-y-6 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] p-6 rounded-[16px] shadow-sm border border-white/40">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Tool to Execute
          </label>
          <input
            type="text"
            className="w-full p-3 border border-gray-300 rounded-lg bg-gray-50 focus:ring-2 focus:ring-[#0066FF]"
            value={toolName}
            onChange={(e) => setToolName(e.target.value)}
          />
        </div>

        <div className="flex items-center gap-2">
          <input
            type="checkbox"
            id="trusted"
            checked={projectTrusted}
            onChange={(e) => setProjectTrusted(e.target.checked)}
            className="w-5 h-5"
          />
          <label htmlFor="trusted" className="text-sm font-medium text-gray-700">
            Project is Trusted (Stage 1)
          </label>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Session Allowed Tools (Comma separated - Stage 2)
          </label>
          <input
            type="text"
            className="w-full p-3 border border-gray-300 rounded-lg bg-gray-50"
            value={sessionAllowedTools}
            onChange={(e) => setSessionAllowedTools(e.target.value)}
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            High Risk Tools (Comma separated - Stage 3)
          </label>
          <input
            type="text"
            className="w-full p-3 border border-gray-300 rounded-lg bg-gray-50"
            value={highRiskTools}
            onChange={(e) => setHighRiskTools(e.target.value)}
          />
        </div>

        <button
          onClick={handleValidate}
          disabled={loading || !toolName.trim()}
          className="w-full py-3 bg-gradient-to-b from-[#0071E3] to-[#005bb5] text-white rounded-xl hover:from-[#005bb5] hover:to-[#004488] disabled:opacity-50 disabled:cursor-not-allowed font-medium transition-colors shadow-[0_2px_8px_rgba(0,113,227,0.3)]"
        >
          {loading ? 'Validating guardrails...' : 'Check Tool Guardrails'}
        </button>
      </div>

      {error && (
        <div className="mt-8 p-6 bg-red-50/70 backdrop-blur-[30px] saturate-[210%] text-red-700 rounded-xl border border-red-200 shadow-sm" data-testid="error-message">
          <h3 className="text-lg font-bold">Guardrail Tripped</h3>
          <div className="bg-white/50 p-4 rounded-lg font-mono text-sm whitespace-pre-wrap mt-2">
            {error}
          </div>
        </div>
      )}

      {result && (
        <div className="mt-8 p-6 glassmorphism bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 shadow-sm rounded-xl" data-testid="success-message">
          <h2 className="text-lg font-bold">Guardrails Passed</h2>
          <p className="bg-white/50 p-4 rounded-lg text-sm mt-2">{result}</p>
        </div>
      )}
    </div>
  );
}
