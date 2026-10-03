'use client';
import React, { useEffect, useState } from 'react';

type Pattern = { id: string; initial_context: string; outcome_score: number; successful_tools: string[] };

async function runtimeJson(response: Response): Promise<unknown> {
  const payload: unknown = await response.json();
  const error = payload && typeof payload === 'object' && 'error' in payload ? payload.error : undefined;
  if (!response.ok || error != null) {
    throw new Error(typeof error === 'string' && error.trim() ? error : 'Pattern runtime request failed');
  }
  return payload;
}

async function readPatterns(): Promise<Pattern[]> {
  const payload = await runtimeJson(await fetch('/api/v1/sona'));
  const patterns = payload && typeof payload === 'object' && 'patterns' in payload ? payload.patterns : undefined;
  if (!Array.isArray(patterns) || patterns.some(pattern => !pattern || typeof pattern !== 'object'
    || typeof pattern.id !== 'string' || !pattern.id || typeof pattern.initial_context !== 'string'
    || typeof pattern.outcome_score !== 'number' || !Number.isFinite(pattern.outcome_score)
    || !Array.isArray(pattern.successful_tools) || pattern.successful_tools.some((tool: unknown) => typeof tool !== 'string'))) {
    throw new Error('Pattern runtime returned an invalid response');
  }
  return patterns;
}

export default function SonaPatternsPage() {
  const [patterns, setPatterns] = useState<Pattern[]>([]);
  const [loading, setLoading] = useState(true);
  const [recording, setRecording] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [newTaskContext, setNewTaskContext] = useState('');
  const [newTool, setNewTool] = useState('');

  useEffect(() => {
    let current = true;
    readPatterns()
      .then(value => { if (current) setPatterns(value); })
      .catch(error => { if (current) setError(error instanceof Error ? error.message : 'Could not load patterns'); })
      .finally(() => { if (current) setLoading(false); });
    return () => { current = false; };
  }, []);

  const recordPattern = async () => {
    if (recording || !newTaskContext.trim() || !newTool.trim()) return;
    const pattern = {
      id: crypto.randomUUID(), initial_context: newTaskContext,
      successful_tools: [newTool], outcome_score: 1.0, created_at: new Date().toISOString(),
    };
    setRecording(true);
    setError(null);
    try {
      await runtimeJson(await fetch('/api/v1/sona', {
        method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(pattern),
      }));
      const recorded = await readPatterns();
      if (!recorded.some(value => value.id === pattern.id && value.initial_context === pattern.initial_context
        && value.outcome_score === pattern.outcome_score && value.successful_tools.length === 1
        && value.successful_tools[0] === newTool)) {
        throw new Error('Pattern recording was not confirmed by the runtime');
      }
      setPatterns(recorded);
      setNewTaskContext('');
      setNewTool('');
    } catch (error) {
      setError(error instanceof Error ? error.message : 'Pattern recording could not be confirmed');
    } finally { setRecording(false); }
  };

  return (
    <div className="p-8 max-w-5xl mx-auto font-sans">
      <h1 className="text-3xl font-bold mb-4">SONA Neural Patterns Dashboard</h1>
      <p className="text-gray-600 mb-8">
        This dashboard visualizes the Self-Learning Trajectory Patterns (SONA) recorded by the Ruflo Agent Harness. These patterns allow the agent to memorize successful tool execution trajectories for similar future tasks.
      </p>

      {loading ? (
        <div className="text-gray-500">Loading patterns...</div>
      ) : error ? (
        <div role="alert" className="text-[#FF3B30] bg-red-50 p-4 rounded-lg">{error}</div>
      ) : patterns.length === 0 ? (
        <div className="text-gray-500">No patterns recorded yet.</div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {patterns.map((p) => (
            <div key={p.id} className="p-6 bg-white border border-gray-200 rounded-lg shadow-sm">
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-xl font-semibold truncate pr-4">{p.initial_context}</h2>
                <span className={`px-2 py-1 text-xs font-bold rounded ${p.outcome_score > 0.8 ? 'bg-green-100 text-green-800' : 'bg-yellow-100 text-yellow-800'}`}>
                  Score: {p.outcome_score.toFixed(2)}
                </span>
              </div>
              <div className="mb-2">
                <span className="text-sm font-medium text-gray-500">Successful Tools Trajectory:</span>
                <div className="flex flex-wrap gap-2 mt-2">
                  {p.successful_tools.map((tool: string, idx: number) => (
                    <span key={idx} className="px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm border border-gray-300">
                      {idx + 1}. {tool}
                    </span>
                  ))}
                </div>
              </div>
            </div>
          ))}
        </div>
      )}

      <div className="mt-8 p-6 bg-gray-50 border rounded-lg">
        <h2 className="text-xl font-bold mb-4">Record New Trajectory Pattern</h2>
        <div className="flex flex-col gap-4">
          <input
            className="p-2 border rounded"
            placeholder="Task Context (e.g. Fix null pointer)"
            disabled={recording}
            value={newTaskContext}
            onChange={(e) => setNewTaskContext(e.target.value)}
          />
          <input
            className="p-2 border rounded"
            placeholder="Tool used (e.g. edit_file)"
            disabled={recording}
            value={newTool}
            onChange={(e) => setNewTool(e.target.value)}
          />
          <button
            onClick={recordPattern}
            className="bg-[#0071E3] text-white p-2 rounded w-fit"
            disabled={loading || recording || !newTaskContext.trim() || !newTool.trim()}
          >
            {recording ? 'Recording...' : 'Record Pattern'}
          </button>
        </div>
      </div>

    </div>
  );
}
