'use client';
import React, { useState } from 'react';

export default function AiderRepoMapPage() {
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<string | null>(null);
  const [rootPath, setRootPath] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const fetchRepoMap = async () => {
    setLoading(true);
    setError(null);
    setResult(null);
    setRootPath(null);

    try {
      const response = await fetch('/api/v1/aider-repomap', {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
      });

      const data = await response.json();

      if (!response.ok) {
        throw new Error(data.error || 'Failed to fetch RepoMap');
      }

      setResult(data.map);
      setRootPath(data.root_path);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-8 max-w-6xl mx-auto font-sans min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800">
      <div className="backdrop-blur-xl bg-white/70 dark:bg-black/40 border border-white/20 dark:border-gray-700 shadow-xl rounded-3xl p-8">
        <h1 className="text-4xl font-bold mb-2 bg-clip-text text-transparent bg-gradient-to-r from-blue-600 to-indigo-600 dark:from-blue-400 dark:to-indigo-400">
          Aider: RepoMap Explorer
        </h1>
        <p className="text-gray-600 dark:text-gray-300 mb-8 font-medium">
          Aider Mechanic: Concise structural view of a codebase to provide context while keeping token count small.
        </p>

        <div className="flex items-center gap-4 mb-8">
          <button
            onClick={fetchRepoMap}
            disabled={loading}
            className="px-8 py-3 bg-blue-600 text-white font-semibold rounded-2xl shadow-lg hover:bg-blue-700 hover:shadow-xl transition-all disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? 'Scanning Repository...' : 'Generate RepoMap'}
          </button>
        </div>

        {error && (
          <div className="p-6 bg-red-500/10 border border-red-500/20 text-red-700 dark:text-red-400 rounded-2xl" data-testid="error-message">
            <div className="font-semibold text-lg mb-1">Error</div>
            {error}
          </div>
        )}

        {result && (
          <div className="animate-in fade-in slide-in-from-bottom-4 duration-500">
            <div className="mb-4 flex items-center justify-between">
              <div className="text-sm font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Root Directory: <span className="text-gray-900 dark:text-gray-100 font-mono normal-case">{rootPath}</span>
              </div>
              <div className="text-xs bg-indigo-100 dark:bg-indigo-900/50 text-indigo-700 dark:text-indigo-300 px-3 py-1 rounded-full font-medium">
                SOTA Harness Pattern
              </div>
            </div>

            <div className="p-6 bg-gray-900 shadow-inner rounded-2xl border border-gray-800 overflow-x-auto" data-testid="success-message">
              <pre className="whitespace-pre text-gray-100 font-mono text-sm leading-relaxed">
                {result}
              </pre>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}