'use client';
import React, { useState } from 'react';

export default function PydanticValidationPage() {
  const [toolName, setToolName] = useState('');
  const [payload, setPayload] = useState('{\n  \n}');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isRecoverable, setIsRecoverable] = useState(false);

  const handleValidate = async () => {
    setLoading(true);
    setError(null);
    setResult(null);
    setIsRecoverable(false);

    try {
      let parsedPayload;
      try {
        parsedPayload = JSON.parse(payload);
      } catch (e) {
        throw new Error('Invalid JSON format in payload');
      }

      const response = await fetch('/api/agents/pydantic', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          tool_name: toolName,
          arguments: parsedPayload
        }),
      });

      const data = await response.json();

      if (!response.ok) {
        if (data.is_recoverable) {
          setIsRecoverable(true);
        }
        throw new Error(data.error || 'Failed to validate payload');
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
      <h1 className="text-3xl font-bold mb-4">Pydantic-First Tool Schema Validation</h1>
      <p className="text-gray-600 mb-8">
        SOTA Harness Patterns (2025-2026): 6. Pydantic-first tool schema - validation errors fed back to LLM for self-correction.
        Test how the system validates tool payloads and generates recoverable errors.
      </p>

      <div className="space-y-6 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] p-8 shadow-[0_8px_30px_rgb(0,0,0,0.04)] border border-white/40 rounded-2xl">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Tool Name
          </label>
          <select
            className="w-full p-3 border border-gray-300 rounded-lg bg-gray-50 focus:bg-white focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] transition-all"
            value={toolName}
            onChange={(e) => setToolName(e.target.value)}
          >
            <option value="">Select a tool...</option>
            <option value="TopicRetrieve">TopicRetrieve</option>
            <option value="TranscriptSearch">TranscriptSearch</option>
            <option value="TopicWrite">TopicWrite</option>
            <option value="Bash">Bash</option>
          </select>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            JSON Payload
          </label>
          <textarea
            className="w-full p-4 border border-gray-300 rounded-xl shadow-[inset_0_2px_4px_rgba(0,0,0,0.02)] focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] transition-all font-mono text-sm bg-white/50"
            rows={8}
            value={payload}
            onChange={(e) => setPayload(e.target.value)}
            placeholder={`{\n  "topic_name": "architecture"\n}`}
          />
        </div>

        <button
          onClick={handleValidate}
          disabled={loading || !toolName || !payload.trim()}
          className="w-full py-3 bg-gradient-to-b from-[#0071E3] to-[#005bb5] text-white rounded-xl hover:from-[#005bb5] hover:to-[#004488] disabled:opacity-50 disabled:cursor-not-allowed font-medium transition-colors shadow-[0_2px_8px_rgba(0,113,227,0.3)]"
        >
          {loading ? 'Validating schema...' : 'Validate Tool Payload'}
        </button>
      </div>

      {error && (
        <div className={`mt-8 p-6 rounded-xl border shadow-sm ${isRecoverable ? 'bg-yellow-50 border-yellow-200 text-yellow-800' : 'bg-red-50 border-red-200 text-red-800'}`} data-testid="error-message">
          <div className="flex items-center gap-2 mb-3">
            {isRecoverable ? (
              <svg className="w-6 h-6 text-yellow-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
              </svg>
            ) : (
              <svg className="w-6 h-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            )}
            <h3 className="text-lg font-bold">
              {isRecoverable ? 'LLM-Recoverable Validation Error' : 'Validation Failed'}
            </h3>
          </div>
          <div className="bg-white/50 p-4 rounded-lg border border-white/20 font-mono text-sm whitespace-pre-wrap">
            {error}
          </div>
          {isRecoverable && (
            <p className="mt-4 text-sm font-medium">
              This error would be automatically fed back to the LLM for self-correction.
            </p>
          )}
        </div>
      )}

      {result && (
        <div className="mt-8 p-6 bg-green-50 border border-green-200 rounded-xl shadow-sm text-green-800" data-testid="success-message">
          <div className="flex items-center gap-2 mb-3">
            <svg className="w-6 h-6 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <h2 className="text-lg font-bold">Validation Successful</h2>
          </div>
          <p className="bg-white/50 p-4 rounded-lg border border-green-100 text-sm">
            {result}
          </p>
        </div>
      )}
    </div>
  );
}
