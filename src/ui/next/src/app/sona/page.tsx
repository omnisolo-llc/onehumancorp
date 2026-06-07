'use client';
import React, { useEffect, useState } from 'react';

export default function SonaPatternsPage() {
  const [patterns, setPatterns] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [newTaskContext, setNewTaskContext] = useState('');
  const [newTool, setNewTool] = useState('');

  useEffect(() => {
    fetch('/api/sona')
      .then(r => r.json())
      .then(d => {
        if (d.error) {
          setError(d.error);
        } else if (d.patterns) {
          setPatterns(d.patterns);
        }
        setLoading(false);
      })
      .catch((e) => { setError(e.message); setLoading(false); });
  }, []);

  return (
    <div className="p-8 max-w-5xl mx-auto font-sans">
      <h1 className="text-3xl font-bold mb-4">SONA Neural Patterns Dashboard</h1>
      <p className="text-gray-600 mb-8">
        This dashboard visualizes the Self-Learning Trajectory Patterns (SONA) recorded by the Ruflo Agent Harness. These patterns allow the agent to memorize successful tool execution trajectories for similar future tasks.
      </p>

      {loading ? (
        <div className="text-gray-500">Loading patterns...</div>
      ) : patterns.length === 0 ? (
        <div className="text-gray-500">No patterns recorded yet.</div>
      ) : error ? (
        <div className="text-red-500 bg-red-50 p-4 rounded-lg">{error}</div>
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
            value={newTaskContext}
            onChange={(e) => setNewTaskContext(e.target.value)}
          />
          <input
            className="p-2 border rounded"
            placeholder="Tool used (e.g. edit_file)"
            value={newTool}
            onChange={(e) => setNewTool(e.target.value)}
          />
          <button
            onClick={() => {
              fetch('/api/sona', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  id: Date.now().toString(),
                  initial_context: newTaskContext,
                  successful_tools: [newTool],
                  outcome_score: 1.0
                })
              }).then(() => { fetch('/api/sona').then(r => r.json()).then(d => { if (d.patterns) { setPatterns(d.patterns); } setNewTaskContext(''); setNewTool(''); }); });
            }}
            className="bg-blue-600 text-white p-2 rounded w-fit"
            disabled={!newTaskContext || !newTool}
          >
            Record Pattern
          </button>
        </div>
      </div>

    </div>
  );
}
