'use client';
import React, { useState } from 'react';
import { motion } from 'framer-motion';

export default function ExpertTeamPage() {
  const [task, setTask] = useState('');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleExecute = async () => {
    setLoading(true);
    setError(null);
    setResult(null);

    try {
      const response = await fetch('/api/expert-team', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ task }),
      });

      if (!response.ok) {
        const err = await response.json();
        throw new Error(err.error || 'Failed to execute expert team');
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
    <div className="min-h-screen bg-gradient-to-br from-indigo-50 via-white to-blue-50 p-8 font-sans flex flex-col items-center">
      <motion.div
        initial={{ opacity: 0, y: -20 }}
        animate={{ opacity: 1, y: 0 }}
        className="max-w-4xl w-full"
      >
        <div className="bg-white/70 backdrop-blur-xl border border-white/50 shadow-2xl rounded-3xl p-10">
          <div className="text-center mb-10">
            <h1 className="text-4xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-blue-700 to-indigo-600 mb-4 tracking-tight">
              Collaborative Expert Team
            </h1>
            <p className="text-gray-600 text-lg max-w-2xl mx-auto leading-relaxed">
              Enter a complex task. The Lead Agent will coordinate 5 domain experts (Industry Researcher, Financial Analyst, Strategic Analyst, Process Supervisor, Quality Auditor) to execute it in parallel, strictly passing through code-enforced quality gates.
            </p>
          </div>

          <div className="mb-8">
            <label className="block text-sm font-semibold text-gray-700 mb-3 ml-1">
              Business Task Context
            </label>
            <motion.textarea
              whileFocus={{ scale: 1.01 }}
              className="w-full p-5 bg-white/50 border border-indigo-100 rounded-2xl shadow-inner focus:ring-4 focus:ring-indigo-100 focus:border-indigo-400 transition-all resize-none text-gray-800 placeholder-gray-400"
              rows={6}
              value={task}
              onChange={(e) => setTask(e.target.value)}
              placeholder="e.g. Write a comprehensive business plan for a new vegan bakery... Chart: Required. Analysis: Deep."
            />
          </div>

          <div className="flex justify-center">
            <motion.button
              whileHover={{ scale: 1.02 }}
              whileTap={{ scale: 0.98 }}
              onClick={handleExecute}
              disabled={loading || !task}
              className="px-10 py-4 bg-gradient-to-r from-blue-600 to-indigo-600 text-white text-lg rounded-2xl shadow-lg hover:shadow-xl hover:from-blue-700 hover:to-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed font-semibold transition-all w-full md:w-auto"
            >
              {loading ? (
                <span className="flex items-center justify-center gap-3">
                  <svg className="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                  </svg>
                  Orchestrating Expert Team...
                </span>
              ) : 'Execute Task via Expert Team'}
            </motion.button>
          </div>

          {error && (
            <motion.div
              initial={{ opacity: 0, height: 0 }}
              animate={{ opacity: 1, height: 'auto' }}
              className="mt-8 p-5 bg-red-50/80 backdrop-blur-sm text-red-700 rounded-2xl border border-red-200 shadow-sm"
            >
              <h3 className="font-bold mb-2 flex items-center gap-2">
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd" /></svg>
                Quality Gate or Execution Error:
              </h3>
              <p className="ml-7">{error}</p>
            </motion.div>
          )}

          {result && (
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className="mt-10 bg-white/80 backdrop-blur-md border border-gray-100 rounded-3xl shadow-xl overflow-hidden"
            >
              <div className="bg-gradient-to-r from-gray-50 to-white px-8 py-5 border-b border-gray-100">
                <h2 className="text-xl font-bold text-gray-800 flex items-center gap-2">
                  <svg className="w-6 h-6 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                  Final Delivered Output
                </h2>
              </div>
              <div className="p-8">
                <pre className="whitespace-pre-wrap text-[15px] leading-relaxed text-gray-700 font-mono overflow-auto max-h-[600px] custom-scrollbar">
                  {result}
                </pre>
              </div>
            </motion.div>
          )}
        </div>
      </motion.div>
    </div>
  );
}
