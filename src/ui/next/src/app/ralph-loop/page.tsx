'use client';
import React, { useState } from 'react';

export default function RalphLoopPage() {
  const [task, setTask] = useState('');
  const [progressFile, setProgressFile] = useState('.ralph_progress.json');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);

  const handleExecute = async () => {
    setLoading(true);
    setError(null);
    setResult(null);

    try {
      const response = await fetch('/api/ralph-loop', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ task, progress_file: progressFile }),
      });

      if (!response.ok) {
        const err = await response.json();
        throw new Error(err.error || 'Failed to execute Ralph Loop');
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
    <div className="p-8 max-w-4xl mx-auto font-sans">
      <h1 className="text-3xl font-bold mb-4">The Ralph Loop (Long-Running Agent)</h1>
      <p className="text-gray-600 mb-8">
        Enter a complex task spanning multiple context windows. The Initializer Agent will break it down into features and make an initial git commit. Then, the Coding Agent will read the git log, pick the highest-priority incomplete feature, execute, and commit, repeating until the task is complete.
      </p>

      <div className="mb-6 space-y-4">
        <div>
          <label htmlFor="task" className="block text-sm font-medium text-gray-700 mb-2">
            Long-Running Task Description
          </label>
          <textarea
            id="task"
            className="w-full p-4 border rounded-lg shadow-sm focus:ring-blue-500 focus:border-blue-500"
            rows={4}
            value={task}
            onChange={(e) => setTask(e.target.value)}
            placeholder="e.g. Build a fully functional web server with authentication and logging."
          />
        </div>

        <div>
          <label htmlFor="progress_file" className="block text-sm font-medium text-gray-700 mb-2">
            Progress File Path (Optional)
          </label>
          <input
            id="progress_file"
            type="text"
            className="w-full p-2 border rounded-lg shadow-sm focus:ring-blue-500 focus:border-blue-500"
            value={progressFile}
            onChange={(e) => setProgressFile(e.target.value)}
            placeholder=".ralph_progress.json"
          />
        </div>
      </div>

      <button
        onClick={handleExecute}
        disabled={loading || !task}
        className="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 font-medium transition-colors"
      >
        {loading ? 'Ralph Loop Executing (Check terminal/git)...' : 'Start Ralph Loop'}
      </button>

      {error && (
        <div className="mt-8 p-4 bg-red-50 text-red-700 rounded-lg border border-red-200" data-testid="error-message">
          <h3 className="font-bold mb-2">Execution Error:</h3>
          <p>{error}</p>
        </div>
      )}

      {result && (
        <div className="mt-8 p-6 bg-white border rounded-lg shadow-sm" data-testid="success-message">
          <h2 className="text-xl font-bold mb-4 border-b pb-2">Loop Status</h2>
          <pre className="whitespace-pre-wrap text-sm text-gray-800 font-mono bg-gray-50 p-4 rounded">
            {JSON.stringify(result, null, 2)}
          </pre>
        </div>
      )}
    </div>
  );
}
