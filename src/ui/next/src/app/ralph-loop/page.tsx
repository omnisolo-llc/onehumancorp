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
      <h1 className="text-3xl font-bold mb-4 text-gray-900">The Ralph Loop (Long-Running Agent)</h1>
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
            className="w-full p-4 border border-gray-300 rounded-lg shadow-sm focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] transition-shadow text-gray-900 bg-white"
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
            className="w-full p-3 border border-gray-300 rounded-lg shadow-sm focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] transition-shadow text-gray-900 bg-white"
            value={progressFile}
            onChange={(e) => setProgressFile(e.target.value)}
            placeholder=".ralph_progress.json"
          />
        </div>
      </div>

      <button
        onClick={handleExecute}
        disabled={loading || !task.trim()}
        className="px-6 py-3 bg-[#0071E3] text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed font-medium transition-all shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[#0071E3]"
      >
        {loading ? 'Ralph Loop Executing (Check terminal/git)...' : 'Start Ralph Loop'}
      </button>

      {error && (
        <div className="mt-8 p-4 bg-red-50 text-red-700 rounded-lg border border-red-200 shadow-sm" data-testid="error-message">
          <h3 className="font-bold mb-2 flex items-center">
            <svg className="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd"></path></svg>
            Execution Error
          </h3>
          <p className="text-sm">{error}</p>
        </div>
      )}

      {result && (
        <div className="mt-8 p-6 bg-white border border-gray-200 rounded-lg shadow-sm" data-testid="success-message">
          <h2 className="text-xl font-bold mb-4 border-b border-gray-100 pb-2 text-gray-900">Loop Status</h2>
          <pre className="whitespace-pre-wrap text-sm text-gray-800 font-mono bg-gray-50 p-4 rounded-md border border-gray-100 overflow-x-auto">
            {JSON.stringify(result, null, 2)}
          </pre>
        </div>
      )}
    </div>
  );
}
