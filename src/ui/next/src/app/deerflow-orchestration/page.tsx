'use client';
import { useState } from 'react';

export default function DeerFlowOrchestration() {
  const [task, setTask] = useState('');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setResult(null);
    setError(null);

    try {
      const res = await fetch('/api/v1/deerflow/run', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ task }),
      });

      const data = await res.json();
      if (!res.ok) {
        throw new Error(data.error || 'Something went wrong');
      }
      setResult(data.result);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-4xl mx-auto p-6">
      <h1 className="text-2xl font-bold mb-4">DeerFlow Sub-agent Orchestration</h1>
      <p className="text-gray-600 mb-6">
        Lead agent decomposes tasks, spawns parallel sub-agents, synthesizes results.
      </p>

      <form onSubmit={handleSubmit} className="mb-8">
        <textarea
          className="w-full p-4 border rounded-md min-h-[150px] mb-4 text-black"
          placeholder="e.g. Analyze the current AI market, compare the top 3 frameworks, and synthesize a recommendation report."
          value={task}
          onChange={(e) => setTask(e.target.value)}
          disabled={loading}
        />
        <button
          type="submit"
          className="bg-[#0071E3] text-white px-6 py-2 rounded-md hover:bg-blue-700 disabled:opacity-50"
          disabled={!task.trim() || loading}
        >
          {loading ? 'Orchestrating Sub-agents...' : 'Execute Task via DeerFlow'}
        </button>
      </form>

      {error && (
        <div className="p-4 bg-red-50 text-red-700 rounded-md mb-6">
          {error}
        </div>
      )}

      {result && (
        <div>
          <h2 className="text-xl font-semibold mb-4">Synthesized Result</h2>
          <div className="bg-gray-50 p-6 border rounded-md whitespace-pre-wrap text-black">
            {result}
          </div>
        </div>
      )}
    </div>
  );
}
