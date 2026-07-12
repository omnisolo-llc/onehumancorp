"use client";
import React, { useState } from 'react';
import { FaSearch, FaBrain, FaRegFileAlt } from 'react-icons/fa';

export default function CrossSessionRecall() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [summarize, setSummarize] = useState(false);

  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!query.trim()) return;

    setLoading(true);
    setError(null);
    setResults([]);

    try {
      const res = await fetch("/api/memory/cross-session", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          query,
          session_id: "global",
          limit: 10,
          summarize,
        }),
      });

      if (!res.ok) {
        throw new Error("Failed to search cross-session memory.");
      }

      const data = await res.json();
      if (data && data.results) {
        setResults(data.results);
      } else {
        setResults([]);
      }
    } catch (err: any) {
      console.error(err);
      setError(err.message || "An error occurred during search.");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex flex-col h-screen w-full bg-gray-50 dark:bg-gray-900 text-[#1D1D1F] dark:text-[#F5F5F7] p-8">
      <div className="max-w-4xl mx-auto w-full space-y-6">
        <header className="mb-8">
          <h1 className="text-3xl font-bold flex items-center gap-3">
            <FaBrain className="text-[#0066FF]" />
            Cross-Session Recall
          </h1>
          <p className="text-gray-600 dark:text-gray-400 mt-2">
            Search across all historical interactions and instantly synthesize customer context using Hermes FTS5 Agent mechanics.
          </p>
        </header>

        <form onSubmit={handleSearch} className="flex flex-col md:flex-row gap-4">
          <div className="relative flex-1">
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <FaSearch className="text-gray-400" />
            </div>
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search past conversations"
              className="block w-full pl-10 pr-3 py-4 border border-gray-300 rounded-xl leading-5 bg-white dark:bg-gray-800 dark:border-gray-700 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-[#0066FF] focus:border-transparent transition-all shadow-sm text-lg"
              autoFocus
            />
          </div>
          <div className="flex items-center gap-4">
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={summarize}
                onChange={(e) => setSummarize(e.target.checked)}
                className="w-5 h-5 text-[#0066FF] rounded border-gray-300 focus:ring-[#0066FF]"
              />
              <span className="text-sm font-medium">LLM Summarize</span>
            </label>
            <button
              type="submit"
              disabled={loading || !query.trim()}
              className="px-6 py-4 bg-[#0066FF] text-white font-medium rounded-xl hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors shadow-sm"
            >
              {loading ? "Searching..." : "Search Memory"}
            </button>
          </div>
        </form>

        {error && (
          <div className="p-4 bg-red-50 text-red-700 rounded-xl border border-red-200">
            {error}
          </div>
        )}

        {loading && (
          <div className="flex justify-center items-center py-20">
             <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-[#0066FF]"></div>
          </div>
        )}

        {!loading && !error && results.length === 0 && query && (
          <div className="text-center py-20 text-gray-500">
            <FaRegFileAlt className="mx-auto text-4xl mb-4 opacity-50" />
            <p className="text-lg">No memory found for this query.</p>
          </div>
        )}

        {!loading && results.length > 0 && (
          <div className="space-y-6 mt-8">
            <h2 className="text-xl font-semibold border-b border-gray-200 dark:border-gray-700 pb-2">Results</h2>
            {summarize ? (
              <div className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-6 shadow-sm rounded-xl">
                 <h3 className="text-sm font-bold text-[#0066FF] mb-3 uppercase tracking-wider">AI Synthesis</h3>
                 <div className="prose dark:prose-invert max-w-none text-gray-800 dark:text-gray-200 whitespace-pre-wrap">
                   {results[0]}
                 </div>
              </div>
            ) : (
              <div className="grid gap-4">
                {results.map((result, idx) => (
                  <div key={idx} className="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 p-5 shadow-sm rounded-xl hover:shadow-md transition-shadow">
                    <p className="text-gray-800 dark:text-gray-200 leading-relaxed font-mono text-sm whitespace-pre-wrap">
                      {result}
                    </p>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}